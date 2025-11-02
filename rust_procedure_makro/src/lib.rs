use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "afbryd" => "break",
        "aflus" => "dbg",
        "aflus_bekræft" => "debug_assert",
        "aflus_bekræft_ikke_lig" => "debug_assert_ne",
        "aflus_bekræft_lig" => "debug_assert_eq",
        "afvent" => "await",
        "aldrig" => "never",
        "asynkron" => "async",
        "bekræft" => "assert",
        "bekræft_ikke_lig" => "assert_ne",
        "bekræft_lig" => "assert_eq",
        "bliv" => "become",
        "Bogstavelig" => "Literal",
        "Boks" => "Box",
        "Bomm" => "Err",
        "Bommert" => "Error",
        "brug" => "use",
        "current" => "nuværende",
        "dynamisk" => "dyn",
        "egenskab" => "trait",
        "ekstern" => "extern",
        "ellers" => "else",
        "falsk" => "false",
        "fil" => "file",
        "flyt" => "move",
        "for" => "for",
        "foranderlig" => "mut",
        "forening" => "union",
        "formater" => "formater",
        "formater_argumenter" => "format_args",
        "fortegnelse" => "enum",
        "fortsæt" => "continue",
        "forvent" => "expect",
        "fra" => "from",
        "funktion" => "fn",
        "Gruppe" => "Group",
        "hent" => "get",
        "hent_eller_indsæt_med" => "get_or_insert_with",
        "hoved" => "main",
        "hvis" => "if",
        "hvor" => "where",
        "i" => "in",
        "Identifikator" => "Ident",
        "ind" => "into",
        "indsæt" => "insert",
        "ingen" => None?,
        "Ingenting" => "None",
        "inkluder" => "include",
        "inkluder_bytes" => "include_bytes",
        "inspicer" => "inspect",
        "iu" => "io",
        "kasse" => "crate",
        "kolonne" => "column",
        "kompileringsfejl" => "compile_error", 
        "konstant" => "const",
        "lad" => "let",
        "linje" => "line",
        "løkke" => "loop",
        "modul" => "mod",
        "Måske" => "Option",
        "Noget" => "Some",
        "ny" => "new",
        "offentlig" => "pub",
        "Ok" => "Ok",
        "omfang" => "span",
        "omgivelser" => "env",
        "Ordbog" => "HashMap",
        "panik" | "lort" | "møg" | "hovsa" => "panic",
        "passer_til" => "matches",
        "PoletStrøm" => "TokenStream",
        "PoletTræ" => "TokenTree",
        "prima" => "super",
        "procedure_makro" => "proc_macro",
        "reference" => "ref",
        "Resultat" => "Result",
        "returner" => "return",
        "sammenlign" => "match",
        "sammensæt" => "concat",
        "sandt" => "true",
        "Selv" => "Self",
        "selv" => "self",
        "skilletegn" => "delimiter",
        "skriv" => "write",
        "skrivlinje" => "writeln",
        "skub" => "push",
        "som" => "as",
        "som_afref" => "as_deref",
        "som_en_str" => "as_str",
        "som_ref" => "as_ref",
        "Standard" => "Default",
        "standard" => "default",
        "statisk" => "static",
        "Streng" => "String",        
        "strengkonverter" => "stringify",
        "struktur" => "struct",
        "strøm" => "stream",
        "så_længe" => "while",
        "Tegnsætning" => "Punct",
        "til_streng" => "to_string",
        "tillad" => "allow",
        "tråd" => "thread",
        "type" => "type",
        "udpak" => "unwrap",
        "udskriv" => "print",
        "udskrivlinje" => "println",
        "udsnit" => "slice",
        "udvid" => "extend",
        "usikker" => "unsafe",
        "utilgængelig" => "unreachable",
        "utilgængelig_kode" => "unreachable_code",
        "uvirkeliggjort" => "unimplemented",
        "vek" => "vec",
        "Vektor" => "Vec",
        "virkeliggør" => "impl",
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
