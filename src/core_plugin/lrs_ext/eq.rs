use ast::{self, MetaItem, Item, Expr};

use codemap::{Span};

use ext::base::{ExtCtxt};
use ext::deriving::generic::{
    TraitDef, MethodDef, Substructure, cs_fold, combine_substructure,
};
use ext::deriving::generic::ty::{
    LifetimeBounds, Literal, borrowed_self, borrowed_explicit_self,
};
use ext::build::{AstBuilder};

use ptr::{P};

use parse::token::{InternedString};

pub fn derive_eq(cx: &mut ExtCtxt, span: Span, mitem: &MetaItem, item: &Item,
                     push: &mut FnMut(P<Item>)) {
    // structures are equal if all fields are equal, and non equal, if
    // any fields are not equal or if the enum variants are different
    fn cs_eq(cx: &mut ExtCtxt, span: Span, substr: &Substructure) -> P<Expr> {
        cs_fold(
            true,  // use foldl
            |cx, span, subexpr, self_f, other_fs| {
                let other_f = match other_fs {
                    [ref o_f] => o_f,
                    _ => cx.span_bug(span, "not exactly 2 arguments in `derive(PartialEq)`")
                };

                let eq = cx.expr_binary(span, ast::BiEq, self_f, other_f.clone());

                cx.expr_binary(span, ast::BiAnd, subexpr, eq)
            },
            cx.expr_bool(span, true),
            Box::new(|cx, span, _, _| cx.expr_bool(span, false)),
            cx, span, substr)
    }
    fn cs_ne(cx: &mut ExtCtxt, span: Span, substr: &Substructure) -> P<Expr> {
        cs_fold(
            true,  // use foldl
            |cx, span, subexpr, self_f, other_fs| {
                let other_f = match other_fs {
                    [ref o_f] => o_f,
                    _ => cx.span_bug(span, "not exactly 2 arguments in `derive(PartialEq)`")
                };

                let eq = cx.expr_binary(span, ast::BiNe, self_f, other_f.clone());

                cx.expr_binary(span, ast::BiOr, subexpr, eq)
            },
            cx.expr_bool(span, false),
            Box::new(|cx, span, _, _| cx.expr_bool(span, true)),
            cx, span, substr)
    }

    macro_rules! md {
        ($name:expr, $f:ident) => { {
            let inline = cx.meta_word(span, InternedString::new("inline"));
            let attrs = vec!(cx.attribute(span, inline));
            MethodDef {
                name: $name,
                generics: LifetimeBounds::empty(),
                explicit_self: borrowed_explicit_self(),
                args: vec!(borrowed_self()),
                ret_ty: Literal(path_local!(bool)),
                attributes: attrs,
                combine_substructure: combine_substructure(Box::new(|a, b, c| {
                    $f(a, b, c)
                }))
            }
        } }
    }

    let trait_def = TraitDef {
        span: span,
        attributes: Vec::new(),
        path: path_std!(cx, linux::ops::Eq),
        additional_bounds: Vec::new(),
        generics: LifetimeBounds::empty(),
        methods: vec!(
            md!("eq", cs_eq),
            md!("ne", cs_ne)
        ),
        associated_types: Vec::new(),
    };
    trait_def.expand(cx, mitem, item, push)
}

