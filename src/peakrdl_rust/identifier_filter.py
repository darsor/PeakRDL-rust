RUST_KEYWORDS = {
    # Base
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    # 2018 edition
    "async",
    "await",
    "dyn",
    # Reserved keywords
    "abstract",
    "become",
    "box",
    "do",
    "final",
    "macro",
    "override",
    "priv",
    "typeof",
    "unsized",
    "virtual",
    "yield",
    "try",
    "gen",
}

# these are reserved words that cannot be used as raw identifiers
# https://internals.rust-lang.org/t/raw-identifiers-dont-work-for-all-identifiers/9094
PATH_IDENTIFIERS = {
    "super",
    "self",
    "Self",
    "extern",
    "crate",
    # mod is included since we don't want to generate files named "mod.rs",
    # which would be the expected filename for "pub mod r#mod"
    "mod",
}


def kw_filter(s: str) -> str:
    """
    Make all user identifiers 'safe' and ensure they do not collide with
    Rust keywords.

    If a Rust keyword is encountered, create a raw identifier or add a "_"
    suffix if a raw identifier is not possible.
    """
    if s in PATH_IDENTIFIERS:
        return s + "_"
    if s in RUST_KEYWORDS:
        return "r#" + s
    return s


def kw_filter_path(s: str) -> str:
    """
    Make all user identifiers 'safe' as module path names.

    In Rust, if a module is named "r#<keyword>", the file path is still
    expected to be "<keyword>". However, some keywords such as "self"
    can not be used as raw identifiers and are instead suffixed with a "_",
    in which case the file path is expected to be "<keyword>_".
    """
    if s in PATH_IDENTIFIERS:
        return s + "_"
    return s
