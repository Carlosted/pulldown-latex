//! A simple MathML Core renderer.
//!
//! This crate provides a "simple" `mathml` renderer which is available through the
//! [`push_mathml`] and [`write_mathml`] functions.

use std::{io, iter::Peekable};

use crate::{
    attribute::{tex_to_css_em, Font},
    config::{DisplayMode, RenderConfig},
    event::{
        ArrayColumn, ColorChange, ColorTarget, Content, DelimiterType, Event, Grouping,
        ScriptPosition, ScriptType, StateChange, Style, Visual,
    },
};

struct MathmlWriter<'a, I: Iterator, W> {
    input: Peekable<I>,
    writer: W,
    config: RenderConfig<'a>,
    env_stack: Vec<Environment>,
    state_stack: Vec<State<'a>>,
    previous_atom: Option<Atom>,
}

impl<'a, I, W, E> MathmlWriter<'a, I, W>
where
    I: Iterator<Item = Result<Event<'a>, E>>,
    W: io::Write,
    E: std::error::Error,
{
    fn new(input: I, writer: W, config: RenderConfig<'a>) -> Self {
        // Size of the buffer is arbitrary for performance guess.
        let mut state_stack = Vec::with_capacity(16);
        state_stack.push(State {
            font: None,
            text_color: None,
            border_color: None,
            background_color: None,
            style: None,
        });
        let env_stack = Vec::with_capacity(16);
        Self {
            input: input.peekable(),
            writer,
            config,
            env_stack,
            state_stack,
            previous_atom: None,
        }
    }

    fn open_tag(&mut self, tag: &str, classes: Option<&str>) -> io::Result<()> {
        let State {
            text_color,
            border_color,
            background_color,
            style,
            font: _,
        } = *self.state();
        write!(self.writer, "<{}", tag)?;
        if let Some(style) = style {
            let args = match style {
                Style::Display => (true, 0),
                Style::Text => (false, 0),
                Style::Script => (false, 1),
                Style::ScriptScript => (false, 2),
            };
            write!(
                self.writer,
                " displaystyle=\"{}\" scriptlevel=\"{}\"",
                args.0, args.1
            )?;
        }

        let mut style_written = false;
        if let Some(text_color) = text_color {
            write!(self.writer, " style=\"color: {}", text_color)?;
            style_written = true;
        }
        if let Some(border_color) = border_color {
            if style_written {
                write!(self.writer, "; border: 0.06em solid {}", border_color)?;
            } else {
                write!(
                    self.writer,
                    " style=\"border: 0.06em solid {}",
                    border_color
                )?;
                style_written = true;
            }
        }
        if let Some(background_color) = background_color {
            if style_written {
                write!(self.writer, "; background-color: {}", background_color)?;
            } else {
                write!(
                    self.writer,
                    " style=\"background-color: {}",
                    background_color
                )?;
                style_written = true;
            }
        }
        if style_written {
            self.writer.write_all(b"\"")?;
        }
        if let Some(classes) = classes {
            write!(self.writer, " style=\"{}\"", classes)?;
        }
        Ok(())
    }

    fn state(&self) -> &State<'a> {
        self.state_stack.last().expect("state stack is empty")
    }

    fn write_event(&mut self, event: Result<Event<'a>, E>) -> io::Result<()> {
        match event {
            Ok(Event::Content(content)) => self.write_content(content, false),
            // TODO: environments.
            // Gather: Does not accept alignments, only newlines. eveything is centered.
            // Align: Columns do the following:
            //     right, left, space, right, left, space ...
            //     - no spaces between columns
            // Array: requires a column specification.
            // Cases: Only one alignment allowed, has considerable (probably \quad) space between columns.
            Ok(Event::Begin(grouping)) => {
                // Mathematical environments do something different with state compared to things
                // like left/right and {}.
                //
                // - They do not inherit state from their parent, their state is reset to default
                // upon entering.
                // - State changes occuring within them are also reset when crossing alignments or
                // newlines (`&` or `\\`).
                //
                // For these reasons, they don't use the `open_tag` method.
                self.previous_atom = None;
                if grouping.is_math_env() {
                    self.state_stack.push(State::default())
                } else {
                    let last_state = *self.state_stack.last().expect("state stack is empty");
                    self.state_stack.extend([last_state, last_state]);
                    while let Some(Ok(Event::StateChange(state_change))) = self.input.peek() {
                        let state_change = *state_change;
                        self.handle_state_change(state_change);
                        self.input.next();
                    }
                    self.open_tag("mrow", None)?;
                    self.writer.write_all(b">")?;
                    self.state_stack.pop();
                }

                let env_group = match grouping {
                    Grouping::Normal => EnvGrouping::Normal,
                    Grouping::LeftRight(opening, closing) => {
                        if let Some(delim) = opening {
                            self.open_tag("mo", None)?;
                            self.writer.write_all(b" stretchy=\"true\">")?;
                            let mut buf = [0u8; 4];
                            self.writer
                                .write_all(delim.encode_utf8(&mut buf).as_bytes())?;
                            self.writer.write_all(b"</mo>")?;
                        }
                        self.previous_atom = Some(Atom::Open);
                        EnvGrouping::LeftRight { closing }
                    }
                    Grouping::Align => {
                        self.writer
                            .write_all(b"<mtable><mtr><mtd class=\"menv-right-align\">")?;
                        EnvGrouping::Align { left_align: false }
                    }
                    Grouping::Matrix => {
                        self.writer.write_all(b"<mtable><mtr><mtd>")?;
                        EnvGrouping::Matrix
                    }
                    Grouping::Cases => {
                        self.writer.write_all(b"<mrow><mo stretchy=\"true\">{</mo><mtable><mtr><mtd class=\"menv-left-align\">")?;
                        EnvGrouping::Cases { used_align: false }
                    }
                    Grouping::Array(cols) => {
                        let mut index = 0;
                        self.writer.write_all(b"<mtable")?;
                        match (cols.first(), cols.last()) {
                            (Some(ArrayColumn::VerticalLine), Some(ArrayColumn::VerticalLine)) => {
                                index += 1;
                                self.writer
                                    .write_all(b" class=\"menv-left-border menv-right-border\"")?
                            }
                            (Some(ArrayColumn::VerticalLine), _) => {
                                index += 1;
                                self.writer.write_all(b" class=\"menv-left-border\"")?
                            }
                            (_, Some(ArrayColumn::VerticalLine)) => {
                                self.writer.write_all(b" class=\"menv-right-border\"")?
                            }
                            (_, _) => {}
                        }
                        self.writer.write_all(b"><mtr><mtd")?;
                        match cols[index] {
                            ArrayColumn::Left => {
                                self.writer.write_all(b" class=\"menv-left-align\"")?
                            }
                            ArrayColumn::Right => {
                                self.writer.write_all(b" class=\"menv-right-align\"")?
                            }
                            ArrayColumn::Center => {}
                            ArrayColumn::VerticalLine => {
                                panic!("vertical line in place of column alignment")
                            }
                        };
                        self.writer.write_all(b">")?;
                        EnvGrouping::Array {
                            cols,
                            cols_index: index + 1,
                        }
                    }
                };
                self.env_stack.push(Environment::from(env_group));
                Ok(())
            }
            Ok(Event::End) => {
                let env = self
                    .env_stack
                    .pop()
                    .expect("cannot pop an environment in group end");
                let Environment::Group(grouping) = env else {
                    panic!("unexpected environment in group end");
                };
                self.state_stack
                    .pop()
                    .expect("cannot pop a state in group end");
                self.previous_atom = Some(Atom::Inner);
                match grouping {
                    EnvGrouping::Normal => self.writer.write_all(b"</mrow>"),
                    EnvGrouping::LeftRight { closing } => {
                        if let Some(delim) = closing {
                            self.open_tag("mo", None)?;
                            self.writer.write_all(b" stretchy=\"true\">")?;
                            let mut buf = [0u8; 4];
                            self.writer
                                .write_all(delim.encode_utf8(&mut buf).as_bytes())?;
                            self.writer.write_all(b"</mo>")?;
                        }
                        self.previous_atom = Some(Atom::Close);
                        self.writer.write_all(b"</mrow>")
                    }
                    EnvGrouping::Matrix | EnvGrouping::Align { .. } | EnvGrouping::Array { .. } => {
                        self.writer.write_all(b"</mtd></mtr></mtable>")
                    }
                    EnvGrouping::Cases { .. } => {
                        self.writer.write_all(b"</mtd></mtr></mtable></mrow>")
                    }
                }
            }
            Ok(Event::Visual(visual)) => {
                if visual == Visual::Negation {
                    match self.input.peek() {
                        Some(Ok(Event::Content(
                            content @ Content::Ordinary { .. }
                            | content @ Content::Relation { .. }
                            | content @ Content::BinaryOp { .. }
                            | content @ Content::LargeOp { .. }
                            | content @ Content::Delimiter { .. }
                            | content @ Content::Punctuation(_),
                        ))) => {
                            let content = *content;
                            self.write_content(content, true)?;
                            self.input.next();
                        }
                        _ => {
                            self.open_tag("mrow", Some("negated"))?;
                            self.writer.write_all(b">")?;
                            self.env_stack.push(Environment::from(visual));
                        }
                    }
                    return Ok(());
                }

                let env = Environment::from(visual);
                self.env_stack.push(env);
                self.open_tag(visual_tag(visual), None)?;
                if let Visual::Fraction(Some(dim)) = visual {
                    write!(self.writer, " linethickness=\"{}em\"", tex_to_css_em(dim))?;
                }

                self.writer.write_all(b">")
            }

            Ok(Event::Script { ty, position }) => {
                let state = self.state();
                let above_below = match position {
                    ScriptPosition::Right => false,
                    ScriptPosition::AboveBelow => true,
                    ScriptPosition::Movable => {
                        state.style == Some(Style::Display)
                            || (state.style.is_none()
                                && self.config.display_mode == DisplayMode::Block)
                    }
                };
                let env = Environment::from((ty, above_below));
                self.env_stack.push(env);
                self.open_tag(script_tag(ty, above_below), None)?;
                self.writer.write_all(b">")
            }

            Ok(Event::Space {
                width,
                height,
                depth,
            }) => {
                if let Some(width) = width {
                    write!(self.writer, "<mspace width=\"{}em\"", tex_to_css_em(width))?;
                    if width.0 < 0.0 {
                        write!(
                            self.writer,
                            " style=\"margin-left: {}em\"",
                            tex_to_css_em(width)
                        )?;
                    }
                }
                if let Some(height) = height {
                    write!(self.writer, " height=\"{}em\"", tex_to_css_em(height))?;
                }
                if let Some(depth) = depth {
                    write!(self.writer, " depth=\"{}em\"", tex_to_css_em(depth))?;
                }
                self.writer.write_all(b" />")
            }
            Ok(Event::StateChange(state_change)) => {
                self.handle_state_change(state_change);
                Ok(())
            }
            Ok(Event::NewLine) => {
                *self.state_stack.last_mut().expect("state stack is empty") = State::default();
                self.previous_atom = None;
                match self.env_stack.last_mut() {
                    Some(Environment::Group(EnvGrouping::Cases { .. })) => self
                        .writer
                        .write_all(b"</mtd></mtr><mtr><mtd class=\"menv-left-align\">"),
                    Some(Environment::Group(EnvGrouping::Matrix)) => {
                        self.writer.write_all(b"</mtd></mtr><mtr><mtd>")
                    }
                    Some(Environment::Group(EnvGrouping::Align)) => self
                        .writer
                        .write_all(b"</mtd></mtr><mtr class=\"menv-align-row\"><mtd>"),
                    Some(Environment::Group(EnvGrouping::Array { cols, cols_index })) => {
                        *cols_index =
                            (cols.first() == Some(&ArrayColumn::VerticalLine)) as usize + 1;
                        self.writer.write_all(b"</mtd></mtr><mtr><mtd")?;
                        match cols[*cols_index - 1] {
                            ArrayColumn::Left => {
                                self.writer.write_all(b" class=\"menv-left-align\"")?
                            }
                            ArrayColumn::Right => {
                                self.writer.write_all(b" class=\"menv-right-align\"")?
                            }
                            ArrayColumn::Center => {}
                            ArrayColumn::VerticalLine => {
                                panic!("vertical line in place of column alignment")
                            }
                        }
                        self.writer.write_all(b">")
                    }
                    _ => panic!("math env does not support newlines"),
                }
            }
            Ok(Event::Alignment) => {
                *self.state_stack.last_mut().expect("state stack is empty") = State::default();
                self.previous_atom = None;
                match self.env_stack.last_mut() {
                    // Left align both
                    Some(Environment::Group(EnvGrouping::Cases { used_align: false })) => self
                        .writer
                        .write_all(b"</mtd><mtd class=\"menv-left-align\">"),
                    // Center align all
                    Some(Environment::Group(EnvGrouping::Matrix)) => {
                        self.writer.write_all(b"</mtd><mtd>")
                    }
                    Some(Environment::Group(EnvGrouping::Array { cols, cols_index })) => {
                        self.writer.write_all(b"</mtd><mtd class=\"")?;
                        if cols[*cols_index] == ArrayColumn::VerticalLine {
                            self.writer.write_all(b"menv-left-border ")?;
                            *cols_index += 1;
                        }
                        self.writer.write_all(match cols[*cols_index] {
                            ArrayColumn::Left => b"menv-left-align\">",
                            ArrayColumn::Right => b"menv-right-align\">",
                            ArrayColumn::Center => b"\">",
                            ArrayColumn::VerticalLine => {
                                panic!("vertical line in place of column alignment")
                            }
                        })?;
                        *cols_index += 1;
                        Ok(())
                    }
                    Some(Environment::Group(EnvGrouping::Align)) => {
                        write!(self.writer, "</mtd><mtd>",)
                    }
                    _ => panic!("alignment not allowed in current environment"),
                }
            }
            Err(e) => {
                let error_color = self.config.error_color;
                write!(
                    self.writer,
                    "<merror style=\"border-color: #{:x}{:x}{:x}\"><mtext>",
                    error_color.0, error_color.1, error_color.2
                )?;
                self.writer.write_all(e.to_string().as_bytes())?;
                self.writer.write_all(b"</mtext></merror>")
            }
        }
    }

    fn write_content(&mut self, content: Content<'a>, negate: bool) -> io::Result<()> {
        let mut buf = [0u8; 4];
        match content {
            Content::Text(text) => {
                self.open_tag("mtext", None)?;
                self.writer.write_all(b">")?;
                self.writer.write_all(text.as_bytes())?;
                self.set_previous_atom(Atom::Ord);
                self.writer.write_all(b"</mtext>")
            }
            Content::Number(number) => {
                self.open_tag("mn", None)?;
                self.writer.write_all(b">")?;
                let buf = &mut [0u8; 4];
                number.chars().try_for_each(|c| {
                    let content = self.state().font.map_or(c, |v| v.map_char(c));
                    let bytes = content.encode_utf8(buf);
                    self.writer.write_all(bytes.as_bytes())
                })?;
                self.set_previous_atom(Atom::Ord);
                self.writer.write_all(b"</mn>")
            }
            // TODO: script shenanigans and parens vs. no parens.
            Content::Function(str) => {
                self.open_tag("mi", None)?;
                self.writer.write_all(if str.chars().count() == 1 {
                    b" mathvariant=\"normal\">"
                } else {
                    b">"
                })?;
                self.writer.write_all(str.as_bytes())?;
                self.set_previous_atom(Atom::Op);
                self.writer.write_all(b"</mi>")

                // TODO: Add function application symbol when no paren is there.
                // let to_append = "<mo>\u{2061}</mo><mspace width=\"0.1667em\" />";
            }
            Content::Ordinary { content, stretchy } => {
                if stretchy {
                    self.writer.write_all(b"<mo stretchy=\"true\">")?;
                    self.writer
                        .write_all(content.encode_utf8(&mut buf).as_bytes())?;
                    self.writer.write_all(b"</mo>")
                } else {
                    self.open_tag("mi", None)?;
                    let content = match (
                        self.state().font,
                        self.config.math_style.should_be_upright(content),
                    ) {
                        (Some(Font::UpRight), _) | (None, true) => {
                            self.writer.write_all(b" mathvariant=\"normal\">")?;
                            content
                        }
                        (Some(font), _) => {
                            self.writer.write_all(b">")?;
                            font.map_char(content)
                        }
                        _ => {
                            self.writer.write_all(b">")?;
                            content
                        }
                    };

                    let buf = &mut [0u8; 4];
                    let bytes = content.encode_utf8(buf);
                    self.writer.write_all(bytes.as_bytes())?;
                    if negate {
                        self.writer.write_all("\u{0338}".as_bytes())?;
                    }
                    self.set_previous_atom(Atom::Ord);
                    self.writer.write_all(b"</mi>")
                }
            }
            // TODO: Follow plain tex's rules for spacing:
            // Varing is a binary when:
            // 1. preceded by closing
            // 3. preceded by punctuation
            // 4. preceded by number
            // 5. preceded by normal
            // and:
            // 1. followed by closing
            // 4. followed by number
            // 5. followed by normal
            //
            // If the current item is a Bin atom, and if this was the first atom in the list,
            // or if the most recent previous atom was Bin, Op, Rel, Open, or Punct, change the current
            // Bin to Ord and continue with Rule 14. Otherwise continue with Rule 17.
            //
            // If the current item is a Rel or Close or Punct atom, and if the most recent previous atom
            // was Bin, change that previous Bin to Ord. Continue with Rule 17.
            //
            // TexBook p. 153 and 157 for math classes.
            Content::BinaryOp { content, small } => {
                let tag = if matches!(
                    self.previous_atom,
                    Some(Atom::Inner | Atom::Close | Atom::Ord)
                ) && matches!(
                    self.input.peek(),
                    Some(Ok(Event::Begin(_)
                        | Event::Visual(_)
                        | Event::Script { .. }
                        | Event::Content(
                            // TODO: check for next atom farther using multi-peek for scripts, visuals,
                            // spacing, and state changes.
                            Content::BinaryOp { .. }
                                | Content::Text(_)
                                | Content::Function(_)
                                | Content::Number(_)
                                | Content::LargeOp { .. }
                                | Content::Ordinary { .. }
                                | Content::Delimiter {
                                    ty: DelimiterType::Open,
                                    ..
                                }
                        )))
                ) {
                    self.set_previous_atom(Atom::Bin);
                    "mo"
                } else {
                    self.set_previous_atom(Atom::Ord);
                    "mi"
                };

                self.open_tag(tag, small.then_some("small"))?;
                self.writer.write_all(b">")?;
                self.writer
                    .write_all(content.encode_utf8(&mut buf).as_bytes())?;
                if negate {
                    self.writer.write_all("\u{0338}".as_bytes())?;
                }
                write!(self.writer, "</{}>", tag)
            }
            Content::Relation { content, small } => {
                self.open_tag("mo", small.then_some("small"))?;
                self.writer.write_all(b">")?;
                self.writer
                    .write_all(content.encode_utf8(&mut buf).as_bytes())?;
                if negate {
                    self.writer.write_all("\u{0338}".as_bytes())?;
                }
                self.set_previous_atom(Atom::Rel);
                self.writer.write_all(b"</mo>")
            }

            Content::LargeOp { content, small } => {
                self.open_tag("mo", None)?;
                if small {
                    self.writer.write_all(b" largeop=\"false\"")?;
                }
                self.writer.write_all(b" movablelimits=\"false\">")?;
                self.writer
                    .write_all(content.encode_utf8(&mut buf).as_bytes())?;
                if negate {
                    self.writer.write_all("\u{0338}".as_bytes())?;
                }
                self.set_previous_atom(Atom::Op);
                self.writer.write_all(b"</mo>")
            }
            Content::Delimiter { content, size, ty } => {
                self.open_tag("mo", None)?;
                write!(self.writer, " stretchy=\"{}\"", ty == DelimiterType::Fence)?;
                if let Some(size) = size {
                    write!(
                        self.writer,
                        "minsize=\"{}em\" maxsize=\"{}em\"",
                        size.to_em(),
                        size.to_em()
                    )?;
                }
                self.writer.write_all(b">")?;
                self.writer
                    .write_all(content.encode_utf8(&mut buf).as_bytes())?;
                if negate {
                    self.writer.write_all("\u{0338}".as_bytes())?;
                }
                self.set_previous_atom(match ty {
                    DelimiterType::Open => Atom::Open,
                    DelimiterType::Fence => Atom::Punct,
                    DelimiterType::Close => Atom::Close,
                });
                self.writer.write_all(b"</mo>")
            }
            Content::Punctuation(content) => {
                self.open_tag("mo", None)?;
                self.writer.write_all(b">")?;
                self.writer
                    .write_all(content.encode_utf8(&mut buf).as_bytes())?;
                if negate {
                    self.writer.write_all("\u{0338}".as_bytes())?;
                }
                self.set_previous_atom(Atom::Punct);
                self.writer.write_all(b"</mo>")
            }
            Content::MultiRelation(content) => {
                self.open_tag("mo", None)?;
                self.writer.write_all(b">")?;
                self.writer.write_all(content.as_bytes())?;
                self.set_previous_atom(Atom::Rel);
                self.writer.write_all(b"</mo>")
            }
        }
    }

    fn handle_state_change(&mut self, state_change: StateChange<'a>) {
        let state = self.state_stack.last_mut().expect("state stack is empty");
        match state_change {
            StateChange::Font(font) => state.font = font,
            StateChange::Color(ColorChange { color, target }) => match target {
                ColorTarget::Text => state.text_color = Some(color),
                ColorTarget::Border => state.border_color = Some(color),
                ColorTarget::Background => state.background_color = Some(color),
            },
            StateChange::Style(style) => state.style = Some(style),
        }
    }

    fn set_previous_atom(&mut self, atom: Atom) {
        if !matches!(
            self.env_stack.last(),
            Some(
                Environment::Visual {
                    ty: Visual::Root | Visual::Fraction(_),
                    count: 0
                } | Environment::Script {
                    ty: ScriptType::Subscript | ScriptType::Superscript,
                    count: 0,
                    ..
                } | Environment::Script {
                    ty: ScriptType::SubSuperscript,
                    count: 0 | 1,
                    ..
                }
            )
        ) {
            self.previous_atom = Some(atom);
        }
    }

    fn write(mut self) -> io::Result<()> {
        // Safety: this function must only write valid utf-8 to the writer.
        // How is the writer used?:
        // - using `write_all` with a utf-8 string.
        // - using `write!` with a utf-8 string, and the parameters must all be valid utf-8 since
        //      they are formatted using the `Display` trait.
        write!(
            self.writer,
            "<math display=\"{}\"",
            self.config.display_mode
        )?;
        if self.config.xml {
            self.writer
                .write_all(b" xmlns=\"http://www.w3.org/1998/Math/MathML\"")?;
        }
        self.writer.write_all(b">")?;
        if self.config.annotation.is_some() {
            self.writer.write_all(b"<semantics>")?;
        }

        while let Some(event) = self.input.next() {
            self.write_event(event)?;

            while let Some((tag, count)) = self.env_stack.last_mut().and_then(|env| match env {
                Environment::Group(_) => None,
                Environment::Visual { ty, count } => Some((visual_tag(*ty), count)),
                Environment::Script {
                    ty,
                    above_below,
                    count,
                } => Some((script_tag(*ty, *above_below), count)),
            }) {
                if *count == 0 {
                    self.writer.write_all(b"</")?;
                    self.writer.write_all(tag.as_bytes())?;
                    self.writer.write_all(b">")?;
                    self.env_stack.pop();
                    continue;
                }
                *count -= 1;
                break;
            }
        }

        if !self.env_stack.is_empty() || self.state_stack.len() != 1 {
            panic!("unbalanced environment stack or state stack");
        }

        if let Some(annotation) = self.config.annotation {
            write!(
                self.writer,
                "<annotation encoding=\"application/x-tex\">{}</annotation>",
                annotation
            )?;
            self.writer.write_all(b"</semantics>")?;
        }
        self.writer.write_all(b"</math>")
    }
}

enum Atom {
    Bin,
    Op,
    Rel,
    Open,
    Close,
    Punct,
    Ord,
    Inner,
}

#[derive(Debug, Clone, PartialEq)]
enum EnvGrouping {
    Normal,
    LeftRight {
        closing: Option<char>,
    },
    Array {
        cols: Box<[ArrayColumn]>,
        cols_index: usize,
    },
    Matrix,
    Cases {
        used_align: bool,
    },
    Align,
}

#[derive(Debug, Clone, PartialEq)]
enum Environment {
    Group(EnvGrouping),
    Visual {
        ty: Visual,
        count: u8,
    },
    Script {
        ty: ScriptType,
        above_below: bool,
        count: u8,
    },
}

impl From<EnvGrouping> for Environment {
    fn from(v: EnvGrouping) -> Self {
        Self::Group(v)
    }
}

impl From<(ScriptType, bool)> for Environment {
    fn from((ty, above_below): (ScriptType, bool)) -> Self {
        let count = match ty {
            ScriptType::Subscript => 2,
            ScriptType::Superscript => 2,
            ScriptType::SubSuperscript => 3,
        };
        Self::Script {
            ty,
            above_below,
            count,
        }
    }
}

impl From<Visual> for Environment {
    fn from(v: Visual) -> Self {
        let count = match v {
            Visual::SquareRoot => 1,
            Visual::Root => 2,
            Visual::Fraction(_) => 2,
            Visual::Negation => 1,
        };
        Self::Visual { ty: v, count }
    }
}

fn script_tag(ty: ScriptType, above_below: bool) -> &'static str {
    match (ty, above_below) {
        (ScriptType::Subscript, false) => "msub",
        (ScriptType::Superscript, false) => "msup",
        (ScriptType::SubSuperscript, false) => "msubsup",
        (ScriptType::Subscript, true) => "munder",
        (ScriptType::Superscript, true) => "mover",
        (ScriptType::SubSuperscript, true) => "munderover",
    }
}

fn visual_tag(visual: Visual) -> &'static str {
    match visual {
        Visual::Root => "mroot",
        Visual::Fraction(_) => "mfrac",
        Visual::SquareRoot => "msqrt",
        Visual::Negation => "mrow",
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct State<'a> {
    font: Option<Font>,
    text_color: Option<&'a str>,
    border_color: Option<&'a str>,
    background_color: Option<&'a str>,
    style: Option<Style>,
}

/// Takes a [`Parser`], or any `Iterator<Item = Result<Event<'_>, E>>` as input, and renders a
/// string of MathML into the input string.
///
/// [`Parser`]: crate::parser::Parser
pub fn push_mathml<'a, I, E>(
    string: &mut String,
    parser: I,
    config: RenderConfig<'a>,
) -> io::Result<()>
where
    I: Iterator<Item = Result<Event<'a>, E>>,
    E: std::error::Error,
{
    // SAFETY: The MathmlWriter guarantees that all writes to the writer are valid utf-8.
    MathmlWriter::new(parser, unsafe { string.as_mut_vec() }, config).write()
}

/// Takes a [`Parser`], or any `Iterator<Item = Result<Event<'_>, E>>`, as input and renders the
/// MathML into the given writer.
///
/// [`Parser`]: crate::parser::Parser
pub fn write_mathml<'a, I, W, E>(writer: W, parser: I, config: RenderConfig<'a>) -> io::Result<()>
where
    I: Iterator<Item = Result<Event<'a>, E>>,
    W: io::Write,
    E: std::error::Error,
{
    MathmlWriter::new(parser, writer, config).write()
}
