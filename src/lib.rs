use janetrs::{
    declare_janet_mod, janet_fn, jpanic, tuple, util::check_fix_arity, Janet, JanetString,
    JanetTuple, TaggedJanet,
};

/// (str-ext/contains? pattern str-or-buff)
///
/// Returns true if and only if this `string|buffer` contains the given `pattern`. A
/// pattern must be string|buffer as well."
#[janet_fn(arity(fix(2)))]
pub fn contains(args: &mut [Janet]) -> Janet {
    match args[1].unwrap() {
        TaggedJanet::Buffer(buf) => match args[0].unwrap() {
            TaggedJanet::Buffer(pattern) => buf.contains(pattern).into(),
            TaggedJanet::String(pattern) => buf.contains(pattern).into(),
            _ => jpanic!(
                "bad slot #0, expected string|buffer, got {}",
                args[0].kind()
            ),
        },
        TaggedJanet::String(s) => match args[0].unwrap() {
            TaggedJanet::Buffer(pattern) => s.contains(pattern).into(),
            TaggedJanet::String(pattern) => s.contains(pattern).into(),
            _ => jpanic!(
                "bad slot #0, expected string|buffer, got {}",
                args[0].kind()
            ),
        },
        _ => jpanic!(
            "bad slot #1, expected string|buffer, got {}",
            args[1].kind()
        ),
    }
}

/// (str-ext/ascii? str-or-buff)
///
/// Returns true if and only if every byte in this `string|buffer` is ASCII.
#[janet_fn(arity(fix(1)))]
pub fn is_ascii(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf.is_ascii().into(),
        TaggedJanet::String(s) => s.is_ascii().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/utf8? str-or-buff)
///
/// Returns true if and only if the entire `string|buffer` is valid UTF-8.
#[janet_fn(arity(fix(1)))]
pub fn is_utf8(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf.is_utf8().into(),
        TaggedJanet::String(s) => s.is_utf8().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/chars str-or-buff)
///
/// Returns a tuple with the chars of the `string|buffer`. If invalid UTF-8 is
/// encountered, then the Unicode replacement codepoint is yielded instead.
#[janet_fn(arity(fix(1)))]
pub fn chars(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf.chars().collect::<JanetTuple>().into(),
        TaggedJanet::String(s) => s.chars().collect::<JanetTuple>().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/fields str-or-buff)
///
/// Returns a tuple with the fields of the `string|buffer` separated by contiguous
/// whitespace.
#[janet_fn(arity(fix(1)))]
pub fn fields(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf
            .fields()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        TaggedJanet::String(s) => s
            .fields()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/graphemes str-or-buff)
///
/// Returns a tuple with the grapheme clusters of the `string|buffer`. If invalid UTF-8 is
/// encountered, then the Unicode replacement codepoint is yielded instead.
#[janet_fn(arity(fix(1)))]
pub fn graphemes(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf.graphemes().collect::<JanetTuple>().into(),
        TaggedJanet::String(s) => s.graphemes().collect::<JanetTuple>().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/lines str-or-buff)
///
/// Returns a tuple with the lines of the `string|buffer` without the terminator. This
/// only recognizes `\\n` and `\\r\\n` as line terminator.
#[janet_fn(arity(fix(1)))]
pub fn lines(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf
            .lines()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        TaggedJanet::String(s) => s
            .lines()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/lines-with-terminator str-or-buff)
///
/// Returns a tuple with the lines of the `string|buffer` with the terminators. This only
/// recognizes `\\n` and `\\r\\n` as line terminator.
#[janet_fn(arity(fix(1)))]
pub fn lines_with_terminator(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf
            .lines_with_terminator()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        TaggedJanet::String(s) => s
            .lines_with_terminator()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/sentences str-or-buff)
///
/// Returns a tuple with the sentences of the `string|buffer`. Typically, a sentence will
/// include its trailing punctuation and whitespace.
///
/// When invalid UTF-8 is encountered, replacement codepoints are substituted.
#[janet_fn(arity(fix(1)))]
pub fn sentences(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf.sentences().collect::<JanetTuple>().into(),
        TaggedJanet::String(s) => s.sentences().collect::<JanetTuple>().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/words str-or-buff)
///
/// Returns a tuple with the words of the `string|buffer`. If invalid UTF-8 is
/// encountered, then the Unicode replacement codepoint is yielded instead.
#[janet_fn(arity(fix(1)))]
pub fn words(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf.words().collect::<JanetTuple>().into(),
        TaggedJanet::String(s) => s.words().collect::<JanetTuple>().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/words-with-breaks str-or-buff)
///
/// Returns a tuple with the words and breaks of the `string|buffer`. If invalid UTF-8 is
/// encountered, then the Unicode replacement codepoint is yielded instead.
#[janet_fn(arity(fix(1)))]
pub fn words_with_breaks(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => buf.words_with_breaks().collect::<JanetTuple>().into(),
        TaggedJanet::String(s) => s.words_with_breaks().collect::<JanetTuple>().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

/// (str-ext/utf8-chunks str-or-buff)
///
/// Returns a tuple with the first value being a tuple of valid UTF-8 chunks and the
/// second value beinf a tuple of invalid UTF-8 chunks.
///
/// Invalid UTF-8 bytes are always 1-3 bytes, which are determined via the "substitution
/// of maximal subparts"
#[janet_fn(arity(fix(1)))]
pub fn utf8_chunks(args: &mut [Janet]) -> Janet {
    match args[0].unwrap() {
        TaggedJanet::Buffer(buf) => {
            let (mut valid, mut invalid) = (Vec::with_capacity(10), Vec::with_capacity(10));

            buf.utf8_chunks().for_each(|chunk| {
                if !chunk.valid().is_empty() {
                    valid.push(chunk.valid());
                }

                if !chunk.invalid().is_empty() {
                    invalid.push(chunk.invalid());
                }
            });

            tuple![
                valid.into_iter().collect::<JanetTuple>(),
                invalid
                    .into_iter()
                    .map(JanetString::from)
                    .collect::<JanetTuple>()
            ]
            .into()
        },
        TaggedJanet::String(s) => {
            let (mut valid, mut invalid) = (Vec::with_capacity(10), Vec::with_capacity(10));

            s.utf8_chunks().for_each(|chunk| {
                if !chunk.valid().is_empty() {
                    valid.push(chunk.valid());
                }

                if !chunk.invalid().is_empty() {
                    invalid.push(chunk.invalid());
                }
            });

            tuple![
                valid.into_iter().collect::<JanetTuple>(),
                invalid
                    .into_iter()
                    .map(JanetString::from)
                    .collect::<JanetTuple>()
            ]
            .into()
        },
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

declare_janet_mod!("str-ext";
    {"contains?", contains},
    {"ascii?", is_ascii},
    {"utf8?", is_utf8},
    {"chars", chars},
    {"fields", fields},
    {"graphemes", graphemes},
    {"lines", lines},
    {"lines-with-terminator", lines_with_terminator},
    {"sentences", sentences},
    {"words", words},
    {"words-with-breaks", words_with_breaks},
    {"utf8-chunks", utf8_chunks}
);
