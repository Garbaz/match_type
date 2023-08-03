//! This crate provides a single macro, `match_type`, which allows you to
//! "match" on the type of an expression at compile time.
//!
//! As it stands, it is not possible in Rust to reflect on the type of an
//! expression when defining a macro. However, there are situtations where we
//! would like a macro to expand differently depending on what the type of an
//! expression we are given is.
//!
//! Say for example we would like to write a macro `as_string` that turns the
//! given expression into _some_ string representation in some way. The type of
//! the expression might implement `Display`, or it might implement `Debug`, or
//! both, or neither. With `match_type` we can implement such a macro easily:
//!
//! ```rust
//! use match_type::match_type;
//! use std::fmt::{Debug, Display};
//!
//! macro_rules! as_string {
//!     ($e:expr) => {
//!         match_type!(
//!             $e {
//!                 <T: Display> T => String: format!("{}", self),
//!                 <T: Debug>   T => String: format!("{:?}", self),
//!                 _              => &'static str: stringify!($e),
//!             }
//!         )
//!     };
//! }
//! ```
//!
//! For more information, see the documentation of [`match_type`](macro.match_type.html), and take a look
//! at the [README.md](https://github.com/Garbaz/match_type#readme) on Github.

use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{braced, parse::Parse, parse_macro_input, Expr, Generics, Token, Type};

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

struct Match {
    expr: Expr,
    match_arms: Vec<MatchArm>,
}

impl Parse for Match {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
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

/// Allows you to match on the type of an expression.
/// # Syntax
///  
/// _ScrutineeExpr_ `{`\
/// &nbsp;&nbsp;&nbsp;&nbsp;(_Generics_? _MatchType_ `=>` _RhsType_ `:`
/// _RhsExpr_ `,`)*\
/// `}`
///
/// The type of the _ScrutineeExpr_ is what is matched on. The _Generics_ and
/// _MatchType_ define the match pattern of a match arm. The _RhsExpr_ is what
/// expression the whole `match_type` block expands to in case of a match, and
/// _RhsType_ is the type of _RhsExpr_.
///
/// To refer back to the value of the _ScrutineeExpr_ inside _RhsExpr_, use
/// `self`. Also unlike normal `match`, it is not possible to refer to, i.e.
/// capture, variables outside the context of the `match_type` block.
///
/// # Intended Usage
///
/// Generally this macro is only of much use inside the definition of another
/// macro, since in such a context it it not normally possible to reflect on the
/// types of expressions. For example:
///
/// ```rust
/// # use match_type::match_type;
/// # use std::fmt::{Debug, Display};
///
/// macro_rules! as_string {
///     ($e:expr) => {
///         match_type!(
///             $e {
///                 <T: Display> T => String: format!("{}", self),
///                 <T: Debug>   T => String: format!("{:?}", self),
///                              _ => String: stringify!($e).into(),
///             }
///         )
///     };
/// }
/// ```
///
/// Note that the type reflection happens during compile time and before any
/// surrounding monomorphization. I.e. if I were to use `as_string` like this:
///
/// ```rust
/// # use match_type::match_type;
/// # use std::fmt::{Debug, Display};
/// #
/// # macro_rules! as_string {
/// #     ($e:expr) => {
/// #         match_type!(
/// #             $e {
/// #                 <T: Display> T => String: format!("{}", self),
/// #                 <T: Debug>   T => String: format!("{:?}", self),
/// #                              _ => String: stringify!($e).into(),
/// #             }
/// #         )
/// #     };
/// # }
///
/// fn f<T>(x: T) -> String {
///     as_string!(x)
/// }
/// ```
///
/// , then this function will always return the string "x", even if the actual
/// type `T` for which we end up monomorphizing `f` does implement `Display` or
/// `Debug`.
///
/// # Differences with `match`
///
/// ## `_` not required
///
/// Contrary to the normal `match` expression in Rust, it is allowed with
/// `match_type` not to have a match-all pattern `_` at the end. In such a case,
/// if the expression being matched on has a type that matches none of the match
/// arms, an error will be thrown at compile time:
///
/// ```text
/// no method named `__match_type_arm_found` found [...]
/// ```
///
/// ## Arms with different types
///
/// Since a `match_type` block is not an expression until after expansion, it's
/// arms do not have to have the same type. For a somewhat contrived example:
///
/// ```rust
/// # use match_type::match_type;
///
/// macro_rules! opposite {
///     ($e:expr) => {
///         match_type!(
///             $e {
///                 <T: Neg> T => <T as Neg>::Output: -self,
///                 <T: Not> T => <T as Not>::Output: !self,
///             }
///         )
///     };
/// }
/// ```
///
/// # Performance & Implementation
///
/// See the [README.md](https://github.com/Garbaz/match_type#readme) on Github.
#[proc_macro]
pub fn match_type(input: pm::TokenStream) -> pm::TokenStream {
    let input = parse_macro_input!(input as Match);

    fn numbered_id(prefix: &str, i: usize) -> TokenStream {
        format!("{}{}", prefix, i).parse::<TokenStream>().unwrap()
    }

    let boilerplate = {
        let ids: Vec<_> = (0..input.match_arms.len())
            .map(|i| numbered_id("__match_type_arm", i))
            .collect();
        quote! {
            struct __MatchTypeDone<T>(T);

            impl<T> __MatchTypeDone<T> {
                #(
                    #[inline(always)]
                    fn #ids(self) -> Self {
                    self
                })*
                #[inline(always)]
                fn __match_type_arm_found(self) -> T {
                    self.0
                }
            }

            struct __MatchTypeWrapper<T>(T);

            trait __MatchTypeCatch<T> {
                #(
                    #[inline(always)]
                    fn #ids(self) -> Self;
                )*
            }

            impl<T> __MatchTypeCatch<T> for __MatchTypeWrapper<T> {
                #(
                    #[inline(always)]
                    fn #ids(self) -> Self {
                    self
                })*
            }
        }
    };

    let mut arms = quote! {};

    let mut add_arm =
        |i: usize, generics: &Generics, match_type: &Type, expr_type: &Type, expr: &Expr| {
            let match_i = numbered_id("__MatchTypeMatch", i);
            let arm_i = numbered_id("__match_type_arm", i);

            arms.extend(quote! {
                trait #match_i {
                    type __MatchTypeReturnType;
                    #[inline(always)]
                    fn __match_type_match(self) -> Self::__MatchTypeReturnType;
                }

                impl #generics #match_i for #match_type {
                    type __MatchTypeReturnType = #expr_type;
                    #[inline(always)]
                    fn __match_type_match(self) -> Self::__MatchTypeReturnType {
                        #expr
                    }
                }

                impl<T: #match_i> __MatchTypeWrapper<T> {
                    #[inline(always)]
                    fn #arm_i(self) -> __MatchTypeDone<<T as #match_i>::__MatchTypeReturnType> {
                        __MatchTypeDone(#match_i::__match_type_match(self.0))
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

    let expr = input.expr;
    let funcs = (0..input.match_arms.len())
        .into_iter()
        .map(|i| numbered_id("__match_type_arm", i));

    quote! {
        {
            #boilerplate
            #arms

            __MatchTypeWrapper(#expr)
            #(
                .#funcs()
            )*
            .__match_type_arm_found()
        }
    }
    .into()
}
