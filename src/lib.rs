use proc_macro as pm;
use quote::{quote, quote_spanned};
use syn::{braced, parse::Parse, parse_macro_input, spanned::Spanned, Expr, Generics, Token, Type};

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
    // println!("{:#?}", input);
    let boilerplate = quote! {
        trait __MatchType {
            type ExprType;
            fn __match_type_arm(self) -> Self::ExprType;
        }

        struct __MatchTypeWrapper<T>(T);

        impl<T: __MatchType> __MatchTypeWrapper<T> {
            fn __match_type_arm(self) -> <T as __MatchType>::ExprType {
                self.0.__match_type_arm()
            }
        }
    };

    let mut arms = quote! {};

    for arm in &input.match_arms {
        match arm.match_type {
            Type::Infer(_) => {
                let expr_type = &arm.expr_type;
                let expr = &arm.expr;
                arms.extend(quote_spanned! { arm.match_type.span() =>
                    trait __MatchTypeDefault {
                        fn __match_type_arm(self) -> #expr_type;
                    }

                    impl<T> __MatchTypeDefault for T {
                        fn __match_type_arm(self) -> #expr_type {
                            #expr
                        }
                    }
                });
            }
            _ => {
                let generics = &arm.generics;
                let match_type = &arm.match_type;
                let expr_type = &arm.expr_type;
                let expr = &arm.expr;
                arms.extend(quote! {
                    impl #generics __MatchType for #match_type {
                        type ExprType = #expr_type;
                        fn __match_type_arm(self) -> Self::ExprType {
                            #expr
                        }
                    }
                });
            }
        }
    }

    let expr = input.expr;

    quote! {
        {
            #boilerplate
            #arms

            __MatchTypeWrapper(#expr).__match_type_arm()
        }
    }
    .into()
}
