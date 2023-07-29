use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{braced, parse::Parse, parse_macro_input, Expr, Generics, Token, Type};

#[derive(Debug)]
struct MatchArm {
    generics: Generics,
    match_type: Type,
    expr_type: Type,
    expr: Expr,
}

impl Parse for MatchArm {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let generics = input.parse()?;
        let match_type = input.parse()?;
        input.parse::<Token![=>]>()?;
        let expr_type = input.parse()?;
        input.parse::<Token![:]>()?;
        let expr = input.parse()?;
        Ok(Self {
            generics,
            match_type,
            expr_type,
            expr,
        })
    }
}

#[derive(Debug)]
struct Match {
    expr: Expr,
    match_arms: Vec<MatchArm>,
}

impl Parse for Match {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // input.parse::<Token![match]>()?;
        let expr = Expr::parse_without_eager_brace(input)?;

        let content;
        braced!(content in input);

        let match_arms = content
            .parse_terminated(MatchArm::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(Self { expr, match_arms })
    }
}

#[proc_macro]
pub fn match_type(input: pm::TokenStream) -> pm::TokenStream {
    let input = parse_macro_input!(input as Match);

    fn numbered_id(prefix: &str, i: usize) -> TokenStream {
        format!("{}{}", prefix, i).parse::<TokenStream>().unwrap()
    }

    let done = {
        let ids = (1..input.match_arms.len()).map(|i| numbered_id("__match_type_w", i));
        quote! {
            struct __MatchTypeDone<T>(T);

            impl<T> __MatchTypeDone<T> {
                #(fn #ids(self) -> Self {
                    self
                })*
                fn __match_type_done(self) -> T {
                    self.0
                }
            }
        }
    };

    let mut arms = quote! {};

    let mut add_arm =
        |i: usize, generics: &Generics, match_type: &Type, expr_type: &Type, expr: &Expr| {
            let match_i = numbered_id("__MatchTypeMatch", i);
            let m_i = numbered_id("__match_type_m", i);
            let wrapper_i = numbered_id("__MatchTypeWrapper", i);
            let w_i = numbered_id("__match_type_w", i);
            let catch_i = numbered_id("__MatchTypeCatch", i);
            let wrapper_i_plus_1 = numbered_id("__MatchTypeWrapper", i + 1);

            arms.extend(quote! {
                trait #match_i {
                    type __MatchTypeReturnType;
                    fn #m_i(self) -> Self::__MatchTypeReturnType;
                }

                impl #generics #match_i for #match_type {
                    type __MatchTypeReturnType = #expr_type;
                    fn #m_i(self) -> Self::__MatchTypeReturnType {
                        #expr
                    }
                }

                struct #wrapper_i<T>(T);

                impl<T: #match_i> #wrapper_i<T> {
                    fn #w_i(self) -> __MatchTypeDone<<T as #match_i>::__MatchTypeReturnType> {
                        __MatchTypeDone(#match_i::#m_i(self.0))
                    }
                }

                trait #catch_i {
                    type __MatchTypeReturnType;
                    fn #w_i(self) -> Self::__MatchTypeReturnType;
                }

                impl<T> #catch_i for #wrapper_i<T> {
                    type __MatchTypeReturnType = #wrapper_i_plus_1<T>;
                    fn #w_i(self) -> Self::__MatchTypeReturnType {
                        #wrapper_i_plus_1(self.0)
                    }
                }
            });
        };

    for (i, arm) in input.match_arms.iter().enumerate() {
        match arm.match_type {
            Type::Infer(_) => {
                let generics = quote! {<__MatchTypeT>}.into();
                let match_type = quote! {__MatchTypeT}.into();
                add_arm(
                    i,
                    &parse_macro_input!(generics as Generics),
                    &parse_macro_input!(match_type as Type),
                    &arm.expr_type,
                    &arm.expr,
                );
            }
            _ => {
                add_arm(i, &arm.generics, &arm.match_type, &arm.expr_type, &arm.expr);
            }
        }
    }

    let final_wrapper: TokenStream = {
        let id = numbered_id("__MatchTypeWrapper", input.match_arms.len());
        quote! {
            struct #id<T>(T);

            impl<T> #id<T> {
                fn __match_type_done(self) -> () {
                    ()
                }
            }
        }
    };

    let expr = input.expr;
    let funcs = (0..input.match_arms.len())
        .into_iter()
        .map(|i| numbered_id("__match_type_w", i));

    quote! {
        {
            #done
            #arms
            #final_wrapper

            __MatchTypeWrapper0(#expr)
            #(
                .#funcs()
            )*
            .__match_type_done()

        }
    }
    .into()
}
