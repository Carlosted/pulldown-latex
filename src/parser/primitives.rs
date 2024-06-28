//! A module that implements the behavior of every primitive of the supported LaTeX syntax. This
//! includes every primitive macro and active character.

use core::panic;

use crate::{
    attribute::{DimensionUnit, Font},
    event::{
       ArrayColumn as AC, ColorChange as CC, ColorTarget as CT, ColumnAlignment, Content as C, DelimiterSize, DelimiterType, Event as E, Grouping as G, ScriptPosition as SP, ScriptType as ST, StateChange as SC, Style as S, Visual as V
    },
};

use super::{
    lex, tables::{char_delimiter_map, control_sequence_delimiter_map, is_binary, is_primitive_color, is_relation, token_to_delim}, AlignmentCount, Argument, CharToken, ErrorKind, InnerParser, InnerResult, Instruction as I, Token
};

impl<'a, 'b> InnerParser<'a, 'b> {
    /// Handle a character token, returning a corresponding event.
    ///
    /// This function specially treats numbers as `mi`.
    ///
    /// ## Panics
    /// - This function will panic if the `\` or `%` character is given
    pub(super) fn handle_char_token(&mut self, token: CharToken<'a>) -> InnerResult<()> {
        let instruction = I::Event(match token.into() {
            '\\' => panic!("(internal error: please report) the `\\` character should never be observed as a token"),
            '%' => panic!("(internal error: please report) the `%` character should never be observed as a token"),
            // TODO: This obviously does not work
            '_' => {
                if self.state.handling_argument {
                    return Err(ErrorKind::ScriptAsArgument)
                }
                self.buffer.extend([
                    I::Event(E::Begin(G::Normal)),
                ]);
                self.content = token.as_str();
                E::End
            }
            '^' => {
                if self.state.handling_argument {
                    return Err(ErrorKind::ScriptAsArgument)
                }
                self.buffer.extend([
                    I::Event(E::Begin(G::Normal)),
                ]);
                self.content = token.as_str();
                E::End
            }
            '$' => return Err(ErrorKind::MathShift),
            '#' => return Err(ErrorKind::HashSign),
            '&' if self
                    .state
                    .allowed_alignment_count
                    .as_deref()
                    .is_some_and(AlignmentCount::can_increment) => {
                       self
                           .state
                           .allowed_alignment_count
                           .as_mut()
                           .expect("we have checked that `allowed_alignment_count` is Some")
                           .increment();
                       E::Alignment  
                    },
            '{' => {
                let str = &mut self.content;
                let group = lex::group_content(str, "{", "}")?;
                self.buffer.extend([
                    I::Event(E::Begin(G::Normal)),
                    I::SubGroup { content: group, allowed_alignment_count: None },
                    I::Event(E::End)
                ]);
                return Ok(())
            },
            '}' => {
                return Err(ErrorKind::UnbalancedGroup(None))
            },

            // Special ( ~ = nobreak space)
            // TODO: Make this a `Spacing` event
            '~' => {
                E::Content(C::Text("&nbsp;"))
            },

            '0'..='9' => {
                let content = token.as_str();
                let mut len = content
                    .chars()
                    .skip(1)
                    .take_while(|&c| matches!(c, '.' | ',' | '0'..='9'))
                    .count()
                    + 1;
                if matches!(content.as_bytes()[len - 1], b'.' | b',') {
                    len -= 1;
                }
                let (number, rest) = content.split_at(len);
                self.content = rest;
                self.buffer
                    .push(I::Event(E::Content(C::Number(number))));
                return Ok(())
            }
            // Punctuation
            '.' | ',' | ';' => E::Content(C::Punctuation(token.into())),
            
            '\'' => ordinary('′'),
            '-' => binary('−'),
            '*' => binary('∗'),
            
            c if is_binary(c) => binary(c),
            c if is_relation(c) => relation(c),
                

            c if char_delimiter_map(c).is_some() => {
                let (content, ty) = char_delimiter_map(c).unwrap();
                if ty == DelimiterType::Fence {
                    ordinary(content)
                } else {
                E::Content(C::Delimiter {
                    content,
                    size: None,
                    ty,
                })
                }
            }
            
            c => ordinary(c),
        });
        self.buffer.push(instruction);
        Ok(())
    }

    /// Handle a supported control sequence, pushing instructions to the provided stack.
    pub(super) fn handle_primitive(&mut self, control_sequence: &'a str) -> InnerResult<()> {
        let event = match control_sequence {
            "arccos" | "cos" | "csc" | "exp" | "ker" | "sinh" | "arcsin" | "cosh" | "deg"
            | "lg" | "ln" | "arctan" | "cot" | "det" | "hom" | "log" | "sec" | "tan" | "arg"
            | "coth" | "dim" | "sin" | "tanh" | "sgn" => {
                E::Content(C::Function(control_sequence))
            }
            "lim" | "Pr" | "sup" | "liminf" | "max" | "inf" | "gcd" | "limsup" | "min" => {
                self.state.allow_suffix_modifiers = true;
                self.state.above_below_suffix_default = true;
                E::Content(C::Function(control_sequence))
            }
            "operatorname" => {
                self.state.allow_suffix_modifiers = true;
                let argument = lex::argument(&mut self.content)?;
                match argument {
                    Argument::Token(Token::ControlSequence(_)) => {
                        return Err(ErrorKind::ControlSequenceAsArgument)
                    }
                    Argument::Token(Token::Character(char_)) => {
                        E::Content(C::Function(char_.as_str()))
                    }
                    Argument::Group(content) => {
                        E::Content(C::Function(content))
                    }
                }
            }
            "bmod" => E::Content(C::Function("mod")),
            // TODO: use left right.
            "pmod" => {
                let argument = lex::argument(&mut self.content)?;
                self.buffer.extend([
                    I::Event(E::Begin(G::Normal)),
                    I::Event(E::Content(C::Delimiter {
                        content: '(',
                        size: None,
                        ty: DelimiterType::Open
                    })),
                ]);
                self.handle_argument(argument)?;
                self.buffer.extend([
                    I::Event(E::Content(C::Delimiter {
                     content: ')',
                     size: None,
                     ty: DelimiterType::Close
                    })),   
                    I::Event(E::End),
                ]);
                return Ok(());
            }

            // TODO: Operators with '*', for operatorname* and friends

            /////////////////////////
            // Non-Latin Alphabets //
            /////////////////////////
            // Lowercase Greek letters
            "alpha" => ordinary('α'),
            "beta" => ordinary('β'),
            "gamma" => ordinary('γ'),
            "delta" => ordinary('δ'),
            "epsilon" => ordinary('ϵ'),
            "zeta" => ordinary('ζ'),
            "eta" => ordinary('η'),
            "theta" => ordinary('θ'),
            "iota" => ordinary('ι'),
            "kappa" => ordinary('κ'),
            "lambda" => ordinary('λ'),
            "mu" => ordinary('µ'),
            "nu" => ordinary('ν'),
            "xi" => ordinary('ξ'),
            "pi" => ordinary('π'),
            "rho" => ordinary('ρ'),
            "sigma" => ordinary('σ'),
            "tau" => ordinary('τ'),
            "upsilon" => ordinary('υ'),
            "phi" => ordinary('φ'),
            "chi" => ordinary('χ'),
            "psi" => ordinary('ψ'),
            "omega" => ordinary('ω'),
            "omicron" => ordinary('ο'),
            // Uppercase Greek letters
            "Alpha" => ordinary('Α'),
            "Beta" => ordinary('Β'),
            "Gamma" => ordinary('Γ'),
            "Delta" => ordinary('Δ'),
            "Epsilon" => ordinary('Ε'),
            "Zeta" => ordinary('Ζ'),
            "Eta" => ordinary('Η'),
            "Theta" => ordinary('Θ'),
            "Iota" => ordinary('Ι'),
            "Kappa" => ordinary('Κ'),
            "Lambda" => ordinary('Λ'),
            "Mu" => ordinary('Μ'),
            "Nu" => ordinary('Ν'),
            "Xi" => ordinary('Ξ'),
            "Pi" => ordinary('Π'),
            "Rho" => ordinary('Ρ'),
            "Sigma" => ordinary('Σ'),
            "Tau" => ordinary('Τ'),
            "Upsilon" => ordinary('Υ'),
            "Phi" => ordinary('Φ'),
            "Chi" => ordinary('Χ'),
            "Psi" => ordinary('Ψ'),
            "Omega" => ordinary('Ω'),
            "Omicron" => ordinary('Ο'),
            // Lowercase Greek Variants
            "varepsilon" => ordinary('ε'),
            "vartheta" => ordinary('ϑ'),
            "varkappa" => ordinary('ϰ'),
            "varrho" => ordinary('ϱ'),
            "varsigma" => ordinary('ς'),
            "varpi" => ordinary('ϖ'),
            "varphi" => ordinary('ϕ'),
            // Uppercase Greek Variants
            "varGamma" => ordinary('𝛤'),
            "varDelta" => ordinary('𝛥'),
            "varTheta" => ordinary('𝛩'),
            "varLambda" => ordinary('𝛬'),
            "varXi" => ordinary('𝛯'),
            "varPi" => ordinary('𝛱'),
            "varSigma" => ordinary('𝛴'),
            "varUpsilon" => ordinary('𝛶'),
            "varPhi" => ordinary('𝛷'),
            "varPsi" => ordinary('𝛹'),
            "varOmega" => ordinary('𝛺'),

            // Hebrew letters
            "aleph" => ordinary('ℵ'),
            "beth" => ordinary('ℶ'),
            "gimel" => ordinary('ℷ'),
            "daleth" => ordinary('ℸ'),
            // Other symbols
            "digamma" => ordinary('ϝ'),
            "eth" => ordinary('ð'),
            "ell" => ordinary('ℓ'),
            "nabla" => ordinary('∇'),
            "partial" => ordinary('∂'),
            "Finv" => ordinary('Ⅎ'),
            "Game" => ordinary('ℷ'),
            "hbar" | "hslash" => ordinary('ℏ'),
            "imath" => ordinary('ı'),
            "jmath" => ordinary('ȷ'),
            "Im" => ordinary('ℑ'),
            "Re" => ordinary('ℜ'),
            "wp" => ordinary('℘'),
            "Bbbk" => ordinary('𝕜'),
            "Angstrom" => ordinary('Å'),
            "backepsilon" => ordinary('϶'),

            ///////////////////////////
            // Symbols & Punctuation //
            ///////////////////////////
            "dots" => if self.content.trim_start().starts_with(['.', ',']) {
                ordinary('…')
            } else {
                ordinary('⋯')
            }
            "ldots" | "dotso" | "dotsc" => ordinary('…'),
            "cdots" | "dotsi" | "dotsm" | "dotsb" | "idotsin" => ordinary('⋯'),
            "ddots" => ordinary('⋱'),
            "iddots" => ordinary('⋰'),
            "vdots" => ordinary('⋮'),
            "mathellipsis" => ordinary('…'),
            "infty" => ordinary('∞'),
            "checkmark" => ordinary('✓'),
            "ballotx" => ordinary('✗'),
            "dagger" | "dag" => ordinary('†'),
            "ddagger" | "ddag" => ordinary('‡'),
            "angle" => ordinary('∠'),
            "measuredangle" => ordinary('∡'),
            "lq" => ordinary('‘'),
            "Box" => ordinary('□'),
            "sphericalangle" => ordinary('∢'),
            "square" => ordinary('□'),
            "top" => ordinary('⊤'),
            "rq" => ordinary('′'),
            "blacksquare" => ordinary('■'),
            "bot" => ordinary('⊥'),
            "triangledown" => ordinary('▽'),
            "Bot" => ordinary('⫫'),
            "triangleleft" => ordinary('◃'),
            "triangleright" => ordinary('▹'),
            "cent" => ordinary('¢'),
            "colon" | "ratio" | "vcentcolon" => ordinary(':'),
            "bigtriangledown" => ordinary('▽'),
            "pounds" | "mathsterling" => ordinary('£'),
            "bigtriangleup" => ordinary('△'),
            "blacktriangle" => ordinary('▲'),
            "blacktriangledown" => ordinary('▼'),
            "yen" => ordinary('¥'),
            "blacktriangleleft" => ordinary('◀'),
            "euro" => ordinary('€'),
            "blacktriangleright" => ordinary('▶'),
            "Diamond" => ordinary('◊'),
            "degree" => ordinary('°'),
            "lozenge" => ordinary('◊'),
            "blacklozenge" => ordinary('⧫'),
            "mho" => ordinary('℧'),
            "bigstar" => ordinary('★'),
            "diagdown" => ordinary('╲'),
            "maltese" => ordinary('✠'),
            "diagup" => ordinary('╱'),
            "P" => ordinary('¶'),
            "clubsuit" => ordinary('♣'),
            "varclubsuit" => ordinary('♧'),
            "S" => ordinary('§'),
            "diamondsuit" => ordinary('♢'),
            "vardiamondsuit" => ordinary('♦'),
            "copyright" => ordinary('©'),
            "heartsuit" => ordinary('♡'),
            "varheartsuit" => ordinary('♥'),
            "circledR" => ordinary('®'),
            "spadesuit" => ordinary('♠'),
            "varspadesuit" => ordinary('♤'),
            "circledS" => ordinary('Ⓢ'),
            "female" => ordinary('♀'),
            "male" => ordinary('♂'),
            "astrosun" => ordinary('☉'),
            "sun" => ordinary('☼'),
            "leftmoon" => ordinary('☾'),
            "rightmoon" => ordinary('☽'),
            "smiley" => ordinary('☺'),
            "Earth" => ordinary('⊕'),
            "flat" => ordinary('♭'),
            "standardstate" => ordinary('⦵'),
            "natural" => ordinary('♮'),
            "sharp" => ordinary('♯'),
            "permil" => ordinary('‰'),
            "QED" => ordinary('∎'),
            "lightning" => ordinary('↯'),
            "diameter" => ordinary('⌀'),
            "leftouterjoin" => ordinary('⟕'),
            "rightouterjoin" => ordinary('⟖'),
            "concavediamond" => ordinary('⟡'),
            "concavediamondtickleft" => ordinary('⟢'),
            "concavediamondtickright" => ordinary('⟣'),
            "fullouterjoin" => ordinary('⟗'),
            "triangle" | "vartriangle" => ordinary('△'),
            "whitesquaretickleft" => ordinary('⟤'),
            "whitesquaretickright" => ordinary('⟥'),


            ////////////////////////
            // Font state changes //
            ////////////////////////
            // LaTeX native absolute font changes (old behavior a.k.a NFSS 1)
            "bf" => self.font_change(Font::Bold),
            "cal" => self.font_change(Font::Script),
            "it" => self.font_change(Font::Italic),
            "rm" => self.font_change(Font::UpRight),
            "sf" => self.font_change(Font::SansSerif),
            "tt" => self.font_change(Font::Monospace),
            // amsfonts font changes (old behavior a.k.a NFSS 1)
            // unicode-math font changes (old behavior a.k.a NFSS 1)
            // changes, as described in https://mirror.csclub.uwaterloo.ca/CTAN/macros/unicodetex/latex/unicode-math/unicode-math.pdf
            // (section. 3.1)
            "mathbf" | "symbf" | "mathbfup" | "symbfup" | "boldsymbol" => {
                return self.font_group(Some(Font::Bold))
            }
            "mathcal" | "symcal" | "mathup" | "symup" => {
                return self.font_group(Some(Font::Script))
            }
            "mathit" | "symit" => return self.font_group(Some(Font::Italic)),
            "mathrm" | "symrm" => return self.font_group(Some(Font::UpRight)),
            "mathsf" | "symsf" | "mathsfup" | "symsfup" => {
                return self.font_group(Some(Font::SansSerif))
            }
            "mathtt" | "symtt" => return self.font_group(Some(Font::Monospace)),
            "mathbb" | "symbb" => return self.font_group(Some(Font::DoubleStruck)),
            "mathfrak" | "symfrak" => return self.font_group(Some(Font::Fraktur)),
            "mathbfcal" | "symbfcal" => return self.font_group(Some(Font::BoldScript)),
            "mathsfit" | "symsfit" => return self.font_group(Some(Font::SansSerifItalic)),
            "mathbfit" | "symbfit" => return self.font_group(Some(Font::BoldItalic)),
            "mathbffrak" | "symbffrak" => return self.font_group(Some(Font::BoldFraktur)),
            "mathbfsfup" | "symbfsfup" => return self.font_group(Some(Font::BoldSansSerif)),
            "mathbfsfit" | "symbfsfit" => return self.font_group(Some(Font::SansSerifBoldItalic)),
            "mathnormal" | "symnormal" => return self.font_group(None),

            ////////////////////////
            // Style state change //
            ////////////////////////
            "displaystyle" => self.style_change(S::Display),
            "textstyle" => self.style_change(S::Text),
            "scriptstyle" => self.style_change(S::Script),
            "scriptscriptstyle" => self.style_change(S::ScriptScript),

            ////////////////////////
            // Color state change //
            ////////////////////////
            "color" => {
                let Argument::Group(color) =
                    lex::argument(&mut self.content)?
                else {
                    return Err(ErrorKind::Argument);
                };
                self.state.skip_suffixes = true;
                
                if !is_primitive_color(color) {
                    return Err(ErrorKind::UnknownColor);
                }
                E::StateChange(SC::Color(CC {
                    color,
                    target: CT::Text,
                }))
            },
            "textcolor" => {
                let str = &mut self.content;
                let Argument::Group(color) =
                    lex::argument(str)?
                else {
                    return Err(ErrorKind::Argument);
                };
                
                if !is_primitive_color(color) {
                    return Err(ErrorKind::UnknownColor);
                }
                let modified = lex::argument(str)?;

                self.buffer.extend([I::Event(E::Begin(G::Normal)), I::Event(E::StateChange(SC::Color(CC {
                    color,
                    target: CT::Text,
                })))]);
                self.handle_argument(modified)?;
                E::End
            }
            "colorbox" => {
                let Argument::Group(color) =
                    lex::argument(&mut self.content)?
                else {
                    return Err(ErrorKind::Argument);
                };
                if !is_primitive_color(color) {
                    return Err(ErrorKind::UnknownColor);
                }
                self.buffer.extend([I::Event(E::Begin(G::Normal)), I::Event(E::StateChange(SC::Color(CC {
                    color,
                    target: CT::Background,
                })))]);
                self.text_argument()?;
                E::End
            }
            "fcolorbox" => {
                let str = &mut self.content;
                let Argument::Group(frame_color) =
                    lex::argument(str)?
                else {
                    return Err(ErrorKind::Argument);
                };
                let Argument::Group(background_color) =
                    lex::argument(str)?
                else {
                    return Err(ErrorKind::Argument);
                };
                if !is_primitive_color(frame_color) || !is_primitive_color(background_color) {
                    return Err(ErrorKind::UnknownColor);
                }
                self.buffer.extend([I::Event(E::Begin(G::Normal)), I::Event(E::StateChange(SC::Color(CC {
                    color: frame_color,
                    target: CT::Text,
                }))), I::Event(E::StateChange(SC::Color(CC {
                    color: background_color,
                    target: CT::Background,
                })))]);
                self.text_argument()?;
                E::End
            },

            ///////////////////////////////
            // Delimiters size modifiers //
            ///////////////////////////////
            // Sizes taken from `texzilla`
            // Big left and right seem to not care about which delimiter is used. i.e., \bigl) and \bigr) are the same.
            "big" | "bigl" | "bigr" | "bigm" => return self.sized_delim(DelimiterSize::Big),
            "Big" | "Bigl" | "Bigr" | "Bigm" => return self.sized_delim(DelimiterSize::BIG),
            "bigg" | "biggl" | "biggr" | "biggm" => return self.sized_delim(DelimiterSize::Bigg),
            "Bigg" | "Biggl" | "Biggr" | "Biggm" => return self.sized_delim(DelimiterSize::BIGG),

            "left" => {
                let curr_str = &mut self.content;
                let opening = if let Some(rest) = curr_str.strip_prefix('.') {
                    *curr_str = rest;
                    None
                } else {
                    Some(lex::delimiter(curr_str)?.0)
                };

                let curr_str = &mut self.content;
                let group_content = lex::group_content(curr_str, r"\left", r"\right")?;
                let closing = if let Some(rest) = curr_str.strip_prefix('.') {
                    *curr_str = rest;
                    None
                } else {
                    Some(lex::delimiter(curr_str)?.0)
                };

                self.buffer.extend([
                    I::Event(E::Begin(G::LeftRight(opening, closing))),
                    I::SubGroup { content: group_content, allowed_alignment_count: None },
                    I::Event(E::End),
                ]);

                return Ok(());
            }
            // TODO: Check the conditions for this op. Does it need to be
            // within a left-right group?
            "middle" => {
                let delimiter = lex::delimiter(&mut self.content)?;
                E::Content(C::Delimiter {
                    content: delimiter.0,
                    size: Some(DelimiterSize::Big),
                    ty: DelimiterType::Fence,
                })
            }
            "right" => {
                return Err(ErrorKind::UnbalancedGroup(None));
            }

            ///////////////////
            // Big Operators //
            ///////////////////
            // NOTE: All of the following operators allow limit modifiers.
            // The following operators have above and below limits by default.
            "sum" => self.large_op('∑', true),
            "prod" => self.large_op('∏', true),
            "coprod" => self.large_op('∐', true),
            "bigvee" => self.large_op('⋁', true),
            "bigwedge" => self.large_op('⋀', true),
            "bigcup" => self.large_op('⋃', true),
            "bigcap" => self.large_op('⋂', true),
            "biguplus" => self.large_op('⨄', true),
            "bigoplus" => self.large_op('⨁', true),
            "bigotimes" => self.large_op('⨂', true),
            "bigodot" => self.large_op('⨀', true),
            "bigsqcup" => self.large_op('⨆', true),
            "bigsqcap" => self.large_op('⨅', true),
            "bigtimes" => self.large_op('⨉', true),
            "intop" => self.large_op('∫', true),
            // The following operators do not have above and below limits by default.
            "int" => self.large_op('∫', false),
            "iint" => self.large_op('∬', false),
            "iiint" => self.large_op('∭', false),
            "smallint" => {
                self.state.allow_suffix_modifiers = true;
                E::Content(C::LargeOp { content: '∫', small: true })
            }
            "iiiint" => self.large_op('⨌', false),
            "intcap" => self.large_op('⨙', false),
            "intcup" => self.large_op('⨚', false),
            "oint" => self.large_op('∮', false),
            "varointclockwise" => self.large_op('∲', false),
            "intclockwise" => self.large_op('∱', false),
            "oiint" => self.large_op('∯', false),
            "pointint" => self.large_op('⨕', false),
            "rppolint" => self.large_op('⨒', false),
            "scpolint" => self.large_op('⨓', false),
            "oiiint" => self.large_op('∰', false),
            "intlarhk" => self.large_op('⨗', false),
            "sqint" => self.large_op('⨖', false),
            "intx" => self.large_op('⨘', false),
            "intbar" => self.large_op('⨍', false),
            "intBar" => self.large_op('⨎', false),
            "fint" => self.large_op('⨏', false),

            /////////////
            // Accents //
            /////////////
            "acute" => return self.accent('´', false),
            "bar" | "overline" => return self.accent('‾', false),
            "underbar" | "underline" => return self.underscript('_'),
            "breve" => return self.accent('˘', false),
            "check" => return self.accent('ˇ', false),
            "dot" => return self.accent('˙', false),
            "ddot" => return self.accent('¨', false),
            "grave" => return self.accent('`', false),
            "hat" => return self.accent('^', false),
            "tilde" => return self.accent('~', false),
            "vec" => return self.accent('→', false),
            "mathring" => return self.accent('˚', false),

            // Arrows
            "overleftarrow" => return self.accent('←', true),
            "underleftarrow" => return self.underscript('←'),
            "overrightarrow" => return self.accent('→', true),
            "Overrightarrow" => return self.accent('⇒', true),
            "underrightarrow" => return self.underscript('→'),
            "overleftrightarrow" => return self.accent('↔', true),
            "underleftrightarrow" => return self.underscript('↔'),
            "overleftharpoon" => return self.accent('↼', true),
            "overrightharpoon" => return self.accent('⇀', true),

            // Wide ops
            "widecheck" => return self.accent('ˇ', true),
            "widehat" => return self.accent('^', true),
            "widetilde" => return self.accent('~', true),
            "wideparen" | "overparen" => return self.accent('⏜', true),

            // Groups
            "overgroup" => return self.accent('⏠', true),
            "undergroup" => return self.underscript('⏡'),
            "overbrace" => return self.accent('⏞', true),
            "underbrace" => return self.underscript('⏟'),
            "underparen" => return self.underscript('⏝'),

            // Primes
            "prime" => ordinary('′'),
            "dprime" => ordinary('″'),
            "trprime" => ordinary('‴'),
            "qprime" => ordinary('⁗'),
            "backprime" => ordinary('‵'),
            "backdprime" => ordinary('‶'),
            "backtrprime" => ordinary('‷'),

            /////////////
            // Spacing //
            /////////////
            "," | "thinspace" => E::Space {
                width: Some((3. / 18., DimensionUnit::Em)),
                height: None,
                depth: None,
            },
            ">" | ":" | "medspace" => E::Space {
                width: Some((4. / 18., DimensionUnit::Em)),
                height: None,
                depth: None,
            },
            ";" | "thickspace" => E::Space {
                width: Some((5. / 18., DimensionUnit::Em)),
                height: None,
                depth: None,
            },
            "enspace" => E::Space {
                width: Some((0.5, DimensionUnit::Em)),
                height: None,
                depth: None,
            },
            "quad" => E::Space {
                width: Some((1., DimensionUnit::Em)),
                height: None,
                depth: None,
            },
            "qquad" => E::Space {
                width: Some((2., DimensionUnit::Em)),
                height: None,
                depth: None,
            },
            "mathstrut" => E::Space {
                width: None,
                height: Some((0.7, DimensionUnit::Em)),
                depth: Some((0.3, DimensionUnit::Em)),
            },
            "~" | "nobreakspace" => E::Content(C::Text("&nbsp;")),
            // Variable spacing
            "kern" => {
                let dimension = lex::dimension(&mut self.content)?;
                E::Space {
                    width: Some(dimension),
                    height: None,
                    depth: None,
                }
            }
            "hskip" => {
                let glue = lex::glue(&mut self.content)?;
                E::Space {
                    width: Some(glue.0),
                    height: None,
                    depth: None,
                }
            }
            "mkern" => {
                let dimension =
                    lex::dimension(&mut self.content)?;
                if dimension.1 == DimensionUnit::Mu {
                    E::Space {
                        width: Some(dimension),
                        height: None,
                        depth: None,
                    }
                } else {
                    return Err(ErrorKind::MathUnit);
                }
            }
            "mskip" => {
                let glue = lex::glue(&mut self.content)?;
                if glue.0.1 == DimensionUnit::Mu
                    && glue.1.map_or(true, |(_, unit)| unit == DimensionUnit::Mu)
                    && glue.2.map_or(true, |(_, unit)| unit == DimensionUnit::Mu) {
                    E::Space {
                        width: Some(glue.0),
                        height: None,
                        depth: None,
                    }
                } else {
                    return Err(ErrorKind::MathUnit);
                }
            }
            "hspace" => {
                let Argument::Group(mut argument) =
                    lex::argument(&mut self.content)?
                else {
                    return Err(ErrorKind::DimensionArgument);
                };
                let glue = lex::glue(&mut argument)?;
                E::Space {
                    width: Some(glue.0),
                    height: None,
                    depth: None,
                }
            }
            // Negative spacing
            "!" | "negthinspace" => E::Space {
                width: Some((-3. / 18., DimensionUnit::Em)),
                height: None,
                depth: None,
            },
            "negmedspace" => E::Space {
                width: Some((-4. / 18., DimensionUnit::Em)),
                height: None,
                depth: None,
            },
            "negthickspace" => E::Space {
                width: Some((-5. / 18., DimensionUnit::Em)),
                height: None,
                depth: None,
            },

            ////////////////////////
            // Logic & Set Theory //
            ////////////////////////
            "forall" => ordinary('∀'),
            "exists" => ordinary('∃'),
            "complement" => ordinary('∁'),
            "nexists" => ordinary('∄'),
            "neg" | "lnot" => ordinary('¬'),
            
            "therefore" => relation('∴'),
            "because" => relation('∵'),
            "subset" => relation('⊂'),
            "supset" => relation('⊃'),
            "strictif" => relation('⥽'),
            "strictfi" => relation('⥼'),
            "mapsto" => relation('↦'),
            "implies" => relation('⟹'),
            "mid" => relation('∣'),
            "to" => relation('→'),
            "impliedby" => relation('⟸'),
            "in" | "isin" => relation('∈'),
            "ni" => relation('∋'),
            "gets" => relation('←'),
            "iff" => relation('⟺'),
            "notni" => relation('∌'),
            
            "land" => binary('∧'),
            
            "emptyset" => ordinary('∅'),
            "varnothing" => ordinary('⌀'),

            //////////////////////
            // Binary Operators //
            //////////////////////
            "ldotp" => binary('.'),
            "cdotp" => binary('·'),
            "cdot" => binary('⋅'),
            "centerdot" => binary('·'),
            "circ" => binary('∘'),
            "bullet" => binary('∙'),
            "circledast" => binary('⊛'),
            "circledcirc" => binary('⊚'),
            "circleddash" => binary('⊝'),
            "bigcirc" => binary('◯'),
            "leftthreetimes" => binary('⋋'),
            "rhd" => binary('⊳'),
            "lhd" => binary('⊲'),
            "rightthreetimes" => binary('⋌'),
            "rtimes" => binary('⋊'),
            "ltimes" => binary('⋉'),
            "leftmodels" => binary('⊨'),
            "amalg" => binary('⨿'),
            "ast" => binary('*'),
            "asymp" => binary('≍'),
            "And" | "with" => binary('&'),
            "lor" => binary('∨'),
            "setminus" => binary('∖'),
            "Cup" => binary('⋓'),
            "cup" => binary('∪'),
            "sqcup" => binary('⊔'),
            "sqcap" => binary('⊓'),
            "lessdot" => binary('⋖'),
            "smallsetminus" => E::Content(C::BinaryOp { content: '∖', small: false }),
            "barwedge" => binary('⌅'),
            "curlyvee" => binary('⋎'),
            "curlywedge" => binary('⋏'),
            "sslash" => binary('⫽'),
            "div" => binary('÷'),
            "mp" => binary('∓'),
            "times" => binary('×'),
            "boxdot" => binary('⊡'),
            "divideontimes" => binary('⋇'),
            "odot" => binary('⊙'),
            "unlhd" => binary('⊴'),
            "boxminus" => binary('⊟'),
            "dotplus" => binary('∔'),
            "ominus" => binary('⊖'),
            "unrhd" => binary('⊵'),
            "boxplus" => binary('⊞'),
            "doublebarwedge" => binary('⩞'),
            "oplus" => binary('⊕'),
            "uplus" => binary('⊎'),
            "boxtimes" => binary('⊠'),
            "doublecap" => binary('⋒'),
            "otimes" => binary('⊗'),
            "vee" => binary('∨'),
            "veebar" => binary('⊻'),
            "Cap" => binary('⋒'),
            "parr" => binary('⅋'),
            "wedge" => binary('∧'),
            "cap" => binary('∩'),
            "gtrdot" => binary('⋗'),
            "pm" => binary('±'),
            "intercal" => binary('⊺'),
            "wr" => binary('≀'),
            "circledvert" => binary('⦶'),
            "blackhourglass" => binary('⧗'),
            "circlehbar" => binary('⦵'),
            "operp" => binary('⦹'),
            "boxast" => binary('⧆'),
            "boxbox" => binary('⧈'),
            "oslash" => binary('⊘'),
            "boxcircle" => binary('⧇'),
            "diamond" => binary('⋄'),
            "Otimes" => binary('⨷'),
            "hourglass" => binary('⧖'),
            "otimeshat" => binary('⨶'),
            "triangletimes" => binary('⨻'),
            "lozengeminus" => binary('⟠'),
            "star" => binary('⋆'),
            "obar" => binary('⌽'),
            "obslash" => binary('⦸'),
            "triangleminus" => binary('⨺'),
            "odiv" => binary('⨸'),
            "triangleplus" => binary('⨹'),
            "circledequal" => binary('⊜'),
            "ogreaterthan" => binary('⧁'),
            "circledparallel" => binary('⦷'),
            "olessthan" => binary('⧀'),

            ///////////////
            // Relations //
            ///////////////
            "eqcirc" => relation('≖'),
            "lessgtr" => relation('≶'),
            "smile" | "sincoh" => relation('⌣'),
            "eqcolon" | "minuscolon" => relation('∹'),
            "lesssim" => relation('≲'),
            "sqsubset" => relation('⊏'),
            "ll" => relation('≪'),
            "sqsubseteq" => relation('⊑'),
            "eqqcolon" => relation('≕'),
            "lll" => relation('⋘'),
            "sqsupset" => relation('⊐'),
            "llless" => relation('⋘'),
            "sqsupseteq" => relation('⊒'),
            "approx" => relation('≈'),
            "eqdef" => relation('≝'),
            "lt" => relation('<'),
            "stareq" => relation('≛'),
            "approxeq" => relation('≊'),
            "eqsim" => relation('≂'),
            "measeq" => relation('≞'),
            "Subset" => relation('⋐'),
            "arceq" => relation('≘'),
            "eqslantgtr" => relation('⪖'),
            "eqslantless" => relation('⪕'),
            "models" => relation('⊨'),
            "subseteq" => relation('⊆'),
            "backcong" => relation('≌'),
            "equiv" => relation('≡'),
            "multimap" => relation('⊸'),
            "subseteqq" => relation('⫅'),
            "fallingdotseq" => relation('≒'),
            "multimapboth" => relation('⧟'),
            "succ" => relation('≻'),
            "backsim" => relation('∽'),
            "frown" => relation('⌢'),
            "multimapinv" => relation('⟜'),
            "succapprox" => relation('⪸'),
            "backsimeq" => relation('⋍'),
            "ge" => relation('≥'),
            "origof" => relation('⊶'),
            "succcurlyeq" => relation('≽'),
            "between" => relation('≬'),
            "geq" => relation('≥'),
            "owns" => relation('∋'),
            "succeq" => relation('⪰'),
            "bumpeq" => relation('≏'),
            "geqq" => relation('≧'),
            "parallel" => relation('∥'),
            "succsim" => relation('≿'),
            "Bumpeq" => relation('≎'),
            "geqslant" => relation('⩾'),
            "perp" => relation('⟂'),
            "Supset" => relation('⋑'),
            "circeq" => relation('≗'),
            "gg" => relation('≫'),
            "Perp" => relation('⫫'),
            "coh" => relation('⌢'),
            "ggg" => relation('⋙'),
            "pitchfork" => relation('⋔'),
            "supseteq" => relation('⊇'),
            "gggtr" => relation('⋙'),
            "prec" => relation('≺'),
            "supseteqq" => relation('⫆'),
            "gt" => relation('>'),
            "precapprox" => relation('⪷'),
            "thickapprox" => relation('≈'),
            "gtrapprox" => relation('⪆'),
            "preccurlyeq" => relation('≼'),
            "thicksim" => relation('∼'),
            "gtreqless" => relation('⋛'),
            "preceq" => relation('⪯'),
            "trianglelefteq" => relation('⊴'),
            "coloneqq" | "colonequals" => relation('≔'),
            "gtreqqless" => relation('⪌'),
            "precsim" => relation('≾'),
            "triangleq" => relation('≜'),
            "Coloneqq" | "coloncolonequals" => relation('⩴'),
            "gtrless" => relation('≷'),
            "propto" => relation('∝'),
            "trianglerighteq" => relation('⊵'),
            "gtrsim" => relation('≳'),
            "questeq" => relation('≟'),
            "varpropto" => relation('∝'),
            "imageof" => relation('⊷'),
            "cong" => relation('≅'),
            "risingdotseq" => relation('≓'),
            "vartriangleleft" => relation('⊲'),
            "curlyeqprec" => relation('⋞'),
            "scoh" => relation('⌢'),
            "vartriangleright" => relation('⊳'),
            "curlyeqsucc" => relation('⋟'),
            "le" => relation('≤'),
            "shortmid" => E::Content(C::Relation { content: '∣', small: true }),
            "shortparallel" => E::Content(C::Relation { content: '∥', small: true }),
            "vdash" => relation('⊢'),
            "dashv" => relation('⊣'),
            "leq" => relation('≤'),
            "vDash" => relation('⊨'),
            "dblcolon" | "coloncolon" => relation('∷'),
            "leqq" => relation('≦'),
            "sim" => relation('∼'),
            "Vdash" => relation('⊩'),
            "doteq" => relation('≐'),
            "leqslant" => relation('⩽'),
            "simeq" => relation('≃'),
            "Dash" => relation('⊫'),
            "Doteq" => relation('≑'),
            "lessapprox" => relation('⪅'),
            "Vvdash" => relation('⊪'),
            "doteqdot" => relation('≑'),
            "lesseqgtr" => relation('⋚'),
            "smallfrown" => relation('⌢'),
            "veeeq" => relation('≚'),
            "eqeq" => relation('⩵'),
            "lesseqqgtr" => relation('⪋'),
            "smallsmile" => E::Content(C::Relation { content: '⌣', small: true }),
            "wedgeq" => relation('≙'),
            "bowtie" | "Join" => relation('⋈'),
            // Negated relations
            "gnapprox" => relation('⪊'),
            "ngeqslant" => relation('≱'),
            "nsubset" => relation('⊄'),
            "nVdash" => relation('⊮'),
            "gneq" => relation('⪈'),
            "ngtr" => relation('≯'),
            "nsubseteq" => relation('⊈'),
            "precnapprox" => relation('⪹'),
            "gneqq" => relation('≩'),
            "nleq" => relation('≰'),
            "nsubseteqq" => relation('⊈'),
            "precneqq" => relation('⪵'),
            "gnsim" => relation('⋧'),
            "nleqq" => relation('≰'),
            "nsucc" => relation('⊁'),
            "precnsim" => relation('⋨'),
            "nleqslant" => relation('≰'),
            "nsucceq" => relation('⋡'),
            "subsetneq" => relation('⊊'),
            "lnapprox" => relation('⪉'),
            "nless" => relation('≮'),
            "nsupset" => relation('⊅'),
            "subsetneqq" => relation('⫋'),
            "lneq" => relation('⪇'),
            "nmid" => relation('∤'),
            "nsupseteq" => relation('⊉'),
            "succnapprox" => relation('⪺'),
            "lneqq" => relation('≨'),
            "notin" => relation('∉'),
            "nsupseteqq" => relation('⊉'),
            "succneqq" => relation('⪶'),
            "lnsim" => relation('⋦'),
            "ntriangleleft" => relation('⋪'),
            "succnsim" => relation('⋩'),
            "nparallel" => relation('∦'),
            "ntrianglelefteq" => relation('⋬'),
            "supsetneq" => relation('⊋'),
            "ncong" => relation('≆'),
            "nprec" => relation('⊀'),
            "ntriangleright" => relation('⋫'),
            "supsetneqq" => relation('⫌'),
            "ne" => relation('≠'),
            "npreceq" => relation('⋠'),
            "ntrianglerighteq" => relation('⋭'),
            "neq" => relation('≠'),
            "nshortmid" => relation('∤'),
            "nvdash" => relation('⊬'),
            "ngeq" => relation('≱'),
            "nshortparallel" => E::Content(C::Relation { content: '∦', small: true }),
            "nvDash" => relation('⊭'),
            "ngeqq" => relation('≱'),
            "nsim" => relation('≁'),
            "nVDash" => relation('⊯'),
            "varsupsetneqq" => E::Content(C::MultiRelation("⫌︀")),
            "varsubsetneqq" => E::Content(C::MultiRelation("⫋︀")),
            "varsubsetneq" => E::Content(C::MultiRelation("⊊︀")),
            "varsupsetneq" => E::Content(C::MultiRelation("⊋︀")),
            "gvertneqq" => E::Content(C::MultiRelation("≩︀")),
            "lvertneqq" => E::Content(C::MultiRelation("≨︀")),
            "Eqcolon" | "minuscoloncolon" => E::Content(C::MultiRelation("−∷")),
            "Eqqcolon" => E::Content(C::MultiRelation("=∷")),
            "approxcolon" => E::Content(C::MultiRelation("≈:")),
            "colonapprox" => E::Content(C::MultiRelation(":≈")),
            "approxcoloncolon" => E::Content(C::MultiRelation("≈∷")),
            "Colonapprox" | "coloncolonapprox" => E::Content(C::MultiRelation("∷≈")),
            "coloneq" | "colonminus" => E::Content(C::MultiRelation(":−")),
            "Coloneq" | "coloncolonminus" => E::Content(C::MultiRelation("∷−")),
            "colonsim" => E::Content(C::MultiRelation(":∼")),
            "Colonsim" | "coloncolonsim" => E::Content(C::MultiRelation("∷∼")),

            ////////////
            // Arrows //
            ////////////
            "circlearrowleft" => relation('↺'),
            "Leftrightarrow" => relation('⇔'),
            "restriction" => relation('↾'),
            "circlearrowright" => relation('↻'),
            "leftrightarrows" => relation('⇆'),
            "rightarrow" => relation('→'),
            "curvearrowleft" => relation('↶'),
            "leftrightharpoons" => relation('⇋'),
            "Rightarrow" => relation('⇒'),
            "curvearrowright" => relation('↷'),
            "leftrightsquigarrow" => relation('↭'),
            "rightarrowtail" => relation('↣'),
            "dashleftarrow" => relation('⇠'),
            "Lleftarrow" => relation('⇚'),
            "rightharpoondown" => relation('⇁'),
            "dashrightarrow" => relation('⇢'),
            "longleftarrow" => relation('⟵'),
            "rightharpoonup" => relation('⇀'),
            "downarrow" => relation('↓'),
            "Longleftarrow" => relation('⟸'),
            "rightleftarrows" => relation('⇄'),
            "Downarrow" => relation('⇓'),
            "longleftrightarrow" => relation('⟷'),
            "rightleftharpoons" => relation('⇌'),
            "downdownarrows" => relation('⇊'),
            "Longleftrightarrow" => relation('⟺'),
            "rightrightarrows" => relation('⇉'),
            "downharpoonleft" => relation('⇃'),
            "longmapsto" => relation('⟼'),
            "rightsquigarrow" => relation('⇝'),
            "downharpoonright" => relation('⇂'),
            "longrightarrow" => relation('⟶'),
            "Rrightarrow" => relation('⇛'),
            "Longrightarrow" => relation('⟹'),
            "Rsh" => relation('↱'),
            "hookleftarrow" => relation('↩'),
            "looparrowleft" => relation('↫'),
            "searrow" => relation('↘'),
            "hookrightarrow" => relation('↪'),
            "looparrowright" => relation('↬'),
            "swarrow" => relation('↙'),
            "Lsh" => relation('↰'),
            "mapsfrom" => relation('↤'),
            "twoheadleftarrow" => relation('↞'),
            "twoheadrightarrow" => relation('↠'),
            "leadsto" => relation('⇝'),
            "nearrow" => relation('↗'),
            "uparrow" => relation('↑'),
            "leftarrow" => relation('←'),
            "nleftarrow" => relation('↚'),
            "Uparrow" => relation('⇑'),
            "Leftarrow" => relation('⇐'),
            "nLeftarrow" => relation('⇍'),
            "updownarrow" => relation('↕'),
            "leftarrowtail" => relation('↢'),
            "nleftrightarrow" => relation('↮'),
            "Updownarrow" => relation('⇕'),
            "leftharpoondown" => relation('↽'),
            "nLeftrightarrow" => relation('⇎'),
            "upharpoonleft" => relation('↿'),
            "leftharpoonup" => relation('↼'),
            "nrightarrow" => relation('↛'),
            "upharpoonright" => relation('↾'),
            "leftleftarrows" => relation('⇇'),
            "nRightarrow" => relation('⇏'),
            "upuparrows" => relation('⇈'),
            "leftrightarrow" => relation('↔'),
            "nwarrow" => relation('↖'),
            "xleftarrow" => {
                let above = lex::argument(&mut self.content)?;
                self.buffer.extend([
                   I::Event(E::Script {
                        ty: ST::Superscript,
                        position: SP::AboveBelow,
                    }),
                    I::Event(relation('⟵'))
                ]);
                self.handle_argument(above)?;
                return Ok(());
            },
            "xrightarrow" => {
                let above = lex::argument(&mut self.content)?;
                self.buffer.extend([
                   I::Event(E::Script {
                        ty: ST::Superscript,
                        position: SP::AboveBelow,
                    }),
                    I::Event(relation('⟶'))
                ]);
                self.handle_argument(above)?;
                return Ok(());
            },

            ///////////////
            // Fractions //
            ///////////////
            "frac" => {
                return self.fraction_like(None, None, None, None);
            }
            // TODO: better errors for this
            "genfrac" => {
                let str = &mut self.content;
                let ldelim_argument = lex::argument(str)?;
                let ldelim = match ldelim_argument {
                    Argument::Token(token) => Some(token_to_delim(token).ok_or(ErrorKind::Delimiter)?),
                    Argument::Group(group) => if group.is_empty() {
                        None
                    } else {
                        return Err(ErrorKind::Delimiter);
                    },
                };
                let rdelim_argument = lex::argument(str)?;
                let rdelim = match rdelim_argument {
                    Argument::Token(token) => Some(token_to_delim(token).ok_or(ErrorKind::Delimiter)?),
                    Argument::Group(group) => if group.is_empty() {
                        None
                    } else {
                        return Err(ErrorKind::Delimiter);
                    },
                };
                let bar_size_argument = lex::argument(str)?;
                let bar_size = match bar_size_argument {
                    Argument::Token(_) => return Err(ErrorKind::DimensionArgument),
                    Argument::Group("") => None,
                    Argument::Group(mut group) => lex::dimension(&mut group).and_then(|dim| {
                        if group.is_empty() {
                            Ok(Some(dim))
                        } else {
                            Err(ErrorKind::DimensionArgument)
                        }
                    })?,
                };
                let display_style_argument = lex::argument(str)?;
                let display_style = match display_style_argument {
                    Argument::Token(t) => match t {
                            Token::ControlSequence(_) => return Err(ErrorKind::Argument),
                            Token::Character(c) => Some(match c.into() {
                                '0' => S::Display,
                                '1' => S::Text,
                                '2' => S::Script,
                                '3' => S::ScriptScript,
                                _ => return Err(ErrorKind::Argument),
                            }),
                    },
                    Argument::Group(group) => {
                        match group {
                            "0" => Some(S::Display),
                            "1" => Some(S::Text),
                            "2" => Some(S::Script),
                            "3" => Some(S::ScriptScript),
                            "" => None,
                            _ => return Err(ErrorKind::Argument),
                        }
                    }
                };

                self.fraction_like(ldelim.map(|d| d.0), rdelim.map(|d| d.0), bar_size, display_style)?;
                
                return Ok(())
            }
            "cfrac" | "dfrac" => {
                self.fraction_like(None, None, None, Some(S::Display))?;
                return Ok(())
            }
            "tfrac" => {
                self.fraction_like(None, None, None, Some(S::Text))?;
                return Ok(())
            }
            "binom" => {
                self.fraction_like(Some('('), Some(')'), Some((0., DimensionUnit::Em)), None)?;
                return Ok(())
            }
            "dbinom" => {
                self.fraction_like(Some('('), Some(')'), Some((0., DimensionUnit::Em)), Some(S::Display))?;
                return Ok(())
            }
            "tbinom" => {
                self.fraction_like(Some('('), Some(')'), Some((0., DimensionUnit::Em)), Some(S::Text))?;
                return Ok(())
            }
            "overset" | "stackrel" => {
                self.buffer.push(I::Event(E::Script {
                    ty: ST::Superscript,
                    position: SP::AboveBelow,
                }));
                let before_over_index = self.buffer.len();
                let over = lex::argument(&mut self.content)?;
                self.handle_argument(over)?;
                let over_events = self.buffer.split_off(before_over_index);
                let base = lex::argument(&mut self.content)?;
                self.handle_argument(base)?;
                self.buffer.extend(over_events);
                return Ok(());
            }
            "underset" => {
                self.buffer.push(I::Event(E::Script {
                    ty: ST::Subscript,
                    position: SP::AboveBelow,
                }));
                let before_under_index = self.buffer.len();
                let under = lex::argument(&mut self.content)?;
                self.handle_argument(under)?;
                let under_events = self.buffer.split_off(before_under_index);
                let base = lex::argument(&mut self.content)?;
                self.handle_argument(base)?;
                self.buffer.extend(under_events);
                return Ok(());
            }

            //////////////
            // Radicals //
            //////////////
            "sqrt" => {
                if let Some(index) =
                    lex::optional_argument(&mut self.content)?
                {
                    self.buffer
                        .push(I::Event(E::Visual(V::Root)));
                    let arg = lex::argument(&mut self.content)?;
                    self.handle_argument(arg)?;
                    self.buffer.push(I::SubGroup {
                        content: index,
                        allowed_alignment_count: None,
                    });
                } else {
                    self.buffer
                        .push(I::Event(E::Visual(V::SquareRoot)));
                    let arg = lex::argument(&mut self.content)?;
                    self.handle_argument(arg)?;
                }
                return Ok(());
            }
            "surd" => {
                self.buffer.extend([
                    I::Event(E::Visual(V::SquareRoot)),
                    I::Event(E::Space {
                        width: Some((0., DimensionUnit::Em)),
                        height: Some((0.7, DimensionUnit::Em)),
                        depth: None,
                    }),
                ]);
                return Ok(());
            }

            "backslash" => ordinary('\\'),

            ///////////////////
            // Miscellaneous //
            ///////////////////
            "#" | "%" | "&" | "$" | "_" => ordinary(
                control_sequence
                    .chars()
                    .next()
                    .expect("the control sequence contains one of the matched characters"),
            ),
            "|" => ordinary('∥'),
            "text" => return self.text_argument(),
            // TODO: should cancel be its own event?
            "not" | "cancel" => {
                self.buffer
                    .push(I::Event(E::Visual(V::Negation)));
                let argument = lex::argument(&mut self.content)?;
                self.handle_argument(argument)?;
                return Ok(());
            }
            "char" => {
                let number = lex::unsigned_integer(&mut self.content)?;
                if number > 255 {
                    return Err(ErrorKind::InvalidCharNumber);
                }
                E::Content(C::Ordinary {
                    content: char::from_u32(number as u32).expect("the number is a valid char since it is less than 256"),
                    stretchy: false,
                })
            },
            "relax" => {
                return if self.state.handling_argument {
                    Err(ErrorKind::Relax)
                } else {
                    Ok(())
                }
            }

            "begingroup" => {
                let group = lex::group_content(&mut self.content, "begingroup", "endgroup")?;
                self.buffer.extend([
                    I::Event(E::Begin(G::Normal)),
                    I::SubGroup { content: group, allowed_alignment_count: None },
                    I::Event(E::End),
                ]);
                return Ok(());
            }
            "endgroup" => return Err(ErrorKind::UnbalancedGroup(None)),

            // TODO: ensure claims that the alignment and newlines event are generated __only__
            // when it is allowed to.
            "begin" => {
                let Argument::Group(argument) = lex::argument(&mut self.content)? else {
                    return Err(ErrorKind::Argument);
                };

                let mut style = None;

                let (environment, wrap, align_count) = match argument {
                    "array" => {
                        let (grouping, count) = self.array_environment()?;
                        (grouping, None, count)
                    },
                    "darray" => {
                        style = Some(S::Display);
                        let (grouping, count) = self.array_environment()?;
                        (grouping, None, count)
                    },
                    "matrix" => (G::Matrix {
                        alignment: ColumnAlignment::Center,
                    }, None, u16::MAX),
                    "matrix*" => {
                        (G::Matrix {
                            alignment: self.optional_alignment()?.unwrap_or(ColumnAlignment::Center)
                        }, None, u16::MAX)
                    },
                    "smallmatrix" => {
                        style = Some(S::Text);
                        (G::Matrix {
                            alignment: ColumnAlignment::Center,
                        }, None, u16::MAX)
                    }
                    "pmatrix" => {
                        (G::Matrix {
                            alignment: ColumnAlignment::Center,
                        }, Some(G::LeftRight(Some('('), Some(')'))), u16::MAX)
                    },
                    "pmatrix*" => {
                        (G::Matrix {
                            alignment: self.optional_alignment()?.unwrap_or(ColumnAlignment::Center),
                        }, Some(G::LeftRight(Some('('), Some(')'))), u16::MAX)
                    },
                    "bmatrix" => {
                        (G::Matrix {
                            alignment: ColumnAlignment::Center,
                        }, Some(G::LeftRight(Some('['), Some(']'))), u16::MAX)
                    },
                    "bmatrix*" => {
                        (G::Matrix {
                            alignment: self.optional_alignment()?.unwrap_or(ColumnAlignment::Center),
                        }, Some(G::LeftRight(Some('['), Some(']'))), u16::MAX)
                    },
                    "vmatrix" => {
                        (G::Matrix {
                            alignment: ColumnAlignment::Center,
                        }, Some(G::LeftRight(Some('|'), Some('|'))), u16::MAX)
                    },
                    "vmatrix*" => {
                        (G::Matrix {
                            alignment: self.optional_alignment()?.unwrap_or(ColumnAlignment::Center),
                        }, Some(G::LeftRight(Some('|'), Some('|'))), u16::MAX)
                    },
                    "Vmatrix" => {
                        (G::Matrix {
                            alignment: ColumnAlignment::Center,
                        }, Some(G::LeftRight(Some('‖'), Some('‖'))), u16::MAX)
                    },
                    "Vmatrix*" => {
                        (G::Matrix {
                            alignment: self.optional_alignment()?.unwrap_or(ColumnAlignment::Center),
                        }, Some(G::LeftRight(Some('‖'), Some('‖'))), u16::MAX)
                    },
                    "Bmatrix" => {
                        (G::Matrix {
                            alignment: ColumnAlignment::Center,
                        }, Some(G::LeftRight(Some('{'), Some('}'))), u16::MAX)
                    },
                    "Bmatrix*" => {
                        (G::Matrix {
                            alignment: self.optional_alignment()?.unwrap_or(ColumnAlignment::Center),
                        }, Some(G::LeftRight(Some('{'), Some('}'))), u16::MAX)
                    },
                    "cases" => (G::Cases {
                        left: true,
                    }, None, 1),
                    "dcases" => {
                        style = Some(S::Display);
                        (G::Cases {
                            left: true,
                        }, None, 1)
                    },
                    "rcases" => (G::Cases {
                        left: false,
                    }, None, 1),
                    "drcases" => {
                        style = Some(S::Display);
                        (G::Cases {
                            left: false,
                        }, None, 1)
                    },
                    "equation" => todo!(),
                    "equation*" => todo!(),
                    "align" => (G::Align {
                        eq_numbers: true,
                    }, None, u16::MAX),
                    "align*" => (G::Align {
                        eq_numbers: false,
                    }, None, u16::MAX),
                    "aligned" => (G::Aligned, None, u16::MAX),
                    "gather" => (G::Gather {
                        eq_numbers: true,
                    }, None, 0),
                    "gather*" => (G::Gather {
                        eq_numbers: false,
                    }, None, 0),
                    "gathered" => (G::Gathered, None, 0),
                    "alignat" => {
                        let pairs = match lex::argument(&mut self.content)? {
                            Argument::Group(mut content) => lex::unsigned_integer(&mut content),
                            _ => Err(ErrorKind::Argument),
                        }? as u16;
                        (G::Alignat { pairs, eq_numbers: true }, None, (pairs * 2).saturating_sub(1))
                    },
                    "alignat*" => {
                        let pairs = match lex::argument(&mut self.content)? {
                            Argument::Group(mut content) => lex::unsigned_integer(&mut content),
                            _ => Err(ErrorKind::Argument),
                        }? as u16;
                        (G::Alignat { pairs, eq_numbers: false }, None, (pairs * 2).saturating_sub(1))
                    },
                    "alignedat" => {
                        let pairs = match lex::argument(&mut self.content)? {
                            Argument::Group(mut content) => lex::unsigned_integer(&mut content),
                            _ => Err(ErrorKind::Argument),
                        }? as u16;
                        (G::Alignedat { pairs }, None, (pairs * 2).saturating_sub(1))
                    },
                    "subarray" => {
                        let alignment = match lex::argument(&mut self.content)? {
                            Argument::Group("l") => ColumnAlignment::Left,
                            Argument::Group("c") => ColumnAlignment::Center,
                            Argument::Group("r") => ColumnAlignment::Right,
                            _ => return Err(ErrorKind::Argument),
                        };
                        (G::SubArray { alignment }, None, 0)
                    },
                    "multline" => (G::Multline, None, 0),
                    "split" => (G::Split, None, 1),
                    _ => return Err(ErrorKind::Environment),
                };

                let wrap_used = if let Some(wrap) = wrap {
                    self.buffer.push(I::Event(E::Begin(wrap)));
                    true
                } else {
                    false
                };
                
                // TODO: correctly spot deeper environment of the same type.
                let content = lex::group_content(
                    &mut self.content,
                    &format!(r"\begin{{{argument}}}"),
                    &format!(r"\end{{{argument}}}")
                )?;
                self.buffer.push(I::Event(E::Begin(environment)));
                if let Some(style) = style {
                    self.buffer.push(I::Event(E::StateChange(SC::Style(style))));
                }
                self.buffer.extend([
                    I::SubGroup { content, allowed_alignment_count: Some(AlignmentCount::new(align_count))},
                    I::Event(E::End)
                ]);

                if wrap_used {
                    self.buffer.push(I::Event(E::End));
                }
                return Ok(());
            }
            "end" => return Err(ErrorKind::UnbalancedGroup(None)),
            // TODO: make newlines accept a dimension argument
            "\\" | "cr" if self.state.allowed_alignment_count.is_some() => {
                self.state.allowed_alignment_count.as_mut().unwrap().reset();
                E::NewLine
            },

            // Delimiters
            cs if control_sequence_delimiter_map(cs).is_some() => {
                let (content, ty) = control_sequence_delimiter_map(cs).unwrap();
                E::Content(C::Delimiter { content, size: None, ty })
            }

            // Spacing
            c if c.trim_start().is_empty() => E::Content(C::Text("&nbsp;")),

            _ => return Err(ErrorKind::UnknownPrimitive),
        };
        self.buffer.push(I::Event(event));
        Ok(())
    }

    /// Return a delimiter with the given size from the next character in the parser.
    fn sized_delim(&mut self, size: DelimiterSize) -> InnerResult<()> {
        let current = &mut self.content;
        let (content, ty) = lex::delimiter(current)?;
        self.buffer
            .push(I::Event(E::Content(C::Delimiter { content, size: Some(size), ty })));
        Ok(())
    }

    /// Override the `font_state` for the argument to the command.
    fn font_group(&mut self, font: Option<Font>) -> InnerResult<()> {
        let argument = lex::argument(&mut self.content)?;
        self.buffer.extend([
            I::Event(E::Begin(G::Normal)),
            I::Event(E::StateChange(SC::Font(font))),
        ]);
        match argument {
            Argument::Token(token) => {
                match token {
                    Token::ControlSequence(cs) => self.handle_primitive(cs)?,
                    Token::Character(c) => self.handle_char_token(c)?,
                };
            }
            Argument::Group(group) => {
                self.buffer.push(I::SubGroup { content: group, allowed_alignment_count: None });
            }
        };
        self.buffer.push(I::Event(E::End));
        Ok(())
    }

    /// Accent commands. parse the argument, and overset the accent.
    fn accent(&mut self, accent: char, stretchy: bool) -> InnerResult<()> {
        let argument = lex::argument(&mut self.content)?;
        self.buffer.push(I::Event(E::Script {
            ty: ST::Superscript,
            position: SP::AboveBelow,
        }));
        self.handle_argument(argument)?;
        self.buffer
            .push(I::Event(E::Content(C::Ordinary {
                content: accent,
                stretchy,
            })));
        Ok(())
    }

    /// Underscript commands. parse the argument, and underset the accent.
    fn underscript(&mut self, content: char) -> InnerResult<()> {
        let argument = lex::argument(&mut self.content)?;
        self.buffer.push(I::Event(E::Script {
            ty: ST::Subscript,
            position: SP::AboveBelow,
        }));
        self.handle_argument(argument)?;
        self.buffer
            .push(I::Event(E::Content(C::Ordinary {
                content,
                stretchy: true,
            })));

        Ok(())
    }

    fn large_op(&mut self, op: char, above_below: bool) -> E<'a> {
        self.state.allow_suffix_modifiers = true;
        self.state.above_below_suffix_default = above_below;
        E::Content(C::LargeOp {
            content: op,
            small: false,
        })
    }

    fn font_change(&mut self, font: Font) -> E<'a> {
        self.state.skip_suffixes = true;
        E::StateChange(SC::Font(Some(font)))
    }

    fn style_change(&mut self, style: S) -> E<'a> {
        self.state.skip_suffixes = true;
        E::StateChange(SC::Style(style))
    }

    fn text_argument(&mut self) -> InnerResult<()> {
        let argument = lex::argument(&mut self.content)?;
        self.buffer
            .push(I::Event(E::Content(C::Text(
                match argument {
                    Argument::Token(Token::Character(c)) => c.as_str(),
                    Argument::Group(inner) => inner,
                    _ => return Err(ErrorKind::ControlSequenceAsArgument),
                },
            ))));
        Ok(())
    }

    fn fraction_like(
        &mut self,
        open: Option<char>,
        close: Option<char>,
        bar_size: Option<(f32, DimensionUnit)>,
        style: Option<S>
    ) -> InnerResult<()> {
        let open_close_group = open.is_some() || close.is_some();
        if open_close_group {
            self.buffer.push(I::Event(E::Begin(G::LeftRight(open, close))));
        }
        if let Some(style) = style {
            if !open_close_group {
                self.buffer.push(I::Event(E::Begin(G::Normal)));
            }
            self.buffer.push(I::Event(E::StateChange(SC::Style(style))));
        };
        
        self.buffer.push(I::Event(E::Visual(V::Fraction(bar_size))));
        let numerator = lex::argument(&mut self.content)?;
        self.handle_argument(numerator)?;
        let denominator = lex::argument(&mut self.content)?;
        self.handle_argument(denominator)?;
        if open_close_group || style.is_some() {
            self.buffer.push(I::Event(E::End));
        }
                  
        Ok(())
    }

    fn array_environment(&mut self) -> InnerResult<(G, u16)> {
        let Argument::Group(array_columns_str) = lex::argument(&mut self.content)? else {
            return Err(ErrorKind::Argument);
        };
        
        let mut column_count = 0;
        let array_columns = array_columns_str.chars().map(|c|  {
        column_count += 1;
        Ok(match c {
            'c' => AC::Column(ColumnAlignment::Center),
            'l' => AC::Column(ColumnAlignment::Left),
            'r' => AC::Column(ColumnAlignment::Right),
            '|' =>  {
                column_count -= 1;
                AC::VerticalLine
            }
            _ => return Err(ErrorKind::Argument), 
        })}).collect::<Result<_, _>>()?;
        
        Ok((G::Array(array_columns), column_count))
    }

    fn optional_alignment(&mut self) -> InnerResult<Option<ColumnAlignment>> {
        let alignment = lex::optional_argument(&mut self.content)?;
        Ok(match alignment {
            Some("c") => Some(ColumnAlignment::Center),
            Some("l") => Some(ColumnAlignment::Left),
            Some("r") => Some(ColumnAlignment::Right),
            None => None,
            _ => return Err(ErrorKind::Argument),
        })
    }
}

#[inline]
fn ordinary(ident: char) -> E<'static> {
    E::Content(C::Ordinary {
        content: ident,
        stretchy: false
    })
}

#[inline]
fn relation(rel: char) -> E<'static> {
    E::Content(C::Relation {
        content: rel,
        small: false,
    })
}

#[inline]
fn binary(op: char) -> E<'static> {
    E::Content(C::BinaryOp{ content: op, small: false })
}

// TODO implementations:
// - `raise`, `lower`
// - `hbox`, `mbox`?
// - `vcenter`
// - `rule`
// - `math_` atoms
// - `mathchoice` (TeXbook p. 151)

// Unimplemented primitives:
// `sl` (slanted) font: https://tug.org/texinfohtml/latex2e.html#index-_005csl
// `bbit` (double-struck italic) font
// `symliteral` wtf is this? (in unicode-math)
// `sc` (small caps) font: https://tug.org/texinfohtml/latex2e.html#index-_005csc
