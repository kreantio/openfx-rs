//! Author: GitHub Copilot / Kimi K3 (High)

use crate::doc_parsing::DocEntry;
use std::collections::HashMap;

pub struct MissingDocsAdder<'a> {
    pub doc_entries: &'a HashMap<String, Vec<DocEntry>>,
}

impl MissingDocsAdder<'_> {
    /// Adds a `#[doc = "..."]` attribute to `attrs` if `attrs` does not
    /// already have a doc attribute and `name` has a doc entry.
    ///
    /// Only the first level of `doc_entries` is consulted, regardless of how
    /// deeply nested the item is.
    pub fn add_doc_if_missing(&self, attrs: &mut Vec<syn::Attribute>, name: &str) {
        if attrs.iter().any(|attr| attr.path().is_ident("doc")) {
            return;
        }
        let Some(entry) = self.doc_entries.get(name) else {
            return;
        };
        let content = entry
            .iter()
            .map(|e| e.content.to_string())
            .collect::<Vec<_>>()
            .join("\n\n");
        let content = content.trim();
        if content.is_empty() {
            return;
        }
        let content = format!(" {}", content);
        attrs.push(syn::parse_quote!(#[doc = #content]));
    }
}

/// Implements one `VisitMut` method per doc-able node kind.
///
/// Each method:
/// - looks up the node's own name in the first level of `doc_entries` and adds
///   a doc attribute if the node doesn't have one yet;
/// - delegates to the default visitor so that nested nodes (fields, variants,
///   items inside modules, etc.) are visited as well.
macro_rules! impl_visit_mut_for_docable {
    ($($method:ident($ty:ty) => |$node:ident| $ident:expr;)*) => {
        $(
            fn $method(&mut self, $node: &mut $ty) {
                let ident: Option<&syn::Ident> = $ident;
                if let Some(ident) = ident {
                    self.add_doc_if_missing(&mut $node.attrs, &ident.to_string());
                }
                syn::visit_mut::$method(self, $node);
            }
        )*
    };
}

impl syn::visit_mut::VisitMut for MissingDocsAdder<'_> {
    impl_visit_mut_for_docable! {
        // Top-level items.
        visit_item_const_mut(syn::ItemConst) => |node| Some(&node.ident);
        visit_item_enum_mut(syn::ItemEnum) => |node| Some(&node.ident);
        visit_item_extern_crate_mut(syn::ItemExternCrate) => |node| Some(&node.ident);
        visit_item_fn_mut(syn::ItemFn) => |node| Some(&node.sig.ident);
        visit_item_macro_mut(syn::ItemMacro) => |node| node.ident.as_ref();
        visit_item_mod_mut(syn::ItemMod) => |node| Some(&node.ident);
        visit_item_static_mut(syn::ItemStatic) => |node| Some(&node.ident);
        visit_item_struct_mut(syn::ItemStruct) => |node| Some(&node.ident);
        visit_item_trait_mut(syn::ItemTrait) => |node| Some(&node.ident);
        visit_item_trait_alias_mut(syn::ItemTraitAlias) => |node| Some(&node.ident);
        visit_item_type_mut(syn::ItemType) => |node| Some(&node.ident);
        visit_item_union_mut(syn::ItemUnion) => |node| Some(&node.ident);

        // Struct/union fields and enum variants.
        visit_field_mut(syn::Field) => |node| node.ident.as_ref();
        visit_variant_mut(syn::Variant) => |node| Some(&node.ident);

        // Foreign (extern) items.
        visit_foreign_item_fn_mut(syn::ForeignItemFn) => |node| Some(&node.sig.ident);
        visit_foreign_item_macro_mut(syn::ForeignItemMacro) =>
            |node| node.mac.path.segments.last().map(|seg| &seg.ident);
        visit_foreign_item_static_mut(syn::ForeignItemStatic) => |node| Some(&node.ident);
        visit_foreign_item_type_mut(syn::ForeignItemType) => |node| Some(&node.ident);

        // Trait items.
        visit_trait_item_const_mut(syn::TraitItemConst) => |node| Some(&node.ident);
        visit_trait_item_fn_mut(syn::TraitItemFn) => |node| Some(&node.sig.ident);
        visit_trait_item_macro_mut(syn::TraitItemMacro) =>
            |node| node.mac.path.segments.last().map(|seg| &seg.ident);
        visit_trait_item_type_mut(syn::TraitItemType) => |node| Some(&node.ident);

        // Impl items.
        visit_impl_item_const_mut(syn::ImplItemConst) => |node| Some(&node.ident);
        visit_impl_item_fn_mut(syn::ImplItemFn) => |node| Some(&node.sig.ident);
        visit_impl_item_macro_mut(syn::ImplItemMacro) =>
            |node| node.mac.path.segments.last().map(|seg| &seg.ident);
        visit_impl_item_type_mut(syn::ImplItemType) => |node| Some(&node.ident);
    }
}
