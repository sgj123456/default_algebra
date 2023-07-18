use proc_macro::*;
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
struct Variant {
    ident: TokenTree,
    value_type: Option<Ident>,
}
#[derive(Clone, Debug)]
struct Enum {
    ident: TokenTree,
    defaults: BTreeMap<String, (TokenTree, TokenTree)>,
    variants: Vec<Variant>,
    annotate: Vec<TokenTree>,
}
#[proc_macro_attribute]
pub fn default_algebra(attribute: TokenStream, input: TokenStream) -> TokenStream {
    let enum_data = get(attribute.clone(), input.clone());
    TokenStream::from_iter(
        set(enum_data.clone())
            .into_iter()
            .chain(add(enum_data.clone()).into_iter()),
    )
}
fn get(attribute: TokenStream, input: TokenStream) -> Enum {
    let mut attribute_iter = attribute.into_iter();
    let mut defaults = BTreeMap::new();
    loop {
        let (value_type, _, value) = (
            attribute_iter.next().unwrap(),
            attribute_iter.next().unwrap(),
            attribute_iter.next().unwrap(),
        );
        defaults.insert(value_type.to_string(), (value_type, value));
        if let None = attribute_iter.next() {
            break;
        }
    }
    let mut input_iter = input.into_iter();
    let mut annotate = Vec::new();
    loop {
        let temp = input_iter.next().unwrap();
        if let TokenTree::Ident(_) = temp {
            break;
        }
        annotate.push(temp)
    }
    let ident = input_iter.next().unwrap();
    let variants = if let TokenTree::Group(value) = input_iter.next().unwrap() {
        value.stream().into_iter().fold(
            Vec::<Variant>::new(),
            |mut state_variants, node| match node {
                TokenTree::Group(group) => {
                    group.stream().into_iter().for_each(|node| {
                        match node {
                            TokenTree::Ident(ident) => {
                                state_variants.last_mut().unwrap().value_type = Some(ident)
                            }
                            _ => panic!(),
                        };
                    });
                    state_variants
                }
                TokenTree::Punct(_) => state_variants,
                ident @ TokenTree::Ident(_) => {
                    state_variants.push(Variant {
                        ident,
                        value_type: None,
                    });
                    state_variants
                }
                _ => panic!(),
            },
        )
    } else {
        panic!()
    };
    Enum {
        ident,
        defaults,
        variants,
        annotate,
    }
}
fn set(enum_data: Enum) -> Vec<TokenTree> {
    // println!("{:?}", enum_data.clone());
    let mut result = enum_data.annotate.clone();
    result.extend([
        TokenTree::Ident(Ident::new("enum", Span::call_site())),
        enum_data.ident,
        TokenTree::Group(Group::new(
            Delimiter::Brace,
            enum_data
                .variants
                .into_iter()
                .flat_map(|node| {
                    match (
                        node.value_type,
                        enum_data.defaults.first_key_value().unwrap().1 .0.clone(),
                    ) {
                        (None, value) => [
                            node.ident,
                            TokenTree::Group(Group::new(Delimiter::Parenthesis, value.into())),
                            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        ]
                        .to_vec(),

                        (Some(value), _) if value.to_string() != "Nil" => [
                            node.ident,
                            TokenTree::Group(Group::new(
                                Delimiter::Parenthesis,
                                TokenTree::Ident(value).into(),
                            )),
                            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        ]
                        .to_vec(),
                        _ => [
                            node.ident,
                            TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                        ]
                        .to_vec(),
                    }
                })
                .collect(),
        )),
    ]);
    result
}
fn add(enum_data: Enum) -> Vec<TokenTree> {
    Vec::from([
        TokenTree::Ident(Ident::new("impl", Span::call_site())),
        enum_data.ident,
        TokenTree::Group(Group::new(
            proc_macro::Delimiter::Brace,
            TokenStream::from_iter(
                enum_data
                    .variants
                    .into_iter()
                    .filter(|node| match node.value_type.clone() {
                        None => true,
                        Some(value) if value.to_string() != "Nil" => true,
                        _ => false,
                    })
                    .flat_map(|node| {
                        Vec::from([
                            TokenTree::Ident(Ident::new("fn", Span::call_site())),
                            TokenTree::Ident(Ident::new(
                                (node.ident.to_string().to_lowercase() + "_default").as_str(),
                                Span::call_site(),
                            )),
                            TokenTree::Group(Group::new(
                                proc_macro::Delimiter::Parenthesis,
                                TokenStream::new(),
                            )),
                            TokenTree::Punct(Punct::new('-', Spacing::Joint)),
                            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                            TokenTree::Ident(Ident::new("Self", Span::call_site())),
                            TokenTree::Group(Group::new(
                                Delimiter::Brace,
                                TokenStream::from_iter(
                                    Vec::from([
                                        TokenTree::Ident(Ident::new("Self", Span::call_site())),
                                        TokenTree::Punct(Punct::new(':', Spacing::Joint)),
                                        TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                                        node.ident.clone(),
                                        TokenTree::Group(Group::new(
                                            Delimiter::Parenthesis,
                                            match node.value_type {
                                                Some(value) => enum_data.defaults.clone()
                                                    [value.to_string().as_str()]
                                                .1
                                                .clone()
                                                .into(),
                                                None => enum_data
                                                    .defaults
                                                    .first_key_value()
                                                    .unwrap()
                                                    .1
                                                     .1
                                                    .clone()
                                                    .into(),
                                            },
                                        )),
                                    ])
                                    .into_iter(),
                                ),
                            )),
                        ])
                        .into_iter()
                    }),
            ),
        )),
    ])
}
#[proc_macro_attribute]
pub fn watch(attribute: TokenStream, input: TokenStream) -> TokenStream {
    fn out_stream(stream: TokenStream, count: usize) {
        let split = "\t".repeat(count);
        stream.into_iter().for_each(|node| match node {
            TokenTree::Group(group) => {
                println!("{}Group:{}Delimiter:{:?}", split, group, group.delimiter());
                out_stream(group.stream(), count + 1)
            }
            TokenTree::Ident(ident) => println!("{}Ident:{}", split, ident),
            TokenTree::Punct(punct) => println!("{}Punct:{}", split, punct),
            TokenTree::Literal(literal) => println!("{}Literal:{}", split, literal),
        })
    }
    out_stream(attribute, 0);
    out_stream(input.clone(), 0);
    input
}
