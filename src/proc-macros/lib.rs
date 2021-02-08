use proc_macro::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree as TT,
};

#[doc(hidden)]
#[proc_macro]
pub fn __tuple_bindings__(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let name = if let Some(TT::Group(group)) = tokens.next() {
        if let Some(TT::Ident(ident)) = group.stream().into_iter().next() {
            ident
        } else {
            panic!()
        }
    } else {
        panic!()
    };

    if !matches!(tokens.next(), Some(TT::Punct(x)) if x.as_char() == ',') {
        panic!("missing comma");
    }

    let variant = if let Some(TT::Group(group)) = tokens.next() {
        if let Some(TT::Ident(ident)) = group.stream().into_iter().next() {
            ident
        } else {
            panic!()
        }
    } else {
        panic!()
    };

    if !matches!(tokens.next(), Some(TT::Punct(x)) if x.as_char() == ',') {
        panic!("missing comma");
    }

    let span = name.span();

    let mut args = vec![];
    let mut idx = 0;
    while let Some(_tok) = tokens.next() {
        let name = format!("_{}", idx);
        args.push(TT::Ident(Ident::new(&name, span)));
        args.push(TT::Punct(Punct::new(',', Spacing::Alone)));

        if !matches!(tokens.next(), Some(TT::Punct(x)) if x.as_char() == ',') {
            panic!("missing comma");
        }
        idx += 1;
    }

    vec![
        TT::Ident(name),
        TT::Punct(Punct::new(':', Spacing::Joint)),
        TT::Punct(Punct::new(':', Spacing::Alone)),
        TT::Ident(variant),
        TT::Group(Group::new(
            Delimiter::Parenthesis,
            args.into_iter().collect(),
        )),
    ]
    .into_iter()
    .collect()
}

#[doc(hidden)]
#[proc_macro]
pub fn __struct_string__(input: TokenStream) -> TokenStream {
    // `__struct_string__!(formatter, literal)`

    let mut tokens = input.into_iter();
    let tok = tokens.next();
    let fmt = if let Some(TT::Group(group)) = tok {
        if let Some(TT::Ident(ident)) = group.stream().into_iter().next() {
            ident
        } else {
            panic!()
        }
    } else {
        panic!()
    };

    if !matches!(tokens.next(), Some(TT::Punct(x)) if x.as_char() == ',') {
        panic!("missing comma");
    }

    let (lit_span, lit) = if let Some(TT::Literal(lit)) = tokens.next() {
        (lit.span(), format!("{}", lit))
    } else {
        panic!()
    };

    let span = lit_span;

    let mut orig_chars = lit
        .trim_start_matches('r')
        .trim_matches('"')
        .chars()
        .peekable();
    let chars = orig_chars.by_ref();

    let mut string = String::new();
    let mut args = vec![];
    while let Some(c) = chars.next() {
        match c {
            '{' => {
                let name = chars
                    .clone()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect::<String>();
                chars.take(name.len()).for_each(drop);

                match chars.peek() {
                    Some(':') => {
                        chars.next();
                        if let Some('?') = chars.peek() {
                            string.push_str("{:?}");
                            args.push(Ident::new(&name, span));
                            chars.take(2).for_each(drop);
                            continue;
                        }
                    }
                    Some('}') => chars.take(1).for_each(drop),
                    // this crate is not meant to report proper errors,
                    // so just do nothing here and silently skip.
                    _ => {}
                }

                string.push_str("{}");
                args.push(Ident::new(&name, span));
            }
            c => string.push(c),
        }
    }

    let args = vec![
        TT::Ident(fmt),
        TT::Punct(Punct::new(',', Spacing::Alone)),
        TT::Literal(Literal::string(string.trim())),
        TT::Punct(Punct::new(',', Spacing::Alone)),
    ]
    .into_iter()
    .chain(
        args.into_iter()
            .flat_map(|x| vec![TT::Ident(x), TT::Punct(Punct::new(',', Spacing::Alone))]),
    );

    vec![
        TT::Ident(Ident::new("write", span)),
        TT::Punct(Punct::new('!', Spacing::Alone)),
        TT::Group(Group::new(Delimiter::Parenthesis, args.collect())),
    ]
    .into_iter()
    .collect()
}

fn map(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter().peekable();
    let mut ret = TokenStream::new();
    while let Some(tt) = tokens.next() {
        ret.extend(Some(match tt {
            TT::Punct(ref p) if p.as_char() == '@' => match tokens.peek() {
                Some(&TT::Group(ref group)) if group.delimiter() == Delimiter::None => {
                    ret.extend(map(group.stream()));
                    drop(tokens.next());
                    continue;
                }
                Some(TT::Punct(ref p)) if p.as_char() == '@' => tokens.next().unwrap(),
                _ => continue,
            },
            TT::Group(group) => Group::new(group.delimiter(), map(group.stream())).into(),
            _ => tt,
        }));
    }
    ret
}

/// This macro is copied from the [`defile`](https://lib.rs/defile) crate
#[doc(hidden)]
#[proc_macro_derive(__expr_hack__)]
pub fn __expr_hack__(input: TokenStream) -> TokenStream {
    // enum
    // EnumName
    // {
    //     VariantName
    //     =
    //     (
    //         stringify
    //         !
    //         (
    //             <input>
    //         )
    // , 0).1,}

    let mut tokens = input.into_iter();
    // `enum EnumName`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `{ <tokens> }`
    let mut tokens = if let Some(TT::Group(it)) = tokens.next() {
        it
    } else {
        panic!()
    }
    .stream()
    .into_iter();
    // `VariantName =`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `( <tokens> )`
    let mut tokens = if let Some(TT::Group(it)) = tokens.next() {
        it
    } else {
        panic!()
    }
    .stream()
    .into_iter();
    // `stringify !`
    let _ = tokens.by_ref().take(2).for_each(drop);
    // `( <input> )`
    let input = if let Some(TT::Group(it)) = tokens.next() {
        it
    } else {
        panic!()
    }
    .stream();
    let ret = map(input);
    let span = Span::call_site();
    vec![
        TT::Ident(Ident::new("macro_rules", span)),
        TT::Punct(Punct::new('!', Spacing::Alone)),
        TT::Ident(Ident::new("__defile__Hack__", span)),
        TT::Group(Group::new(
            Delimiter::Brace,
            vec![
                TT::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
                TT::Punct(Punct::new('=', Spacing::Joint)),
                TT::Punct(Punct::new('>', Spacing::Alone)),
                TT::Group(Group::new(Delimiter::Parenthesis, ret)),
            ]
            .into_iter()
            .collect(),
        )),
    ]
    .into_iter()
    .collect()
}
