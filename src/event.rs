//! The definition of the [`Event`] enum, which is used as a logical
//! representation of `LaTeX` content.
//!
//! A stream of `Result<Event, ParserError>`s is produced by the [`Parser`], which can then be typeset/rendered
//! by a renderer. This crate only provides a simple `mathml` renderer available through the
//! [`push_mathml`] and [`write_mathml`] functions.
//!
//! [`Parser`]: crate::parser::Parser
//! [`push_mathml`]: crate::mathml::push_mathml
//! [`write_mathml`]: crate::mathml::write_mathml

use crate::attribute::{Dimension, Font};

/// All events that can be produced by the parser.
///
/// When an [`Event`] is referreing to an "_element_", it is referring to the next logical unit of
/// content in the stream. This can be a single content element, a group, a visual element, etc.
#[derive(Debug, Clone, PartialEq)]
pub enum Event<'a> {
    /// The event is a [`Content`] element.
    Content(Content<'a>),
    /// The events following this one constitute a "group" which counts as a single _element_
    /// (i.e., a set of elements within `{}` in `LaTeX`), until the [`Event::EndGroup`] event
    /// is reached.
    Begin(Grouping),
    /// Marks the end of a "group".
    End,
    /// The `n` events following this one constitute the content of the [`Visual`] element. `n` is
    /// defined in the documentation of for the [`Visual`] variant.
    Visual(Visual),
    /// The `n` events following this one constitute a base and its script(s). `n` is defined in
    /// the documentation for the associated [`ScriptType`] variant.
    Script {
        ty: ScriptType,
        position: ScriptPosition,
    },
    /// This events specifes a custom spacing element. This is produced by commands such as
    /// `\kern`, `\hspace`, etc.
    ///
    /// If any of the components are `None`, then the spacing is set to 0 for that component.
    Space {
        width: Option<Dimension>,
        height: Option<Dimension>,
        depth: Option<Dimension>,
    },
    /// This event specifies a state change in the renderer.
    ///
    /// This state change only applies to the current group nesting and deeper groups.
    StateChange(StateChange<'a>),
    /// This event specifies an alignment mark in a mathematical environment.
    ///
    /// This event is only used when inside a `Grouping` that allows it.
    Alignment,
    /// This event specifies a line break in a mathematical environment.
    ///
    /// This event is only used when inside a `Grouping` that allows it.
    NewLine,
}

/// Base events that produce `mathml` nodes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Content<'a> {
    /// Text content that should be typeset following the rules of `LaTeX`'s `text` mode.
    Text(&'a str),
    /// A number, which can include decimal points and commas.
    Number(&'a str),
    /// A function identifier, such as `sin`, `lim`, or a custom function with
    /// `\operatorname{arccotan}`.
    Function(&'a str),
    /// A relation made up of multiple characters such as `:=` (\coloneqq), or made with combining
    /// characters such as `⫋︀` (\varsubsetneqq).
    MultiRelation(&'a str),
    /// A variable identifier, such as `x`, `\theta`, `\aleph`, etc., and stuff that do not have
    /// any spacing around them. This includes stuff that normally go in under and overscripts
    /// which may be stretchy, such as `→`, `‾`, etc.
    Ordinary { content: char, stretchy: bool },
    /// A large operator, such as `\sum`, `\int`, `\prod`, etc.
    ///
    // TODO: Deny movable limits in renderer
    LargeOp { content: char, small: bool },
    /// A binary operator, such as `+`, `*`, `⊗`, `?`, etc.
    BinaryOp { content: char, small: bool },
    /// A relation, such as `=`, `≠`, `≈`, etc.
    Relation {
        content: char,
        small: bool,
    },
    /// An opening, closing, or fence delimiter, such as `(`, `[`, `{`, `|`, `)`, `]`, `}`, etc.
    Delimiter {
        content: char,
        size: Option<DelimiterSize>,
        ty: DelimiterType,
    },
    /// A punctuation character, such as `,`, `.`, `;`, etc.
    Punctuation(char),
}

// MathML operator types:
// A: Arrows and other stretchy stuff
// B: Binary operators
// C: Things with less spacing such as `%`, `*`, `⊗`, `?`
// D: Prefixes
// E: Postfixes
// F: Opening Delim
// G: Closing Delim
// H: Prefix large operators (integrals)
// I: Stretchy under and overscripts
// J: Prefix large operators p2 (sums, products, etc.) Those are specified to have movable limits
//     per mathml spec.
// K: Invisible accessibility stuff, (function application, invisible plus, etc.)
// L: Derivative specific stuff such as `d`, `∂`, etc.
// M: Punctuation
//
// Could conceivably be regrouped into the following:
// Binary operators: (part of B) (part of C)
// Relations: (part of B)
// Unary operators: D E L (parto of C)
// Large operators: H J
// Stretchy stuff: A I
// Delimiters (should be slplit): F G
// Punctuation: M

/// Modifies the visual representation of the following event(s)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visual {
    /// The following element is the content of the root.
    SquareRoot,
    /// The 2 following elements are the radicand and the index of the root.
    Root,
    /// The 2 following elements are the numerator and denominator of the fraction.
    ///
    /// If the content of the variant is `None`, then the size of the middle line is set to the
    /// default size, otherwise the size is set to the specified size.
    Fraction(Option<Dimension>),
    /// The "negation" operator as in "not equal" (≠) or "does not exist" (∄). This applies to the
    /// next event in the stream.
    ///
    /// This event can occur before an arbitrary event, not just a `Content` event. It is left to
    /// the renderer to determine how to apply the negation. In `LaTeX`, the renderer usually
    /// generates an akward looking negation across the next element, when it does not correspond
    /// to a commonly negated element.
    Negation,
}

/// Logical type of the script. This is used to determine how to render the scripts.
///
/// Things like subscripts, underscripts, and movable scripts can be represented when using this
/// `enum` in conjunction with the [`ScriptPosition`] `enum`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScriptType {
    /// The 2 following elements are the base and and the subscript
    Subscript,
    /// The 2 following elements are the base and and the superscript
    Superscript,
    /// The 3 following elements are the base, subscript and superscript
    SubSuperscript,
}

/// Position of the script. This is used to determine how to render the scripts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScriptPosition {
    /// The scripts are rendered to the (bottom and top) right of the operator.
    Right,
    /// The scripts are rendered above and below the operator instead of to the right.
    AboveBelow,
    /// Is set to `AboveBelow` by preference, but should be changed to `Right` when rendering in
    /// inline mode.
    ///
    /// This is used by the `lim` and `sum` (Σ) operators for example.
    Movable,
}

/// Represents a state change for the following content.
///
/// State changes take effect for the current group nesting and all deeper groups.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StateChange<'a> {
    /// Changes the font of the content.
    ///
    /// If the font is `None`, then the default renderer font is used, otherwise the font is set to
    /// the specified font.
    Font(Option<Font>),
    /// Changes the color of the content.
    Color(ColorChange<'a>),
    /// Changes the style of the content (mostly affects the sizing of the content).
    Style(Style),
}

/// The style of the content.
///
/// This is analogous to the different "modes" in `LaTeX`, such as `display`, `text`, etc., which
/// are set by commands like `\displaystyle`, `\textstyle`, etc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Display,
    Text,
    Script,
    ScriptScript,
}

/// Represents a color change.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorChange<'a> {
    /// The color to change to.
    ///
    /// A string that represents the color to change to, either as a hex code in the form #RRGGBB,
    /// or as one of the color names existing as part of CSS3 (e.g., "red").
    pub color: &'a str,
    /// The target of the color change.
    ///
    /// Specifies which part of the content to change the color of.
    pub target: ColorTarget,
}

/// The target of the color change.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorTarget {
    /// The text of the content.
    Text,
    /// The background of the content.
    Background,
    /// The border surrounding the content.
    Border,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Grouping {
    Normal,
    LeftRight(Option<char>, Option<char>),
    Array(Box<[ArrayColumn]>),
    Matrix {
        alignment: ColumnAlignment,
    },
    Cases {
        left: bool,
    },
    Equation {
        eq_numbers: bool,
    },
    Align {
        eq_numbers: bool,
    },
    Aligned,
    // According to what was specified
    SubArray {
        alignment: ColumnAlignment,
    },
    // Same as align, but without space between columns, and specified number of left right
    // pairs.
    Alignat {
        pairs: u16,
        eq_numbers: bool,
    },
    Alignedat {
        pairs: u16,
    },
    // All center
    Gather {
        eq_numbers: bool,
    },
    Gathered,
    // First: left, last: right, in between: center
    Multline,
    // Only one alignment allowed, right, left just like `align`
    Split,
}

impl Grouping {
    pub(crate) fn is_math_env(&self) -> bool {
        !matches!(self, Self::Normal | Self::LeftRight(_, _))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArrayColumn {
    Column(ColumnAlignment),
    VerticalLine,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DelimiterSize {
    /// Corresponds to `\bigl`, `\bigr`, etc.
    Big,
    /// Corresponds to `\Bigl`, `\Bigr`, etc.
    BIG,
    /// Corresponds to `\biggl`, `\biggr`, etc.
    Bigg,
    /// Corresponds to `\Biggl`, `\Biggr`, etc.
    BIGG,
}

impl DelimiterSize {
    pub fn to_em(&self) -> f32 {
        match self {
            DelimiterSize::Big => 1.2,
            DelimiterSize::BIG => 1.8,
            DelimiterSize::Bigg => 2.4,
            DelimiterSize::BIGG => 3.,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DelimiterType {
    /// Corresponds to the left delimiter.
    Open,
    /// Corresponds to a delimiter that is neither an opening nor a closing delimiter.
    Fence,
    /// Corresponds to the right delimiter.
    Close,
}
