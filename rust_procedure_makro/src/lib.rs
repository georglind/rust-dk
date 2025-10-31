use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Bomm" => "Err",
        "Ok" => "Ok",
        "Streng" => "String",
        "Ordbog" => "HashMap",
        "Standard" => "Default",
        "Bommert" => "Error",
        "Måske" => "Option",
        "Noget" => "Some",
        "Ingenting" => "None",
        "Resultat" => "Result",
        "Selv" => "Self",
        "skrivlinje" => "println",
        "afbryd" => "break",
        "asynkron" => "async",
        "afvent" => "await",
        "bliv" => "become",
        "fortsæt" => "continue",
        "løkke" => "loop",
        "flyt" => "move",
        "kasse" => "crate",
        "utilgængelig_kode" => "unreachable_code",
        "som" => "as",
        "konstant" => "const",
        "egenskab" => "trait",
        "usikker" => "unsafe",
        "i" => "in",
        "fra" => "from",
        "dynamisk" => "dyn",
        "udpak" => "unwrap",
        "standard" => "default",
        "som_ref" => "as_ref",
        "som_afref" => "as_deref",
        "inspicer" => "inspect",
        "iu" => "io",
        "ekstern" => "extern",
        "falsk" => "false",
        "funktion" => "fn",
        "prima" => "super",
        "indsæt" => "insert",
        "hent" => "get",
        "tillad" => "allow",
        "panik" | "lort" | "møg" | "hovsa" => "panic",
        "modul" => "mod",
        "foranderlig" => "mut",
        "ny" => "new",
        "hvor" => "where",
        "for" => "for",
        "hent_eller_indsæt_med" => "get_or_insert_with",
        "hoved" => "main",
        "offentlig" => "pub",
        "ingen" => None?,
        "returner" => "return",
        "virkeliggør" => "impl",
        "reference" => "ref",
        "sammenlign" => "match",
        "hvis" => "if",
        "ellers" => "else",
        "selv" => "self",
        "lad" => "let",
        "statisk" => "static",
        "struktur" => "struct",
        "forvent" => "expect",
        "så_længe" => "while",
        "brug" => "use",
        "ind" => "into",
        "sandt" => "true",
        "optælling" => "enum",
        "Gruppe" => "Group",
        "Identifikator" => "Ident",
        "PoletStrøm" => "TokenStream",
        "PoletTræ" => "TokenTree",
        "til_streng" => "to_string",
        "som_en_str" => "as_str",
        "omfang" => "span",
        "Vektor" => "Vec",
        "vek" => "vec",
        "strøm" => "stream",
        "skub" => "push",
        "udvid" => "extend",
        "skilletegn" => "delimiter",
        "Tegnsætning" => "Punct",
        "Bogstavelig" => "Literal",
        "procedure_makro" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn rust(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
