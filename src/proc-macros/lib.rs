use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree as TT};

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

    let mut orig_chars = lit.trim_start_matches('r').trim_matches('"').chars();
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
                chars.take(name.len() + 1).for_each(drop);

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
