use std::{collections::BTreeMap, ffi::CString, path::Path, sync::LazyLock};

use convert_case::Casing as _;
use openfx_datagen::{
    parsing::{
        CPrimitiveType, DefineValue, FunctionParameter, RootItem, RootItemWithCommentAbove,
        TypeStraightforward, TypedIntegerLiteralCType, TypedefEnumCValueExpr, TypedefStructField,
    },
    processing::Bindings,
};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;

use crate::input_data::InputData;

pub fn generate_bindings(
    input_data: &InputData,
    output_folder: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    input_data
        .bindings
        .par_iter()
        .try_for_each(|(file_name, bindings)| {
            generate_bindings_one(input_data, file_name, bindings, output_folder)
        })?;

    Ok(())
}

fn generate_bindings_one(
    input_data: &InputData,
    file_name: &str,
    bindings: &Bindings,
    output_folder: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rs_file_name = {
        let file_stem = file_name
            .strip_suffix(".h")
            .ok_or_else(|| format!("Failed to strip suffix `.h` from file name: {}", file_name))?;
        if file_stem.chars().all(|c| c.is_alphabetic()) {
            let simple_file_stem = file_stem.strip_prefix("ofx").ok_or_else(|| {
                format!("Failed to strip prefix `ofx` from file name: {}", file_name)
            })?;
            simple_file_stem.to_case(convert_case::Case::Snake) + ".rs"
        } else {
            file_stem.to_owned() + ".rs"
        }
    };

    let mut output_text = String::new();
    let mut output = TokenStream::new();

    // extend_with_copyright_header(&mut output, bindings);
    output_text.push_str(&gen_copyright_header_text(bindings));
    output_text.push('\n');

    extend_with_use_statements(&mut output, bindings)?;

    let mut items = bindings.items.clone();

    let items_typedefs: Vec<_> = items.extract_if(.., |item| item.is_typedef()).collect();
    let items_status_defines: Vec<_> = items
        .extract_if(.., |item| {
            matches!(
                item,
                RootItemWithCommentAbove::Item {
                    item: RootItem::Define { name, .. },
                    ..
                } if name.starts_with("kOfxStat")
            )
        })
        .collect();

    // following `bindgen`'s convention: put `typedef …` at the end.
    items.extend(items_typedefs);
    // following our convention: put `#define kOfxStat…` at the end.
    items.extend(items_status_defines);

    for item in &items {
        let RootItemWithCommentAbove::Item {
            comment_above,
            item,
        } = item
        else {
            continue;
        };

        let name_str = item.name();
        let name = syn::Ident::new(name_str, proc_macro2::Span::call_site());

        match item {
            RootItem::Define {
                name: _,
                value: item,
                comment,
            } => {
                extend_with_doc(&mut output, comment_above.as_deref());
                extend_with_doc(&mut output, comment.as_deref());

                let (ty, value) = match item {
                    DefineValue::StringLiteral { value } => {
                        let ty = quote_define_value_type(item);
                        let c_str = CString::new(value.as_str())?;
                        (ty, quote! { #c_str })
                    }
                    DefineValue::IntegerLiteral { value } => {
                        let value =
                            syn::LitInt::new(&value.to_string(), proc_macro2::Span::call_site());
                        if name_str.starts_with("kOfxStat") {
                            (quote! { OfxStatus }, quote! { #value })
                        } else {
                            let ty = quote_define_value_type(item);
                            (ty, quote! { #value })
                        }
                    }
                    DefineValue::BooleanLiteral { value } => {
                        (quote_define_value_type(item), quote! { #value })
                    }
                    DefineValue::TypedIntegerLiteral { value, .. } => {
                        let value =
                            syn::LitInt::new(&value.to_string(), proc_macro2::Span::call_site());
                        if name_str.starts_with("kOfxStat") {
                            (quote! { OfxStatus }, quote! { #value })
                        } else {
                            (quote_define_value_type(item), quote! { #value })
                        }
                    }
                    DefineValue::Identifier { value } => {
                        let original_value = input_data.find_define_value(value).unwrap();
                        let ty = quote_define_value_type(original_value);

                        let ident = syn::Ident::new(value, proc_macro2::Span::call_site());
                        output.extend(quote! {
                            pub const #name: #ty = #ident;
                        });
                        continue;
                    }
                    DefineValue::WellKnownIdentifier { value } => {
                        output.extend(match value {
                            openfx_datagen::parsing::WellKnownIdentifier::INT_MAX => quote! {
                                pub const #name: ::std::os::raw::c_int = ::std::os::raw::c_int::MAX;
                            },
                            openfx_datagen::parsing::WellKnownIdentifier::INT_MIN => quote! {
                                pub const #name: ::std::os::raw::c_int = ::std::os::raw::c_int::MIN;
                            },
                        });
                        continue;
                    }
                };
                output.extend(quote! {
                    pub const #name: #ty = #value;
                });
            }
            RootItem::TypedefPrimitive { name: _, c_type } => {
                extend_with_doc(&mut output, comment_above.as_deref());

                let ty = quote_c_primitive_type(c_type);
                output.extend(quote! {
                    pub type #name = #ty;
                });
            }
            RootItem::TypedefOpaquePointer {
                name: _,
                pointee_struct_name,
            } => {
                let pointee_struct_name =
                    syn::Ident::new(pointee_struct_name, proc_macro2::Span::call_site());
                output.extend(quote! {
                    #[repr(C)]
                    #[derive(Debug, Copy, Clone)]
                    pub struct #pointee_struct_name {
                        _unused: [u8; 0],
                    }
                });

                extend_with_doc(&mut output, comment_above.as_deref());

                output.extend(quote! {
                    pub type #name = *mut #pointee_struct_name;
                });
            }
            RootItem::TypedefFunction {
                name: _,
                parameters,
                is_variadic,
                return_type,
            } => {
                extend_with_doc(&mut output, comment_above.as_deref());

                let (fn_ty, param_docs) =
                    quote_function_type(input_data, parameters, *is_variadic, return_type);

                extend_with_parameter_docs(&mut output, &param_docs);

                output.extend(quote! {
                    pub type #name = #fn_ty;
                });
            }
            RootItem::TypedefStruct { name: _, fields } => {
                extend_with_doc(&mut output, comment_above.as_deref());

                let mut body = TokenStream::new();

                for field in fields {
                    let TypedefStructField::Item {
                        comment_above,
                        item: field,
                    } = field
                    else {
                        continue;
                    };

                    extend_with_doc(&mut body, comment_above.as_deref());

                    let field_name = syn::Ident::new(&field.name, proc_macro2::Span::call_site());
                    let field_ty = match &field.r#type {
                        openfx_datagen::parsing::TypedefStructFieldType::Straightforward {
                            r#type,
                        } => quote_type_straightforward(input_data, r#type),
                        openfx_datagen::parsing::TypedefStructFieldType::FunctionPointer {
                            parameters,
                            is_variadic,
                            return_type,
                        } => {
                            let (fn_ty, param_docs) = quote_function_type(
                                input_data,
                                parameters,
                                *is_variadic,
                                return_type,
                            );

                            extend_with_parameter_docs(&mut body, &param_docs);

                            fn_ty
                        }
                    };

                    body.extend(quote! {
                        pub #field_name: #field_ty,
                    });
                }

                output.extend(quote! {
                    #[repr(C)]
                    #[derive(Debug, Copy, Clone)]
                    pub struct #name { #body }
                });
            }
            RootItem::TypedefEnum { name: _, variants } => {
                let mut body = TokenStream::new();
                for (i, variant) in variants.iter().enumerate() {
                    extend_with_doc(&mut body, variant.comment.as_deref());
                    let variant_name =
                        syn::Ident::new(&variant.name, proc_macro2::Span::call_site());
                    let value = match &variant.c_value_expr {
                        Some(c_expr) => quote_enum_c_value_expr(c_expr),
                        None => syn::LitInt::new(&i.to_string(), proc_macro2::Span::call_site())
                            .into_token_stream(),
                    };
                    body.extend(quote! {
                        pub const #variant_name: #name = #value;
                    });
                }

                extend_with_doc(&mut body, comment_above.as_deref());

                body.extend(quote! {
                    pub type #name = ::std::os::raw::c_uint;
                });

                output.extend(body);
            }
        }
    }

    output_text.push_str(&prettyplease::unparse(&syn::parse2(output)?));

    std::fs::write(output_folder.join(rs_file_name), output_text)?;
    Ok(())
}

// /// FIXME:
// ///
// /// ```text
// /// expected outer doc comment
// /// inner doc comments like this (starting with `//!` or `/*!`) can only appear before items
// /// ```
// fn extend_with_copyright_header(output: &mut TokenStream, bindings: &Bindings) {
//     if !bindings.copyright_comments.is_empty() {
//         let comment = bindings.copyright_comments.join("\n");
//         let backtick_count = safe_backtick_count(&comment);
//         let md_text = format!(
//             "{}copyright\n{}\n{}",
//             "`".repeat(backtick_count),
//             comment,
//             "`".repeat(backtick_count)
//         );
//         md_text.lines().for_each(|line| {
//             let line = format!(" {line}");
//             output.extend(quote! {
//                 #![doc = #line]
//             })
//         });
//     }
// }

fn gen_copyright_header_text(bindings: &Bindings) -> String {
    let text = bindings.copyright_comments.join("\n");
    text.lines()
        .map(|line| format!("// {line}"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn extend_with_doc(output: &mut TokenStream, doc: Option<&str>) {
    if let Some(comment_above) = doc {
        let backtick_count = safe_backtick_count(comment_above);
        let md_text = format!(
            "{}doxygen\n{}\n{}",
            "`".repeat(backtick_count),
            comment_above,
            "`".repeat(backtick_count)
        );
        md_text.lines().for_each(|line| {
            let line = format!(" {line}");
            output.extend(quote! {
                #[doc = #line]
            })
        });
    }
}

fn extend_with_parameter_docs(output: &mut TokenStream, param_docs: &BTreeMap<String, String>) {
    if param_docs.is_empty() {
        return;
    }

    let mut doc = "## Parameters\n".to_owned();
    for (param_name, param_doc) in param_docs {
        doc.push_str(&format!("### Parameter `{}`\n", param_name));
        let backtick_count = safe_backtick_count(param_doc);
        doc.push_str(&format!(
            "{}doxygen\n{}\n{}\n",
            "`".repeat(backtick_count),
            param_doc,
            "`".repeat(backtick_count)
        ));
    }
    for line in doc.trim().lines() {
        let line = format!(" {line}");
        output.extend(quote! {
            #[doc = #line]
        });
    }
}

fn extend_with_use_statements(
    output: &mut TokenStream,
    bindings: &Bindings,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut merged_map = bindings.used_types.clone();
    for (file_name, consts) in &bindings.used_consts {
        merged_map
            .entry(file_name.clone())
            .or_default()
            .extend(consts.iter().cloned());
    }

    for (file_name, idents) in &merged_map {
        let mod_name = file_name
            .strip_suffix(".h")
            .ok_or_else(|| format!("Failed to strip suffix `.h` from file name: {}", file_name))?
            .strip_prefix("ofx")
            .ok_or_else(|| format!("Failed to strip prefix `ofx` from file name: {}", file_name))?
            .to_case(convert_case::Case::Snake);
        let mod_name = syn::Ident::new(&mod_name, proc_macro2::Span::call_site());
        let idents = idents
            .iter()
            .map(|ident| syn::Ident::new(ident, proc_macro2::Span::call_site()));

        output.extend(quote! {
            use super::#mod_name::{#(#idents),*};
        });
    }

    Ok(())
}

fn quote_define_value_type(value: &DefineValue) -> TokenStream {
    match value {
        DefineValue::StringLiteral { .. } => quote! { &::std::ffi::CStr },
        DefineValue::IntegerLiteral { .. } => quote! { u32 },
        DefineValue::BooleanLiteral { .. } => quote! { bool },
        DefineValue::TypedIntegerLiteral { c_type, .. } => quote_integer_literal_c_type(c_type),
        DefineValue::Identifier { .. } => unreachable!(),
        DefineValue::WellKnownIdentifier { .. } => unreachable!(),
    }
}

fn quote_integer_literal_c_type(ty: &TypedIntegerLiteralCType) -> TokenStream {
    match ty.as_str() {
        "int" => quote! { ::std::os::raw::c_int },
        _ => todo!(),
    }
}

fn quote_c_primitive_type(ty: &CPrimitiveType) -> TokenStream {
    match ty.as_str() {
        "void" => quote! { ::std::os::raw::c_void },
        "bool" => quote! { bool },
        "char" => quote! { ::std::os::raw::c_char },
        "int" => quote! { ::std::os::raw::c_int },
        "float" => quote! { f32 },
        "double" => quote! { f64 },
        "size_t" => quote! { usize },
        "unsigned char" => quote! { ::std::os::raw::c_uchar },
        "unsigned short" => quote! { ::std::os::raw::c_ushort },
        "unsigned int" => quote! { ::std::os::raw::c_uint },
        _ => todo!(),
    }
}

fn quote_type_straightforward(input_data: &InputData, ty: &TypeStraightforward) -> TokenStream {
    match ty {
        TypeStraightforward::Ptr { pointee } | TypeStraightforward::ConstPtr { pointee } => {
            if let TypeStraightforward::TypeIdentifier { is } = &**pointee
                && matches!(
                    input_data.find_item(is.as_str()).unwrap(),
                    RootItem::TypedefFunction { .. }
                )
            {
                let ty = syn::Ident::new(is.as_str(), proc_macro2::Span::call_site());
                return quote! { #ty };
            }

            let inner = quote_type_straightforward(input_data, pointee);

            if matches!(ty, TypeStraightforward::Ptr { .. }) {
                quote! { *mut #inner }
            } else {
                quote! { *const #inner }
            }
        }
        TypeStraightforward::CPrimitive { is } => quote_c_primitive_type(is),
        TypeStraightforward::TypeIdentifier { is } => {
            let ty = syn::Ident::new(is.as_str(), proc_macro2::Span::call_site());
            quote! { #ty }
        }
    }
}

fn quote_function_type(
    input_data: &InputData,
    parameters: &[FunctionParameter],
    is_variadic: bool,
    return_type: &TypeStraightforward,
) -> (TokenStream, BTreeMap<String, String>) {
    let mut params: Vec<TokenStream> = Vec::new();
    let mut param_docs: BTreeMap<String, String> = BTreeMap::new();

    for param in parameters {
        let (param_tokens, param_doc) = quote_function_parameter(input_data, param);
        params.push(param_tokens);
        if let Some(doc) = param_doc {
            param_docs.insert(param.name.clone(), doc);
        }
    }
    if is_variadic {
        params.push(quote! { ... });
    }

    let ret_part = if return_type.is_void() {
        quote! {}
    } else {
        let ret_ty = quote_type_straightforward(input_data, return_type);
        quote! { -> #ret_ty }
    };

    (
        quote! {
            ::std::option::Option<unsafe extern "C" fn(#(#params),*) #ret_part>
        },
        param_docs,
    )
}

fn quote_function_parameter(
    input_data: &InputData,
    param: &FunctionParameter,
) -> (TokenStream, Option<String>) {
    let name = syn::Ident::new(param.name.as_str(), proc_macro2::Span::call_site());
    let ty = quote_type_straightforward(input_data, &param.r#type);

    (quote! { #name: #ty }, param.comment.clone())
}

fn quote_enum_c_value_expr(expr: &TypedefEnumCValueExpr) -> TokenStream {
    static RE_HEX_LITERAL: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r#"^0x([\da-fA-F]+)$"#).unwrap());
    static RE_PAREN_BITOR_IDENTS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r#"^\(\s*([_a-zA-Z][_a-zA-Z\d]*)\s*\|\s*([_a-zA-Z][_a-zA-Z\d]*)\s*\)*$"#)
            .unwrap()
    });

    if let Some(captures) = RE_HEX_LITERAL.captures(expr.as_str()) {
        let value = u32::from_str_radix(&captures[1], 16).unwrap();
        let value = syn::LitInt::new(&value.to_string(), proc_macro2::Span::call_site());
        return quote! { #value };
    }
    if let Some(captures) = RE_PAREN_BITOR_IDENTS.captures(expr.as_str()) {
        let left = syn::Ident::new(&captures[1], proc_macro2::Span::call_site());
        let right = syn::Ident::new(&captures[2], proc_macro2::Span::call_site());
        return quote! { (#left | #right) };
    }

    todo!()
}

fn safe_backtick_count(s: &str) -> usize {
    let mut max = 0;
    s.lines().for_each(|line| {
        let count = line.chars().take_while(|&c| c == '`').count();
        max = max.max(count);
    });
    (max + 1).max(3)
}
