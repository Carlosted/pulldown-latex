use crate::attribute::{Dimension, DimensionUnit, Glue};

use super::{operator_table::is_delimiter, Argument, ErrorKind, InnerResult, Token};

/// Parse the right-hand side of a definition (TeXBook p. 271).
///
/// In this case, a definition is any of `def`, `edef`, `gdef`, or `xdef`.
///
/// Returns the control sequence, the parameter text, and the replacement text.
// TODO: make sure that the parameter text includes none of: `}`, or `%`
pub fn definition<'a>(input: &mut &'a str) -> InnerResult<(&'a str, &'a str, &'a str)> {
    let control_sequence = control_sequence(input)?;
    let (parameter_text, rest) = input.split_once('{').ok_or(ErrorKind::EndOfInput)?;
    *input = rest;
    let replacement_text = group_content(input)?;

    Ok((control_sequence, parameter_text, replacement_text))
}

pub fn argument<'a>(input: &mut &'a str) -> InnerResult<Argument<'a>> {
    *input = input.trim_start();

    if input.starts_with('{') {
        *input = &input[1..];
        let content = group_content(input)?;
        Ok(Argument::Group(content))
    } else {
        Ok(Argument::Token(token(input)?))
    }
}

/// Parses the inside of a group, when the first `{` is already parsed.
///
/// The output is the content within the group without the surrounding `{}`. This content is
/// guaranteed to be balanced.
pub fn group_content<'a>(input: &mut &'a str) -> InnerResult<&'a str> {
    let mut escaped = false;
    let mut in_comment = false;
    // In this case `Err` is the desired result.
    let end_index = input
        .char_indices()
        .try_fold(0usize, |balance, (index, c)| match c {
            _ if in_comment => {
                if c == '\n' {
                    in_comment = false;
                }
                Ok(balance)
            }
            '{' if !escaped => Ok(balance + 1),
            '}' if !escaped => {
                if balance == 0 {
                    Err(index)
                } else {
                    Ok(balance - 1)
                }
            }
            '\\' => {
                // Makes it so that two backslashes in a row don't escape the next character.
                escaped = !escaped;
                Ok(balance)
            }
            '%' if !escaped => {
                in_comment = true;
                Ok(balance)
            }
            _ => {
                escaped = false;
                Ok(balance)
            }
        });

    if let Err(end_index) = end_index {
        let (argument, rest) = input.split_at(end_index);
        *input = &rest[1..];
        Ok(argument)
    } else {
        Err(ErrorKind::EndOfInput)
    }
}

/// Converts a control sequence or character into its corresponding delimiter unicode
/// character.
///
/// Current delimiters supported are listed in TeXBook p. 146, and on https://temml.org/docs/en/supported ("delimiter" section).
pub fn delimiter(input: &mut &str) -> InnerResult<char> {
    *input = input.trim_start();
    let maybe_delim = token(input)?;
    match maybe_delim {
        Token::ControlSequence("lparen") => Ok('('),
        Token::ControlSequence("rparen") => Ok(')'),
        Token::ControlSequence("llparenthesis") => Ok('⦇'),
        Token::ControlSequence("rrparenthesis") => Ok('⦈'),
        Token::ControlSequence("lgroup") => Ok('⟮'),
        Token::ControlSequence("rgroup") => Ok('⟯'),

        Token::ControlSequence("lbrack") => Ok('['),
        Token::ControlSequence("rbrack") => Ok(']'),
        Token::ControlSequence("lBrack") => Ok('⟦'),
        Token::ControlSequence("rBrack") => Ok('⟧'),

        Token::ControlSequence("{") | Token::ControlSequence("lbrace") => Ok('{'),
        Token::ControlSequence("}") | Token::ControlSequence("rbrace") => Ok('}'),
        Token::ControlSequence("lBrace") => Ok('⦃'),
        Token::ControlSequence("rBrace") => Ok('⦄'),

        Token::ControlSequence("langle") => Ok('⟨'),
        Token::ControlSequence("rangle") => Ok('⟩'),
        Token::ControlSequence("lAngle") => Ok('⟪'),
        Token::ControlSequence("rAngle") => Ok('⟫'),
        Token::ControlSequence("llangle") => Ok('⦉'),
        Token::ControlSequence("rrangle") => Ok('⦊'),

        Token::ControlSequence("lfloor") => Ok('⌊'),
        Token::ControlSequence("rfloor") => Ok('⌋'),
        Token::ControlSequence("lceil") => Ok('⌈'),
        Token::ControlSequence("rceil") => Ok('⌉'),
        Token::ControlSequence("ulcorner") => Ok('┌'),
        Token::ControlSequence("urcorner") => Ok('┐'),
        Token::ControlSequence("llcorner") => Ok('└'),
        Token::ControlSequence("lrcorner") => Ok('┘'),

        Token::ControlSequence("lmoustache") => Ok('⎰'),
        Token::ControlSequence("rmoustache") => Ok('⎱'),

        Token::Character('/') => Ok('/'),
        Token::ControlSequence("backslash") => Ok('\\'),

        Token::ControlSequence("vert") => Ok('|'),
        Token::ControlSequence("|") | Token::ControlSequence("Vert") => Ok('‖'),
        Token::ControlSequence("uparrow") => Ok('↑'),
        Token::ControlSequence("Uparrow") => Ok('⇑'),
        Token::ControlSequence("downarrow") => Ok('↓'),
        Token::ControlSequence("Downarrow") => Ok('⇓'),
        Token::ControlSequence("updownarrow") => Ok('↕'),
        Token::ControlSequence("Updownarrow") => Ok('⇕'),
        Token::Character(c) if is_delimiter(c) => Ok(c),
        _ => Err(ErrorKind::Delimiter),
    }
}

/// Parse the right-hand side of a `futurelet` assignment (TeXBook p. 273).
///
/// Returns the control sequence and both following tokens.
pub fn futurelet_assignment<'a>(input: &mut &'a str) -> InnerResult<(&'a str, Token<'a>, Token<'a>)> {
    let control_sequence = control_sequence(input)?;

    let token1 = token(input)?;
    let token2 = token(input)?;
    Ok((control_sequence, token1, token2))
}

/// Parse the right-hand side of a `let` assignment (TeXBook p. 273).
///
/// Returns the control sequence and the value it is assigned to.
pub fn let_assignment<'a>(input: &mut &'a str) -> InnerResult<(&'a str, Token<'a>)> {
    let control_sequence = control_sequence(input)?;

    *input = input.trim_start();
    if let Some(s) = input.strip_prefix('=') {
        *input = s;
        one_optional_space(input);
    }

    let token = token(input)?;
    Ok((control_sequence, token))
}

/// Parse a control_sequence, including the leading `\`.
pub fn control_sequence<'a>(input: &mut &'a str) -> InnerResult<&'a str> {
    if input.starts_with('\\') {
        *input = &input[1..];
        rhs_control_sequence(input)
    } else {
        input
            .chars()
            .next()
            .map_or(Err(ErrorKind::EndOfInput), |_| {
                Err(ErrorKind::ControlSequence)
            })
    }
}

/// Parse the right side of a control sequence (`\` already being parsed).
///
/// A control sequence can be of the form `\controlsequence`, or `\#` (control symbol).
pub fn rhs_control_sequence<'a>(input: &mut &'a str) -> InnerResult<&'a str> {
    if input.is_empty() {
        return Err(ErrorKind::EndOfInput);
    }

    let len = input
        .chars()
        .take_while(|c| c.is_ascii_alphabetic())
        .count()
        .max(1);

    let (control_sequence, rest) = input.split_at(len);
    *input = rest.trim_start();
    Ok(control_sequence)
}

/// Parse a glue (TeXBook p. 267).
pub fn glue(input: &mut &str) -> InnerResult<Glue> {
    let mut dimen = (dimension(input)?, None, None);
    if let Some(s) = input.trim_start().strip_prefix("plus") {
        *input = s;
        dimen.1 = Some(dimension(input)?);
    }
    if let Some(s) = input.trim_start().strip_prefix("minus") {
        *input = s;
        dimen.2 = Some(dimension(input)?);
    }
    Ok(dimen)
}

/// Parse a glue that can only be specified in math units (mu)
pub fn math_glue(input: &mut &str) -> InnerResult<Glue> {
    let mut dimen = (math_dimension(input)?, None, None);
    if let Some(s) = input.trim_start().strip_prefix("plus") {
        *input = s;
        dimen.1 = Some(math_dimension(input)?);
    }
    if let Some(s) = input.trim_start().strip_prefix("minus") {
        *input = s;
        dimen.2 = Some(math_dimension(input)?);
    }
    Ok(dimen)
}

/// Parse a dimension (TeXBook p. 266).
pub fn dimension(input: &mut &str) -> InnerResult<Dimension> {
    let number = floating_point(input)?;
    let unit = dimension_unit(input)?;
    Ok((number, unit))
}

/// Parse a dimension that can only be specified in math units (mu)
pub fn math_dimension(input: &mut &str) -> InnerResult<Dimension> {
    let number = floating_point(input)? as f32;
    *input = input.trim_start();
    input.strip_prefix("mu").ok_or(ErrorKind::MathUnit)?;
    let unit = DimensionUnit::Mu;
    Ok((number, unit))
}

/// Parse a dimension unit (TeXBook p. 266).
pub fn dimension_unit(input: &mut &str) -> InnerResult<DimensionUnit> {
    *input = input.trim_start();
    if input.len() < 2 {
        return Err(ErrorKind::EndOfInput);
    }

    let unit = input.get(0..2).ok_or(ErrorKind::DimensionUnit)?;
    let unit = match unit {
        "em" => DimensionUnit::Em,
        "ex" => DimensionUnit::Ex,
        "pt" => DimensionUnit::Pt,
        "pc" => DimensionUnit::Pc,
        "in" => DimensionUnit::In,
        "bp" => DimensionUnit::Bp,
        "cm" => DimensionUnit::Cm,
        "mm" => DimensionUnit::Mm,
        "dd" => DimensionUnit::Dd,
        "cc" => DimensionUnit::Cc,
        "sp" => DimensionUnit::Sp,
        "mu" => DimensionUnit::Mu,
        _ => return Err(ErrorKind::DimensionUnit),
    };

    *input = &input[2..];
    one_optional_space(input);

    Ok(unit)
}

/// Parse an integer that may be positive or negative and may be represented as octal, decimal,
/// hexadecimal, or a character code (TeXBook p. 265).
pub fn integer(input: &mut &str) -> InnerResult<isize> {
    let signum = signs(input)?;

    // The following character must be ascii.
    let next_char = input.chars().next().ok_or(ErrorKind::EndOfInput)?;
    if next_char.is_ascii_digit() {
        return Ok(decimal(input) as isize * signum);
    }
    *input = &input[1..];
    let unsigned_int = match next_char {
        '`' => {
            let mut next_byte = *input.as_bytes().first().ok_or(ErrorKind::EndOfInput)?;
            if next_byte == b'\\' {
                *input = &input[1..];
                next_byte = *input.as_bytes().first().ok_or(ErrorKind::EndOfInput)?;
            }
            if next_byte.is_ascii() {
                *input = &input[1..];
                next_byte as usize
            } else {
                return Err(ErrorKind::CharacterNumber);
            }
        }
        '\'' => octal(input),
        '"' => hexadecimal(input),
        _ => return Err(ErrorKind::Number),
    };

    Ok(unsigned_int as isize * signum)
}

/// Parse the signs in front of a number, returning the signum.
pub fn signs(input: &mut &str) -> InnerResult<isize> {
    let signs = input.trim_start();
    let mut minus_count = 0;
    *input = signs
        .trim_start_matches(|c: char| {
            if c == '-' {
                minus_count += 1;
                true
            } else {
                c == '+' || c.is_whitespace()
            }
        })
        .trim_start();
    Ok(if minus_count % 2 == 0 { 1 } else { -1 })
}

/// Parse a base 16 unsigned number.
pub fn hexadecimal(input: &mut &str) -> usize {
    let mut number = 0;
    *input = input.trim_start_matches(|c: char| {
        if c.is_ascii_alphanumeric() && c < 'G' {
            number =
                number * 16 + c.to_digit(16).expect("the character is a valid hex digit") as usize;
            true
        } else {
            false
        }
    });
    one_optional_space(input);

    number
}

/// Parse a floating point number (named `factor` in TeXBook p. 266).
pub fn floating_point(input: &mut &str) -> InnerResult<f32> {
    let signum = signs(input)?;

    let mut number = 0.;
    *input = input.trim_start_matches(|c: char| {
        if c.is_ascii_digit() {
            number = number * 10. + (c as u8 - b'0') as f32;
            true
        } else {
            false
        }
    });

    if let Some(stripped_decimal_point) = input.strip_prefix(|c| c == '.' || c == ',') {
        let mut decimal = 0.;
        let mut decimal_divisor = 1.;
        *input = stripped_decimal_point.trim_start_matches(|c: char| {
            if c.is_ascii_digit() {
                decimal = decimal * 10. + (c as u8 - b'0') as f32;
                decimal_divisor *= 10.;
                true
            } else {
                false
            }
        });
        number += decimal / decimal_divisor;
    };

    Ok(signum as f32 * number)
}

/// Parse a base 10 unsigned number.
pub fn decimal(input: &mut &str) -> usize {
    let mut number = 0;
    *input = input.trim_start_matches(|c: char| {
        if c.is_ascii_digit() {
            number = number * 10 + (c as u8 - b'0') as usize;
            true
        } else {
            false
        }
    });
    one_optional_space(input);

    number
}

/// Parse a base 8 unsigned number.
pub fn octal(input: &mut &str) -> usize {
    let mut number = 0;
    *input = input.trim_start_matches(|c: char| {
        if c.is_ascii_digit() {
            number = number * 8 + (c as u8 - b'0') as usize;
            true
        } else {
            false
        }
    });
    one_optional_space(input);

    number
}

/// Parse an optional space.
pub fn one_optional_space(input: &mut &str) -> bool {
    let mut chars = input.chars();
    if chars.next().is_some_and(|c| c.is_whitespace()) {
        *input = &input[1..];
        true
    } else {
        false
    }
}

/// Return the next token in the input.
///
/// A token will never be whitespace, and will never be inside of a comment.
pub fn token<'a>(input: &mut &'a str) -> InnerResult<Token<'a>> {
    *input = input.trim_start();
    match input.chars().next() {
        Some('\\') => {
            *input = &input[1..];
            Ok(Token::ControlSequence(rhs_control_sequence(input)?))
        },
        Some('%') => {
           let (_, rest) = input.split_once('\n').ok_or(ErrorKind::EndOfInput)?;
           *input = rest;
           token(input)
        }
        Some(c) => {
            *input = &input[c.len_utf8()..];
            Ok(Token::Character(c))
        }
        None => Err(ErrorKind::EndOfInput),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        attribute::DimensionUnit,
        parser::{lex, Token},
    };

    #[test]
    fn signs() {
        let mut input = "  +    +-   \\test";
        assert_eq!(lex::signs(&mut input).unwrap(), -1);
        assert_eq!(input, "\\test");
    }

    #[test]
    fn no_signs() {
        let mut input = "\\mycommand";
        assert_eq!(lex::signs(&mut input).unwrap(), 1);
        assert_eq!(input, "\\mycommand");
    }

    // A complex exanple from problem 20.7 in TeXBook (p. 205):
    // \def\cs AB#1#2C$#3\$ {#3{ab#1}#1 c##\x #2}
    #[test]
    fn definition_texbook() {
        let mut input = "\\cs AB#1#2C$#3\\$ {#3{ab#1}#1 c##\\x #2}";

        let (cs, param, repl) = lex::definition(&mut input).unwrap();
        assert_eq!(cs, "cs");
        assert_eq!(param, "AB#1#2C$#3\\$ ");
        assert_eq!(repl, "#3{ab#1}#1 c##\\x #2");
        assert_eq!(input, "");
    }

    #[test]
    fn complex_definition() {
        let mut input = r"\foo #1\test#2#{##\####2#2 \{{\}} \{\{\{} 5 + 5 = 10";
        let (cs, param, repl) = lex::definition(&mut input).unwrap();

        assert_eq!(cs, "foo");
        assert_eq!(param, r"#1\test#2#");
        assert_eq!(repl, r"##\####2#2 \{{\}} \{\{\{");
        assert_eq!(input, " 5 + 5 = 10");
    }

    #[test]
    fn let_assignment() {
        let mut input = r"\foo = \bar";
        let (cs, token) = lex::let_assignment(&mut input).unwrap();

        assert_eq!(cs, "foo");
        assert_eq!(token, Token::ControlSequence("bar".into()));
        assert_eq!(input, "");
    }

    #[test]
    fn futurelet_assignment() {
        let mut input = r"\foo\bar\baz blah";
        let (cs, token1, token2) = lex::futurelet_assignment(&mut input).unwrap();

        assert_eq!(cs, "foo");
        assert_eq!(token1, Token::ControlSequence("bar".into()));
        assert_eq!(token2, Token::ControlSequence("baz".into()));
        assert_eq!(input, "blah");
    }

    #[test]
    fn dimension() {
        let mut input = "1.2pt";
        let dim = lex::dimension(&mut input).unwrap();

        assert_eq!(dim, (1.2, DimensionUnit::Pt));
        assert_eq!(input, "");
    }

    #[test]
    fn complex_glue() {
        let mut input = "1.2 pt plus 3.4pt minus 5.6pt nope";
        let glue = lex::glue(&mut input).unwrap();

        assert_eq!(
            glue,
            (
                (1.2, DimensionUnit::Pt),
                Some((3.4, DimensionUnit::Pt)),
                Some((5.6, DimensionUnit::Pt))
            )
        );
        assert_eq!(input, "nope");
    }

    #[test]
    fn numbers() {
        let mut input = "123 -\"AEF24 --'3475 `\\a -.47";
        assert_eq!(lex::integer(&mut input).unwrap(), 123);
        assert_eq!(lex::integer(&mut input).unwrap(), -716580);
        assert_eq!(lex::integer(&mut input).unwrap(), 1853);
        assert_eq!(lex::integer(&mut input).unwrap(), 97);
        assert_eq!(lex::floating_point(&mut input).unwrap(), -0.47);
        assert_eq!(input, "");
    }

    #[test]
    fn group_content() {
        let mut input =
            "this { { is a test } to see if { the content parsing { of this } } } works }";
        let content = lex::group_content(&mut input).unwrap();
        assert_eq!(
            content,
            "this { { is a test } to see if { the content parsing { of this } } } works "
        );
    }
}
