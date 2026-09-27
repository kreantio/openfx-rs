//! Author: GitHub Copilot / Kimi K3 (High)

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocEntry {
    pub name: String,
    pub content: String,
}

pub struct DocRegulator<'a> {
    pub doc_entries: &'a HashMap<String, Vec<DocEntry>>,
}

impl DocRegulator<'_> {
    /// Regulates the doc attributes of a node named `name`:
    ///
    /// - existing `#[doc = "..."]` attributes are collected and removed;
    /// - if there were none, the doc content is taken from `doc_entries`
    ///   (only the first level is consulted, regardless of how deeply nested
    ///   the item is);
    /// - the resulting doc content is re-emitted, wrapped in a markdown code
    ///   fence tagged with the `doxygen` language mark. The fence length is
    ///   calculated from the longest backtick run inside the content
    ///   (minimum 3 backticks).
    pub fn regulate_docs(&self, attrs: &mut Vec<syn::Attribute>, name: &str) {
        let mut existing_docs: Vec<String> = vec![];
        attrs.retain(|attr| {
            if !attr.path().is_ident("doc") {
                return true;
            }
            if let syn::Meta::NameValue(name_value) = &attr.meta
                && let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit),
                    ..
                }) = &name_value.value
            {
                existing_docs.push(lit.value());
            }
            false
        });

        let content = if existing_docs.is_empty() {
            let Some(entries) = self.doc_entries.get(name) else {
                return;
            };
            entries
                .iter()
                .map(|e| e.content.clone())
                .collect::<Vec<_>>()
                .join("\n\n")
        } else {
            existing_docs.join("\n")
        };
        let content = content.trim();
        if content.is_empty() {
            return;
        }

        // The fence must be longer than any backtick run inside the content.
        let mut longest_backtick_run = 0usize;
        let mut current_run = 0usize;
        for ch in content.chars() {
            if ch == '`' {
                current_run += 1;
                longest_backtick_run = longest_backtick_run.max(current_run);
            } else {
                current_run = 0;
            }
        }
        let fence = "`".repeat((longest_backtick_run + 1).max(3));

        let mut doc_attrs: Vec<syn::Attribute> = Vec::with_capacity(content.lines().count() + 2);
        let opening = format!(" {fence}doxygen");
        doc_attrs.push(syn::parse_quote!(#[doc = #opening]));
        for line in content.lines() {
            let line = if line.is_empty() {
                String::new()
            } else {
                format!(" {line}")
            };
            doc_attrs.push(syn::parse_quote!(#[doc = #line]));
        }
        let closing = format!(" {fence}");
        doc_attrs.push(syn::parse_quote!(#[doc = #closing]));

        doc_attrs.append(attrs);
        *attrs = doc_attrs;
    }
}

/// Implements one `VisitMut` method per doc-able node kind.
///
/// Each method:
/// - regulates the node's doc attributes (wrapping existing docs and docs
///   from the first level of `doc_entries` in a fenced `doxygen` block);
/// - delegates to the default visitor so that nested nodes (fields, variants,
///   items inside modules, etc.) are visited as well.
macro_rules! impl_visit_mut_for_docable {
    ($($method:ident($ty:ty) => |$node:ident| $ident:expr;)*) => {
        $(
            fn $method(&mut self, $node: &mut $ty) {
                let ident: Option<&syn::Ident> = $ident;
                if let Some(ident) = ident {
                    self.regulate_docs(&mut $node.attrs, &ident.to_string());
                }
                syn::visit_mut::$method(self, $node);
            }
        )*
    };
}

impl syn::visit_mut::VisitMut for DocRegulator<'_> {
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
