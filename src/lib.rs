use janetrs::{
    janet_fn, janet_mod, jpanic, tuple,
    types::{Janet, JanetBuffer, JanetString, JanetTuple, JanetType},
    util::check_fix_arity,
};

#[janet_fn]
pub fn contains(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 2);

    match args[1].kind() {
        JanetType::Buffer => {
            let buff = args[0].unwrap::<JanetBuffer>().unwrap();

            match args[0].kind() {
                JanetType::Buffer => {
                    let pattern = args[0].unwrap::<JanetBuffer>().unwrap();
                    buff.contains(pattern).into()
                },
                JanetType::String => {
                    let pattern = args[0].unwrap::<JanetString>().unwrap();
                    buff.contains(pattern).into()
                },
                _ => jpanic!(
                    "bad slot #0, expected string|buffer, got {}",
                    args[0].kind()
                ),
            }
        },
        JanetType::String => {
            let s = args[0].unwrap::<JanetString>().unwrap();

            match args[0].kind() {
                JanetType::Buffer => {
                    let pattern = args[0].unwrap::<JanetBuffer>().unwrap();
                    s.contains(pattern).into()
                },
                JanetType::String => {
                    let pattern = args[0].unwrap::<JanetString>().unwrap();
                    s.contains(pattern).into()
                },
                _ => jpanic!(
                    "bad slot #0, expected string|buffer, got {}",
                    args[0].kind()
                ),
            }
        },
        _ => jpanic!(
            "bad slot #1, expected string|buffer, got {}",
            args[1].kind()
        ),
    }
}

#[janet_fn]
pub fn is_ascii(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0].unwrap::<JanetBuffer>().unwrap().is_ascii().into(),
        JanetType::String => args[0].unwrap::<JanetString>().unwrap().is_ascii().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

#[janet_fn]
pub fn is_utf8(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0].unwrap::<JanetBuffer>().unwrap().is_utf8().into(),
        JanetType::String => args[0].unwrap::<JanetString>().unwrap().is_utf8().into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

#[janet_fn]
pub fn chars(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0]
            .unwrap::<JanetBuffer>()
            .unwrap()
            .chars()
            .collect::<JanetTuple>()
            .into(),
        JanetType::String => args[0]
            .unwrap::<JanetString>()
            .unwrap()
            .chars()
            .collect::<JanetTuple>()
            .into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

#[janet_fn]
pub fn fields(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0]
            .unwrap::<JanetBuffer>()
            .unwrap()
            .fields()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        JanetType::String => args[0]
            .unwrap::<JanetString>()
            .unwrap()
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

#[janet_fn]
pub fn graphemes(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0]
            .unwrap::<JanetBuffer>()
            .unwrap()
            .graphemes()
            .collect::<JanetTuple>()
            .into(),
        JanetType::String => args[0]
            .unwrap::<JanetString>()
            .unwrap()
            .graphemes()
            .collect::<JanetTuple>()
            .into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

#[janet_fn]
pub fn lines(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0]
            .unwrap::<JanetBuffer>()
            .unwrap()
            .lines()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        JanetType::String => args[0]
            .unwrap::<JanetString>()
            .unwrap()
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

#[janet_fn]
pub fn lines_with_terminator(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0]
            .unwrap::<JanetBuffer>()
            .unwrap()
            .lines_with_terminator()
            .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        JanetType::String => args[0]
            .unwrap::<JanetString>()
            .unwrap()
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

#[janet_fn]
pub fn sentences(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0]
            .unwrap::<JanetBuffer>()
            .unwrap()
            .sentences()
            .collect::<JanetTuple>()
            .into(),
        JanetType::String => args[0]
            .unwrap::<JanetString>()
            .unwrap()
            .sentences()
            .collect::<JanetTuple>()
            .into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

#[janet_fn]
pub fn words(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0]
            .unwrap::<JanetBuffer>()
            .unwrap()
            .words()
            // .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        JanetType::String => args[0]
            .unwrap::<JanetString>()
            .unwrap()
            .words()
            // .map(JanetString::from)
            .collect::<JanetTuple>()
            .into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

#[janet_fn]
pub fn words_with_breaks(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => args[0]
            .unwrap::<JanetBuffer>()
            .unwrap()
            .words_with_breaks()
            .collect::<JanetTuple>()
            .into(),
        JanetType::String => args[0]
            .unwrap::<JanetString>()
            .unwrap()
            .words_with_breaks()
            .collect::<JanetTuple>()
            .into(),
        _ => jpanic!(
            "bad slot #0, expected string|buffer, got {}",
            args[0].kind()
        ),
    }
}

#[janet_fn]
pub fn utf8_chunks(args: &mut [Janet]) -> Janet {
    check_fix_arity(args, 1);

    match args[0].kind() {
        JanetType::Buffer => {
            let buff = args[0].unwrap::<JanetBuffer>().unwrap();

            let (mut valid, mut invalid) = (Vec::with_capacity(10), Vec::with_capacity(10));

            buff.utf8_chunks().for_each(|chunk| {
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
        JanetType::String => {
            let s = args[0].unwrap::<JanetString>().unwrap();

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

janet_mod!("str-ext";
    {"contains?", contains, "(str-ext/contains? pattern str-or-buff)\n\nReturns true if and only if this string|buffer contains the given pattern. A pattern must be string|buffer as well."},
    {"ascii?", is_ascii, "(str-ext/ascii? str-or-buff)\n\nReturns true if and only if every byte in this string|buffer is ASCII."},
    {"utf8?", is_utf8, "(str-ext/utf8? str-or-buff)\n\nReturns true if and only if the entire string|buffer is valid UTF-8."},
    {"chars", chars, "(str-ext/chars str-or-buff)\n\nReturns a tuple with the chars of the string|buffer. If invalid UTF-8 is encountered, then the Unicode replacement codepoint is yielded instead."},
    {"fields", fields, "(str-ext/chars str-or-buff)\n\nReturns a tuple with the fields of the string|buffer separated by contiguous whitespace."},
    {"graphemes", graphemes, "(str-ext/graphemes str-or-buff)\n\nReturns a tuple with the grapheme clusters of the string|buffer. If invalid UTF-8 is encountered, then the Unicode replacement codepoint is yielded instead."},
    {"lines", lines, "(str-ext/lines str-or-buff)\n\nReturns a tuple with the lines of the string|buffer without the terminator. This only recognizes `\\n` and `\\r\\n` as line terminator."},
    {"lines-with-terminator", lines_with_terminator, "(str-ext/lines-with-terminator str-or-buff)\n\nReturns a tuple with the lines of the string|buffer with the terminators. This only recognizes `\\n` and `\\r\\n` as line terminator."},
    {"sentences", sentences, "(str-ext/sentences str-or-buff)\n\nReturns a tuple with the sentences of the string|buffer. Typically, a sentence will include its trailing punctuation and whitespace.\n\nWhen invalid UTF-8 is encountered, replacement codepoints are substituted."},
    {"words", words, "(str-ext/words str-or-buff)\n\nReturns a tuple with the words of the string|buffer. If invalid UTF-8 is encountered, then the Unicode replacement codepoint is yielded instead."},
    {"words-with-breaks", words_with_breaks, "(str-ext/words-with-breaks str-or-buff)\n\nReturns a tuple with the words and breaks of the string|buffer. If invalid UTF-8 is encountered, then the Unicode replacement codepoint is yielded instead."},
    {"utf8-chunks", utf8_chunks, "(str-ext/utf8-chunks str-or-buff)\n\nReturns a tuple with the first value being a tuple of valid UTF-8 chunks and the second value beinf a tuple of invalid UTF-8 chunks.\n\nInvalid UTF-8 bytes are always 1-3 bytes, which are determined via the \"substitution of maximal subparts\""}
);
