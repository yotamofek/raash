#![feature(box_patterns, debug_closure_helpers)]

use std::{
    fmt::{Display, FormatterFn},
    ops::RangeInclusive,
};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    meta,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, Expr, ExprLit, ExprRange, Item, ItemEnum, ItemFn, ItemStruct,
    ItemType, ItemUnion, Lit, LitInt, LitStr, RangeLimits,
};

struct LinesRange(RangeInclusive<u16>);

impl Parse for LinesRange {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let range: ExprRange = input.parse()?;
        let ExprRange {
            start:
                Some(box Expr::Lit(ExprLit {
                    lit: Lit::Int(start_line),
                    ..
                })),
            limits: RangeLimits::Closed(..),
            end:
                Some(box Expr::Lit(ExprLit {
                    lit: Lit::Int(end_line),
                    ..
                })),
            ..
        } = range
        else {
            return Err(input.error("invalid \"lines\" property"));
        };

        Ok(Self(start_line.base10_parse()?..=end_line.base10_parse()?))
    }
}

struct SingleLine(u16);

impl Parse for SingleLine {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse::<LitInt>()?.base10_parse()?))
    }
}

#[derive(parse_variants::Parse)]
enum LinesSpec {
    Range(LinesRange),
    SingleLine(SingleLine),
}

impl LinesSpec {
    fn as_url_frag(&self) -> impl Display + '_ {
        FormatterFn(move |f| match self {
            LinesSpec::Range(LinesRange(range)) => write!(f, "L{}-L{}", range.start(), range.end()),
            LinesSpec::SingleLine(SingleLine(line)) => write!(f, "L{line}"),
        })
    }
}

#[proc_macro_attribute]
pub fn ffmpeg_src(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut file: Option<LitStr> = None;
    let mut lines: Option<LinesSpec> = None;
    let mut orig_name: Option<String> = None;

    let attr_parser = meta::parser(|meta| {
        if meta.path.is_ident("file") {
            file = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("lines") {
            lines = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("name") {
            orig_name = Some(meta.value()?.parse::<LitStr>()?.value());
        } else {
            return Err(meta.error("unsupported ffmpeg src property"));
        }

        Ok(())
    });

    parse_macro_input!(attr with attr_parser);

    let file = file.expect("missing \"file\" property").value();
    let lines = lines.expect("missing \"lines\" property");

    let ffmpeg_rev = "2d9ed64859c9887d0504cd71dbd5b2c15e14251a";
    let lines_frag = lines.as_url_frag();
    let url = format!("https://github.com/FFmpeg/FFmpeg/blob/{ffmpeg_rev}/{file}#{lines_frag}");

    let alias_attr = orig_name.as_ref().map(|name| quote!(#[doc(alias = #name)]));

    let orig_name_doc = orig_name
        .map(|name| format!("(`{name}`)"))
        .unwrap_or_default();
    let doc = format!("Ffmpeg source: [{file}]({url}) {orig_name_doc}");

    let mut item = parse_macro_input!(item as Item);
    let attrs = match &mut item {
        Item::Enum(ItemEnum { attrs, .. })
        | Item::Fn(ItemFn { attrs, .. })
        | Item::Struct(ItemStruct { attrs, .. })
        | Item::Type(ItemType { attrs, .. })
        | Item::Union(ItemUnion { attrs, .. }) => attrs,
        _ => panic!("Proc macro used on invalid item type"),
    };
    attrs.push(parse_quote! {
        #[doc = ""]
    });
    attrs.push(parse_quote! {
        #[doc = #doc]
    });

    quote! {
        #alias_attr
        #item
    }
    .into()
}
