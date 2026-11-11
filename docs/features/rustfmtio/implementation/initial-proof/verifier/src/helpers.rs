fn comments(source: &str) -> Vec<String> {
    let mut starts = vec![0];
    for (i, b) in source.bytes().enumerate() { if b == b'\n' { starts.push(i + 1); } }
    let index = |p: proc_macro2::LineColumn| {
        let start = starts[p.line - 1];
        let tail = &source[start..];
        start + tail.char_indices().nth(p.column).map_or(tail.len(), |(i,_)| i)
    };
    fn ranges(ts: proc_macro2::TokenStream, out: &mut Vec<proc_macro2::Span>) {
        for token in ts {
            match token {
                proc_macro2::TokenTree::Group(g) => {
                    out.push(g.span_open()); ranges(g.stream(), out); out.push(g.span_close());
                }
                t => out.push(t.span()),
            }
        }
    }
    let mut spans = Vec::new();
    ranges(source.parse().unwrap(), &mut spans);
    let mut intervals: Vec<_> = spans.iter().map(|s| (index(s.start()), index(s.end()))).collect();
    intervals.sort();
    let mut end = 0;
    let mut comments = Vec::new();
    for (a, b) in intervals {
        if a > end && !source[end..a].trim().is_empty() { comments.push(source[end..a].trim().to_string()); }
        end = end.max(b);
    }
    if !source[end..].trim().is_empty() { comments.push(source[end..].trim().to_string()); }
    comments
}
fn parse(source: &str) -> (syn::File, BTreeMap<&'static str,usize>) {
    if let Ok(file) = syn::parse_file(source) { return (file,BTreeMap::new()); }
    fn repair(tokens: proc_macro2::TokenStream, count: &mut BTreeMap<&'static str,usize>) -> proc_macro2::TokenStream {
        use proc_macro2::{TokenTree,Delimiter,Group,Ident};
        let items: Vec<_> = tokens.into_iter().collect();
        items.iter().enumerate().map(|(i,t)| match t {
            TokenTree::Group(g) => {
                let mut group=Group::new(g.delimiter(),repair(g.stream(),count));
                group.set_span(g.span());TokenTree::Group(group)
            }
            TokenTree::Ident(id) if id=="in"
                && matches!(items.get(i+1),Some(TokenTree::Group(g)) if g.delimiter()==Delimiter::Parenthesis)
                && (i==0 || !matches!(items[i-1],TokenTree::Ident(_))) => {
                *count.entry("reserved_in_call").or_insert(0)+=1;TokenTree::Ident(Ident::new_raw("in",id.span()))
            }
            TokenTree::Ident(id) if id=="_"
                && matches!(items.get(i+1),Some(TokenTree::Punct(p)) if p.as_char()==':')
                && matches!(items.get(i+2),Some(TokenTree::Punct(p)) if p.as_char()==':') => {
                *count.entry("placeholder_namespace").or_insert(0)+=1;
                TokenTree::Ident(Ident::new("__kuna_unresolved_namespace",id.span()))
            }
            _ => t.clone(),
        }).collect()
    }
    let mut count=BTreeMap::new();
    let tokens=repair(source.parse().unwrap(), &mut count);
    if count.is_empty() {
        let error=syn::parse_file(source).unwrap_err(); let at=error.span().start();
        panic!("pre-existing parse: {error} at {at:?}: {}",source.lines().nth(at.line-1).unwrap_or(""));
    }
    let file=syn::parse2(tokens).unwrap_or_else(|error| {
        let at=error.span().start();
        panic!("parse after keyword repair: {error} at {at:?}: {}",source.lines().nth(at.line-1).unwrap_or(""));
    });
    (file,count)
}
