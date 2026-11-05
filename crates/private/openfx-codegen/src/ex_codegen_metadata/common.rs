use std::collections::{BTreeMap, HashSet};

use quote::quote;

use openfx_datagen::metadata_extracting::{
    PropdefDimension, PropdefType, PropdefTypeSimple, PropsetMetadataEntry, WriteSide,
};

use crate::{
    input_metadata::InputMetadata,
    utils::{new_ident, to_snake_case},
};

pub fn process_property_set(
    metadata: &InputMetadata,
    set_name: &str,
    set: &PropsetMetadataEntry,
    refs: &[syn::Ident],
    extra_meta_args: &[proc_macro2::TokenStream],
) -> Result<proc_macro2::TokenStream, Box<dyn std::error::Error>> {
    let mut items: BTreeMap<&str, proc_macro2::TokenStream> = BTreeMap::new();

    for (prop_cname, prop) in &set.props {
        let write = prop.write.as_ref();
        add_property_set_item(&mut items, metadata, prop_cname, write, set_name)?;
    }

    let mut meta_items: Vec<proc_macro2::TokenStream> = extra_meta_args.to_vec();

    meta_items.push(match set.write {
        WriteSide::All => quote! { write = all },
        WriteSide::Host => quote! { write = host },
        WriteSide::Plugin => quote! { write = plugin },
    });

    if !set.props_refs.is_empty() {
        meta_items.push(quote! { refs = #(#refs)+* });
    }

    let parenthesized_meta_items = if meta_items.is_empty() {
        quote! {}
    } else {
        quote! { (#(#meta_items),*) }
    };

    let struct_name_ident = new_ident(format!("{set_name}PropertySet"));

    let items: Vec<_> = items.values().cloned().collect();

    Ok(quote! {
        #[propset #parenthesized_meta_items]
        struct #struct_name_ident { #(#items)* }
    })
}

pub fn process_property_set_definitions(
    metadata: &InputMetadata,
    used_refs: &HashSet<String>,
) -> Result<Option<proc_macro2::TokenStream>, Box<dyn std::error::Error>> {
    let mut output_def_inner = proc_macro2::TokenStream::new();

    for (def_name, def) in &metadata.raw.propsetdef_map {
        if !used_refs.contains(def_name) {
            continue;
        }

        let mut items: BTreeMap<&str, proc_macro2::TokenStream> = BTreeMap::new();

        for (prop_cname, prop) in &def.props {
            let write = prop.write.as_ref();
            add_property_set_item(&mut items, metadata, prop_cname, write, def_name)?;
        }

        let def_name_ident = new_ident(def_name);

        let items: Vec<_> = items.values().cloned().collect();

        output_def_inner.extend(quote! {
            #[propsetdef]
            struct #def_name_ident { #(#items)* }
        });
    }

    if output_def_inner.is_empty() {
        Ok(None)
    } else {
        Ok(Some(quote! {
            #[propsetdefs]
            mod __ { #output_def_inner }
        }))
    }
}

pub fn add_property_set_item<'a>(
    items: &mut BTreeMap<&'a str, proc_macro2::TokenStream>,
    metadata: &InputMetadata,
    prop_cname: &'a str,
    write: Option<&WriteSide>,
    set_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut prop_names = PropertyNames::new(prop_cname);

    let Some(def) = metadata.raw.propdef_map.get(prop_cname) else {
        return Err(format!("Missing propdef: {prop_cname}").into());
    };

    let ty = match &def.r#type {
        PropdefType::Simple { one_of } => {
            let mut tys: Vec<proc_macro2::TokenStream> = vec![];
            for ty in one_of {
                let ty_tokens = match ty {
                    PropdefTypeSimple::Int => quote! { Int },
                    PropdefTypeSimple::Double => quote! { Double },
                    PropdefTypeSimple::Bool => quote! { Bool },
                    PropdefTypeSimple::String => quote! { String },
                    PropdefTypeSimple::Pointer => quote! { Pointer },
                };
                tys.push(ty_tokens);
            }

            if one_of.len() == 1 {
                tys.pop().unwrap()
            } else {
                quote! { ty_one_of!(#(#tys)|*) }
            }
        }
        PropdefType::StringEnum { .. } => {
            let simple_name_ident = new_ident(prop_names.simple_name()?);
            quote! { ty_enum!(#simple_name_ident) }
        }
    };

    let ty = match &def.dimension {
        PropdefDimension::Fixed { size } if size.get() == 1 => ty,
        PropdefDimension::Fixed { size } => {
            let size = syn::LitInt::new(&size.to_string(), proc_macro2::Span::call_site());
            quote! { [#ty; # size] }
        }
        PropdefDimension::Dynamic => {
            quote! { [#ty] }
        }
    };

    let mut meta_items: Vec<proc_macro2::TokenStream> = vec![];

    if let Some(write) = write {
        meta_items.push(match write {
            WriteSide::All => quote! { write = all },
            WriteSide::Host => quote! { write = host },
            WriteSide::Plugin => quote! { write = plugin },
        });
    }

    let prop_canonical_name_ident = new_ident(prop_names.canonical_name()?);
    let prop_snake_name_ident = match prop_names.snake_name()?.as_str() {
        "type" => syn::Ident::new_raw("type", proc_macro2::Span::call_site()),
        prop_snake_name => new_ident(prop_snake_name),
    };

    meta_items.push(quote! { @#prop_canonical_name_ident });

    let parenthesized_meta_items = if meta_items.is_empty() {
        quote! {}
    } else {
        quote! { (#(#meta_items),*) }
    };

    let item = quote! {
        #[prop #parenthesized_meta_items]
        #prop_snake_name_ident: #ty,
    };
    match items.entry(prop_cname) {
        std::collections::btree_map::Entry::Occupied(occupied_entry) => {
            assert_eq!(occupied_entry.get().to_string(), item.to_string());
            tracing::warn!("Duplicate entry `{prop_cname}` on `{set_name}`.");
        }
        std::collections::btree_map::Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(item);
        }
    };

    Ok(())
}

pub struct PropertyNames<'a> {
    cname: &'a str,
    canonical_name: Option<&'a str>,
    simple_name: Option<&'a str>,
    super_simple_name: Option<String>,
    snake_name: Option<String>,
}
impl<'a> PropertyNames<'a> {
    pub fn new(cname: &'a str) -> Self {
        Self {
            cname,
            canonical_name: None,
            simple_name: None,
            super_simple_name: None,
            snake_name: None,
        }
    }

    pub fn canonical_name(&mut self) -> Result<&'a str, Box<dyn std::error::Error>> {
        if self.canonical_name.is_none() {
            let Some(canonical_name) = self.cname.strip_prefix("k") else {
                return Err(format!("Irregular property cname: {}", self.cname).into());
            };
            self.canonical_name = Some(canonical_name);
        }
        Ok(self.canonical_name.unwrap())
    }
    pub fn simple_name(&mut self) -> Result<&'a str, Box<dyn std::error::Error>> {
        if self.simple_name.is_none() {
            let canonical_name = self.canonical_name()?;
            let Some(simple_name) = canonical_name.strip_prefix("Ofx") else {
                return Err(
                    format!("Irregular property canonical name: {}", canonical_name).into(),
                );
            };
            self.simple_name = Some(simple_name);
        }
        Ok(self.simple_name.unwrap())
    }
    pub fn super_simple_name(&mut self) -> Result<&str, Box<dyn std::error::Error>> {
        if self.super_simple_name.is_none() {
            let simple_name = self.simple_name()?;
            let super_simple_name =
                Self::get_property_super_simple_name_from_simple_name(simple_name);
            self.super_simple_name = Some(super_simple_name);
        }
        Ok(self.super_simple_name.as_ref().unwrap())
    }
    pub fn snake_name(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        if self.snake_name.is_none() {
            let super_simple_name = self.super_simple_name()?;
            let snake_name = to_snake_case(super_simple_name);
            self.snake_name = Some(snake_name);
        }
        Ok(self.snake_name.as_ref().unwrap().clone())
    }

    fn get_property_super_simple_name_from_simple_name(simple_name: &str) -> String {
        static RE: std::sync::LazyLock<regex::Regex> =
            std::sync::LazyLock::new(|| regex::Regex::new(r"Prop([A-Z])").unwrap());

        if !RE.is_match(simple_name) {
            tracing::warn!("Property simple name without the `Prop` part: {simple_name}");
            return simple_name.to_string();
        }

        RE.replace(simple_name, "$1").to_string()
    }
}
