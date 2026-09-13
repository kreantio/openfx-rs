use convert_case::Casing as _;
use proc_macro::TokenStream;

pub fn impl_suite(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = syn::parse_macro_input!(item as syn::ItemImpl);

    let mut errors: Vec<syn::Error> = Vec::new();

    for item in &mut input.items {
        let syn::ImplItem::Fn(item) = item else {
            continue;
        };

        if consume_no_sys_fn_flag(item) {
            continue;
        }

        let mut rewriter = Rewriter {
            ident: item.sig.ident.clone(),
            called_times: 0,
            errors: Vec::new(),
        };
        syn::visit_mut::visit_impl_item_fn_mut(&mut rewriter, item);

        if rewriter.called_times != 1 {
            errors.push(syn::Error::new_spanned(
                &item.sig.ident,
                format!("Expected sys_fn!() to be called exactly once in {}, but it was called {} times.", item.sig.ident, rewriter.called_times),
            ));
        }
        errors.extend(rewriter.errors);
    }

    let errors_tokens = errors.iter().map(syn::Error::to_compile_error);

    quote::quote! {
        #input
        #(#errors_tokens)*
    }
    .into()
}

fn consume_no_sys_fn_flag(item: &mut syn::ImplItemFn) -> bool {
    let old_len = item.attrs.len();
    item.attrs.retain(|attr| !attr.path().is_ident("no_sys_fn"));
    old_len != item.attrs.len()
}

struct Rewriter {
    ident: syn::Ident,

    called_times: usize,
    errors: Vec<syn::Error>,
}

impl syn::visit_mut::VisitMut for Rewriter {
    fn visit_expr_mut(&mut self, i: &mut syn::Expr) {
        let syn::Expr::Macro(mac) = i else {
            return syn::visit_mut::visit_expr_mut(self, i);
        };
        if !mac.mac.path.is_ident("sys_fn") {
            return syn::visit_mut::visit_expr_mut(self, i);
        }

        let style: SysFnMacroStyle = match syn::parse2(mac.mac.tokens.clone()) {
            Ok(parsed) => parsed,
            Err(err) => {
                self.errors.push(err);
                return;
            }
        };

        let name = self.ident.to_string();
        let final_name = match style {
            SysFnMacroStyle::Camel => name.to_case(convert_case::Case::Camel),
            SysFnMacroStyle::Pascal => name.to_case(convert_case::Case::Pascal),
            SysFnMacroStyle::Custom(custom) => custom,
        };
        let ident = syn::Ident::new(&final_name, self.ident.span());

        let replacement: syn::Expr = syn::parse_quote! {
            self.sys_ref()
                .#ident
                .unwrap_unchecked()
        };
        *i = replacement;
        self.called_times += 1;
    }
}

enum SysFnMacroStyle {
    Camel,
    Pascal,
    Custom(String),
}

impl syn::parse::Parse for SysFnMacroStyle {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(SysFnMacroStyle::Camel);
        }
        if input.peek(syn::Ident) {
            let ident: syn::Ident = input.parse()?;
            if ident == "pascal" {
                Ok(SysFnMacroStyle::Pascal)
            } else {
                Err(input.error("expected `pascal` or a string literal"))
            }
        } else if input.peek(syn::LitStr) {
            let lit: syn::LitStr = input.parse()?;
            Ok(SysFnMacroStyle::Custom(lit.value()))
        } else {
            Err(input.error("expected `pascal` or a string literal"))
        }
    }
}
