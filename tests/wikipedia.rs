use std::{fs::File, path::Path};

use common::{html_template, tabled, OUTPUT_DIR};

mod common;

fn main() {
    let concl = common::test();
    if std::env::var("RENDER").as_deref() != Ok("true") {
        concl.exit()
    }

    let mut file = File::create(Path::new(OUTPUT_DIR).join("wikipedia.html")).unwrap();
    html_template(&mut file, "Wikipedia Tests", None, tabled).unwrap();

    concl.exit();
}

// General Stuff

round_trip!(
    basic,
    r"\alpha",
    r"f(x) = x^2",
    r"\{1,e,\pi\}",
    r"|z| \leq 2"
);

round_trip!(
    accents_and_diacritics,
    r"\dot{a}, \ddot{a}, \acute{a}, \grave{a}",
    r"\check{a}, \breve{a}, \tilde{a}, \bar{a}",
    r"\hat{a}, \widehat{a}, \vec{a}"
);

round_trip!(
    standard_numerical_functions,
    r"\exp_a b = a^b, \exp b = e^b, 10^m",
    r"\ln c = \log c, \lg d = \log_{10} d",
    r"\sin a, \cos b, \tan c, \cot d, \sec f, \csc g",
    r"\arcsin h, \arccos i, \arctan j",
    r"\sinh k, \cosh l, \tanh m, \coth n",
    r"\operatorname{sh}k, \operatorname{ch}l, \operatorname{th}m, \operatorname{coth}n",
    r"\operatorname{argsh}o, \operatorname{argch}p, \operatorname{argth}q",
    r"\sgn r, \left\vert s \right\vert",
    r"\min(x,y), \max(x,y)"
);

round_trip!(
    bounds,
    r"\min x, \max y, \inf s, \sup t",
    r"\lim u, \liminf v, \limsup w",
    r"\dim p, \deg q, \det m, \ker\phi"
);

round_trip!(projections, r"\Pr j, \hom l, \lVert z \rVert, \arg z");

round_trip!(
    differential_and_derivatives,
    r"dt, \mathrm{d}t, \partial t, \nabla\psi",
    r"dy/dx, \mathrm{d}y/\mathrm{d}x, \frac{dy}{dx}, \frac{\mathrm{d}y}{\mathrm{d}x}",
    r"\frac{\partial^2}{\partial x_1\partial x_2}y, \left.\frac{\partial^3 f}{\partial^2 x \partial y}\right\vert_{p_0}",
    r"\prime, \backprime, f^\prime, f', f'', f^{(3)}, \dot y, \ddot y"
);
round_trip!(
    letter_like_symbols_or_constants,
    r"\infty, \aleph, \complement, \backepsilon, \eth, \Finv, \hbar",
    r"\Im, \imath, \jmath, \Bbbk, \ell, \mho, \wp, \Re, \circledS, \S, \P"
);

round_trip!(
    modular_arithmetic,
    r"s_k \equiv 0 \pmod{m}",
    r"a \bmod b",
    r"\gcd(m, n), \operatorname{lcm}(m, n)",
    r"\mid, \nmid, \shortmid, \nshortmid"
);

round_trip!(
    radicals,
    r"\surd, \sqrt{2}, \sqrt[n]{2}, \sqrt[3]{\frac{x^3+y^3}{2}}"
);

round_trip!(
    operators,
    r"+, -, \pm, \mp, \dotplus",
    r"\times, \div, \divideontimes, /, \backslash",
    r"\cdot, * \ast, \star, \circ, \bullet",
    r"\boxplus, \boxminus, \boxtimes, \boxdot",
    r"\oplus, \ominus, \otimes, \oslash, \odot",
    r"\circleddash, \circledcirc, \circledast",
    r"\bigoplus, \bigotimes, \bigodot"
);

round_trip!(
    sets,
    r"\{ \}, \emptyset, \varnothing",
    r"\in, \notin \not\in, \ni, \not\ni",
    r"\cap, \Cap, \sqcap, \bigcap",
    r"\cup, \Cup, \sqcup, \bigcup, \bigsqcup, \uplus, \biguplus",
    r"\setminus, \smallsetminus, \times",
    r"\subset, \Subset, \sqsubset",
    r"\supset, \Supset, \sqsupset",
    r"\subseteq, \nsubseteq, \subsetneq, \varsubsetneq, \sqsubseteq",
    r"\supseteq, \nsupseteq, \supsetneq, \varsupsetneq, \sqsupseteq",
    r"\subseteqq, \nsubseteqq, \subsetneqq, \varsubsetneqq",
    r"\supseteqq, \nsupseteqq, \supsetneqq, \varsupsetneqq"
);

round_trip!(
    relations,
    r"=, \ne, \neq, \equiv, \not\equiv",
    r"\doteq, \doteqdot, \overset{\underset{\mathrm{def}}{}}{=}, :=",
    r"\sim, \nsim, \backsim, \thicksim, \simeq, \backsimeq, \eqsim, \cong, \ncong",
    r"\approx, \thickapprox, \approxeq, \asymp, \propto, \varpropto",
    r"<, \nless, \ll, \not\ll, \lll, \not\lll, \lessdot",
    r">, \ngtr, \gg, \not\gg, \ggg, \not\ggg, \gtrdot",
    r"\le, \leq, \lneq, \leqq, \nleq, \nleqq, \lneqq, \lvertneqq",
    r"\ge, \geq, \gneq, \geqq, \ngeq, \ngeqq, \gneqq, \gvertneqq",
    r"\lessgtr, \lesseqgtr, \lesseqqgtr, \gtrless, \gtreqless, \gtreqqless",
    r"\leqslant, \nleqslant, \eqslantless",
    r"\geqslant, \ngeqslant, \eqslantgtr",
    r"\lesssim, \lnsim, \lessapprox, \lnapprox",
    r"\gtrsim, \gnsim, \gtrapprox, \gnapprox",
    r"\prec, \nprec, \preceq, \npreceq, \precneqq",
    r"\succ, \nsucc, \succeq, \nsucceq, \succneqq",
    r"\preccurlyeq, \curlyeqprec",
    r"\succcurlyeq, \curlyeqsucc",
    r"\precsim, \precnsim, \precapprox, \precnapprox",
    r"\succsim, \succnsim, \succapprox, \succnapprox"
);

round_trip!(
    geometric,
    r"\parallel, \nparallel, \shortparallel, \nshortparallel",
    r"\perp, \angle, \sphericalangle, \measuredangle, 45^\circ",
    r"\Box, \square, \blacksquare, \diamond, \Diamond, \lozenge, \blacklozenge, \bigstar",
    r"\bigcirc, \triangle, \bigtriangleup, \bigtriangledown",
    r"\vartriangle, \triangledown",
    r"\blacktriangle, \blacktriangledown, \blacktriangleleft, \blacktriangleright"
);

round_trip!(
    logic,
    r"\forall, \exists, \nexists",
    r"\therefore, \because, \And",
    r"\lor, \vee, \curlyvee, \bigvee",
    r"\land, \wedge, \curlywedge, \bigwedge",
    r"\bar{q}, \bar{abc}, \overline{q}, \overline{abc}",
    r"\lnot, \neg, \not\operatorname{R}, \bot, \to",
    r"\vdash, \dashv, \vDash, \Vdash, \models",
    r"\Vvdash, \nvdash, \nVdash, \nvDash, \nVDash",
    r"\ulcorner, \urcorner, \llcorner, \lrcorner"
);

round_trip!(
    arrows,
    r"\Rrightarrow, \Lleftarrow",
    r"\Rightarrow, \nRightarrow, \Longrightarrow, \implies",
    r"\Leftarrow, \nLeftarrow, \Longleftarrow",
    r"\Leftrightarrow, \nLeftrightarrow, \Longleftrightarrow, \iff",
    r"\Uparrow, \Downarrow, \Updownarrow",
    r"\rightarrow, \to, \nrightarrow, \longrightarrow",
    r"\leftarrow, \gets, \nleftarrow, \longleftarrow",
    r"\leftrightarrow, \nleftrightarrow, \longleftrightarrow",
    r"\uparrow, \downarrow, \updownarrow",
    r"\nearrow, \swarrow, \nwarrow, \searrow",
    r"\mapsto, \longmapsto",
    r"\rightharpoonup \rightharpoondown \leftharpoonup \leftharpoondown \upharpoonleft \upharpoonright \downharpoonleft \downharpoonright \rightleftharpoons \leftrightharpoons",
    r"\curvearrowleft \circlearrowleft \Lsh \upuparrows \rightrightarrows \rightleftarrows \rightarrowtail \looparrowright",
    r"\curvearrowright \circlearrowright \Rsh \downdownarrows \leftleftarrows \leftrightarrows \leftarrowtail \looparrowleft",
    r"\hookrightarrow \hookleftarrow \multimap \leftrightsquigarrow \rightsquigarrow \twoheadrightarrow \twoheadleftarrow"
);

round_trip!(
    special,
    r"\amalg \P \S \% \dagger \ddagger \ldots \cdots \vdots \ddots",
    r"\smile \frown \wr \triangleleft \triangleright",
    r"\diamondsuit, \heartsuit, \clubsuit, \spadesuit, \Game, \flat, \natural, \sharp"
);

round_trip!(
    unsorted,
    r"\diagup \diagdown \centerdot \ltimes \rtimes \leftthreetimes \rightthreetimes",
    r"\eqcirc \circeq \triangleq \bumpeq \Bumpeq \doteqdot \risingdotseq \fallingdotseq",
    r"\intercal \barwedge \veebar \doublebarwedge \between \pitchfork",
    r"\vartriangleleft \ntriangleleft \vartriangleright \ntriangleright",
    r"\trianglelefteq \ntrianglelefteq \trianglerighteq \ntrianglerighteq"
);

round_trip!(
    should_panic,
    unsupported,
    r"\N \R \Z \C \Q",
    r"\AA",
    r"\O \empty"
);

// Larger Expressions

round_trip!(superscript, r"a^2, a^{x+3}");

round_trip!(subscript, r"a_2");

round_trip!(grouping, r"10^{30} a^{2+2}", r"a_{i,j} b_{f'}");

round_trip!(combined_sub_superscript, r"x_2^3", r"{x_2}^3");

round_trip!(super_super, r"10^{10^{8}}");

round_trip!(
    preceding_and_or_additional,
//     r"\sideset{_1^2}{_3^4}\prod_a^b",
    r"{}_1^2\!\Omega_3^4"
);

round_trip!(
    stacking,
    r"\overset{\alpha}{\omega}",
    r"\underset{\alpha}{\omega}",
    r"\overset{\alpha}{\underset{\gamma}{\omega}}",
    r"\stackrel{\alpha}{\omega}"
);

round_trip!(
    derivatives,
    r"x', y'', f', f''",
    r"x^\prime, y^{\prime\prime}"
);

round_trip!(derivative_dots, r"\dot{x}, \ddot{x}");

round_trip!(
    underline_overline_vectors,
    r"\hat a \ \bar b \ \vec c",
    r"\overrightarrow{a b} \ \overleftarrow{c d} \ \widehat{d e f}",
    r"\overline{g h i} \ \underline{j k l}"
);

round_trip!(arc, r"\overset{\frown} {AB}");

round_trip!(
    arrows_example,
    r"A \xleftarrow{n+\mu-1} B \xrightarrow[T]{n\pm i-1} C"
);

round_trip!(
    overbraces,
    r"\overbrace{ 1+2+\cdots+100 }^{5050}",
    r"\underbrace{ a+b+\cdots+z }_{26}"
);

round_trip!(sum, r"\sum_{k=1}^N k^2", r"\textstyle \sum_{k=1}^N k^2");

round_trip!(
    sum_in_fraction,
    r"\frac{\sum_{k=1}^N k^2}{a}",
    r"\frac{\displaystyle \sum_{k=1}^N k^2}{a}",
    r"\frac{\sum\limits^{N}_{k=1} k^2}{a}"
);

round_trip!(
    product,
    r"\prod_{i=1}^N x_i",
    r"\textstyle \prod_{i=1}^N x_i"
);

round_trip!(
    coproduct,
    r"\coprod_{i=1}^N x_i",
    r"\textstyle \coprod_{i=1}^N x_i"
);

round_trip!(
    limit,
    r"\lim_{x \to \infty} x_n",
    r"\textstyle \lim_{x \to \infty} x_n"
);

round_trip!(
    integral,
    r"\int\limits_{1}^{3}\frac{e^3/x}{x^2}\, dx",
    r"\int_{1}^{3}\frac{e^3/x}{x^2}\, dx",
    r"\textstyle \int\limits_{-N}^{N} e^x dx",
    r"\textstyle \int_{-N}^{N} e^x dx"
);

round_trip!(double_integral, r"\iint\limits_{D} dx\,dy");

round_trip!(triple_integral, r"\iiint\limits_{D} dx\,dy\,dz");

round_trip!(quadruple_integral, r"\iiiint\limits_{D} dx\,dy\,dz\,dt");

round_trip!(
    line_or_path_integral,
    r"\int_{(x,y)\in C} x^3\, dx + 4y^2\, dy"
);

round_trip!(
    closed_line_or_path_integral,
    r"\oint_{(x,y)\in C} x^3\, dx + 4y^2\, dy"
);

round_trip!(intersections, r"\bigcap_{i=1}^n E_i");

round_trip!(unions, r"\bigcup_{i=1}^n E_i");

// Fractions, Matrices, Multilines

round_trip!(
    fractions,
    r"\frac{2}{4} = 0.5",
    r"\tfrac{2}{4} = 0.5",
    r"\dfrac{2}{4} = 0.5 \qquad \dfrac{2}{c + \dfrac{2}{d + \dfrac{2}{4}}} = a",
    r"\cfrac{2}{c + \cfrac{2}{d + \cfrac{2}{4}}} = a",
    r"\cfrac{x}{1 + \cfrac{\cancel{y}}{\cancel{y}}} = \cfrac{x}{2}"
);

round_trip!(
    binomials,
    r"\binom{n}{k}",
    r"\tbinom{n}{k}",
    r"\dbinom{n}{k}"
);

round_trip!(
    matrices,
    r"\begin{matrix}
    -x & y \\
    z & -v
    \end{matrix}",
    r"\begin{vmatrix}
    -x & y \\
    z & -v
    \end{vmatrix}",
    r"\begin{Vmatrix}
    -x & y \\
    z & -v
    \end{Vmatrix}",
    r"\begin{bmatrix}
    0 & \cdots & 0 \\
    \vdots & \ddots & \vdots \\
    0 & \cdots & 0
    \end{bmatrix}",
    r"\begin{Bmatrix}
    x & y \\
    z & v
    \end{Bmatrix}",
    r"\begin{pmatrix}
    x & y \\
    z & v
    \end{pmatrix}",
    r"\bigl( \begin{smallmatrix}
    a&b\\ c&d
    \end{smallmatrix} \bigr)"
);

round_trip!(
    cases,
    r"f(n) =
    \begin{cases}
    n/2, & \text{if }n\text{ is even} \\
    3n+1, & \text{if }n\text{ is odd}
    \end{cases}",
    r"\begin{cases}
    3x + 5y + z \\
    7x - 2y + 4z \\
    -6x + 3y + 2z
    \end{cases}"
);

round_trip!(
    multiline_equations,
    r"\begin{align}
    f(x) & = (a+b)^2 \\
    & = a^2+2ab+b^2 \\
    \end{align}",
//     r"\begin{alignat}{2}
//     f(x) & = (a-b)^2 \\
//     & = a^2-2ab+b^2 \\
//     \end{alignat}",
    r"\begin{align}
    f(a,b) & = (a+b)^2 && = (a+b)(a+b) \\
    & = a^2+ab+ba+b^2  && = a^2+2ab+b^2 \\
    \end{align}",
//     r"\begin{alignat}{3}
//     f(a,b) & = (a+b)^2 && = (a+b)(a+b) \\
//     & = a^2+ab+ba+b^2  && = a^2+2ab+b^2 \\
//     \end{alignat}",
    r"\begin{array}{lcl}
    z & = & a \\
    f(x,y,z) & = & x + y + z
    \end{array}",
    r"\begin{array}{lcr}
    z & = & a \\
    f(x,y,z) & = & x + y + z
    \end{array}"
//     r"\begin{alignat}{4}
//     F:\; && C(X) && \;\to\;     & C(X) \\
//          && g    && \;\mapsto\; & g^2
//     \end{alignat}",
//     r"\begin{alignat}{4}
//     F:\; && C(X) && \;\to\;     && C(X) \\
//          && g    && \;\mapsto\; && g^2
//     \end{alignat}"
);

// round_trip!(
//     arrays,
//     r"\begin{array}{|c|c|c|} a & b & S \\
//     \hline
//     0 & 0 & 1 \\
//     0 & 1 & 1 \\
//     1 & 0 & 1 \\
//     1 & 1 & 0 \\
//     \end{array}"
// );

// Delimiters

round_trip!(parentheses, r"\left ( \frac{a}{b} \right )");

round_trip!(
    brackets,
    r"\left [ \frac{a}{b} \right ]",
    r"\left \lbrack \frac{a}{b} \right \rbrack"
);

round_trip!(
    braces,
    r"\left \{ \frac{a}{b} \right \}",
    r"\left \lbrace \frac{a}{b} \right \rbrace"
);

round_trip!(angle_brackets, r"\left \langle \frac{a}{b} \right \rangle");

round_trip!(
    bars_and_double_bars,
    r"\left | \frac{a}{b} \right \vert",
    r"\left \| \frac{a}{b} \right \Vert"
);

round_trip!(
    floor_and_ceiling,
    r"\left \lfloor \frac{a}{b} \right \rfloor",
    r"\left \lceil \frac{a}{b} \right \rceil"
);

round_trip!(
    slashes_and_backslashes,
    r"\left / \frac{a}{b} \right \backslash"
);

round_trip!(
    up_down_updown_arrows,
    r"\left \uparrow \frac{a}{b} \right \downarrow",
    r"\left \Uparrow \frac{a}{b} \right \Downarrow",
    r"\left \updownarrow \frac{a}{b} \right \Updownarrow"
);

round_trip!(
    mixed,
    r"\left [ 0,1 \right )",
    r"\left \langle \psi \right |"
);

round_trip!(no_delimiter, r"\left . \frac{A}{B} \right \} \to X");

round_trip!(
    delimiter_sizes,
    r"( \bigl( \Bigl( \biggl( \Biggl( \dots \Biggr] \biggr] \Bigr] \bigr] ]",
    r"\{ \bigl\{ \Bigl\{ \biggl\{ \Biggl\{ \dots \Biggr\rangle \biggr\rangle \Bigr\rangle \bigr\rangle \rangle",
    r"\| \big\| \Big\| \bigg\| \Bigg\| \dots \Bigg| \bigg| \Big| \big| |",
    r"\lfloor \bigl\lfloor \Bigl\lfloor \biggl\lfloor \Biggl\lfloor \dots \Biggr\rceil \biggr\rceil \Bigr\rceil \bigr\rceil \rceil",
    r"\uparrow \big\uparrow \Big\uparrow \bigg\uparrow \Bigg\uparrow \dots \Bigg\Downarrow \bigg\Downarrow \Big\Downarrow \big\Downarrow \Downarrow",
    r"\updownarrow \big\updownarrow \Big\updownarrow \bigg\updownarrow \Bigg\updownarrow \dots \Bigg\Updownarrow \bigg\Updownarrow \Big\Updownarrow \big\Updownarrow \Updownarrow",
    r"/ \big/ \Big/ \bigg/ \Bigg/ \dots \Bigg\backslash \bigg\backslash \Big\backslash \big\backslash \backslash"
);

// Fonts

round_trip!(
    greek_alphabet,
    r"\Alpha \Beta \Gamma \Delta \Epsilon \Zeta \Eta \Theta",
    r"\Iota \Kappa \Lambda \Mu \Nu \Xi \Omicron \Pi",
    r"\Rho \Sigma \Tau \Upsilon \Phi \Chi \Psi \Omega",
    r"\alpha \beta \gamma \delta \epsilon \zeta \eta \theta",
    r"\iota \kappa \lambda \mu \nu \xi \omicron \pi",
    r"\rho \sigma \tau \upsilon \phi \chi \psi \omega",
    r"\varGamma \varDelta \varTheta \varLambda \varXi \varPi \varSigma \varPhi \varUpsilon \varOmega",
    r"\varepsilon \digamma \varkappa \varpi \varrho \varsigma \vartheta \varphi"
);

round_trip!(hebrew_symbols, r"\aleph \beth \gimel \daleth");

round_trip!(
    blackboard_bold,
    r"\mathbb{ABCDEFGHI}",
    r"\mathbb{JKLMNOPQR}",
    r"\mathbb{STUVWXYZ}"
);

round_trip!(
    boldface,
    r"\mathbf{ABCDEFGHI}",
    r"\mathbf{JKLMNOPQR}",
    r"\mathbf{STUVWXYZ}",
    r"\mathbf{abcdefghijklm}",
    r"\mathbf{nopqrstuvwxyz}",
    r"\mathbf{0123456789}"
);

round_trip!(
    boldface_greek,
    r"\boldsymbol{\Alpha \Beta \Gamma \Delta \Epsilon \Zeta \Eta \Theta}",
    r"\boldsymbol{\Iota \Kappa \Lambda \Mu \Nu \Xi \Omicron \Pi}",
    r"\boldsymbol{\Rho \Sigma \Tau \Upsilon \Phi \Chi \Psi \Omega}",
    r"\boldsymbol{\alpha \beta \gamma \delta \epsilon \zeta \eta \theta}",
    r"\boldsymbol{\iota \kappa \lambda \mu \nu \xi \omicron \pi}",
    r"\boldsymbol{\rho \sigma \tau \upsilon \phi \chi \psi \omega}",
    r"\boldsymbol{\varepsilon\digamma\varkappa\varpi}",
    r"\boldsymbol{\varrho\varsigma\vartheta\varphi}"
);

round_trip!(italics, r"\mathit{0123456789}");

round_trip!(
    greek_italics,
    r"\mathit{\Alpha \Beta \Gamma \Delta \Epsilon \Zeta \Eta \Theta}",
    r"\mathit{\Iota \Kappa \Lambda \Mu \Nu \Xi \Omicron \Pi}",
    r"\mathit{\Rho \Sigma \Tau \Upsilon \Phi \Chi \Psi \Omega}"
);

round_trip!(
    greek_uppercase_boldface_italics,
    r"\boldsymbol{\varGamma \varDelta \varTheta \varLambda}",
    r"\boldsymbol{\varXi \varPi \varSigma \varUpsilon \varOmega}"
);

round_trip!(
    roman_typeface,
    r"\mathrm{ABCDEFGHI}",
    r"\mathrm{JKLMNOPQR}",
    r"\mathrm{STUVWXYZ}",
    r"\mathrm{abcdefghijklm}",
    r"\mathrm{nopqrstuvwxyz}",
    r"\mathrm{0123456789}"
);

round_trip!(
    sans_serif,
    r"\mathsf{ABCDEFGHI}",
    r"\mathsf{JKLMNOPQR}",
    r"\mathsf{STUVWXYZ}",
    r"\mathsf{abcdefghijklm}",
    r"\mathsf{nopqrstuvwxyz}",
    r"\mathsf{0123456789}"
);

round_trip!(
    sans_serif_greek,
    r"\mathsf{\Alpha \Beta \Gamma \Delta \Epsilon \Zeta \Eta \Theta}",
    r"\mathsf{\Iota \Kappa \Lambda \Mu \Nu \Xi \Omicron \Pi}",
    r"\mathsf{\Rho \Sigma \Tau \Upsilon \Phi \Chi \Psi \Omega}"
);

round_trip!(
    calligraphiy,
    r"\mathcal{ABCDEFGHI}",
    r"\mathcal{JKLMNOPQR}",
    r"\mathcal{STUVWXYZ}",
    r"\mathcal{abcdefghi}",
    r"\mathcal{jklmnopqr}",
    r"\mathcal{stuvwxyz}"
);

round_trip!(
    fraktur,
    r"\mathfrak{ABCDEFGHI}",
    r"\mathfrak{JKLMNOPQR}",
    r"\mathfrak{STUVWXYZ}",
    r"\mathfrak{abcdefghijklm}",
    r"\mathfrak{nopqrstuvwxyz}",
    r"\mathfrak{0123456789}"
);

round_trip!(small_script, r"{\scriptstyle\text{abcdefghijklm}}");

round_trip!(
    mixed_faces,
    r"x y z",
    r"\text{x y z}",
    r"\text{if} n \text{is even}",
    r"\text{if }n\text{ is even}",
    r"\text{if}~n\ \text{is even}"
);

// Color

// round_trip!(
//     color,
//     r"{\color{Blue}x^2}+{\color{Orange}2x}-{\color{LimeGreen}1}",
//     r"x=\frac{{\color{Blue}-b}\pm\sqrt{\color{Red}b^2-4ac}}{\color{Green}2a}",
//     r"x\color{red}\neq y=z",
//     r"x{\color{red}\neq} y=z",
//     r"x\color{red}\neq\color{black} y=z",
//     r"\frac{-b\color{Green}\pm\sqrt{b^2\color{Blue}-4{\color{Red}a}c}}{2a}=x",
//     r"{\color{Blue}x^2}+{\color{Orange}2x}-{\color{LimeGreen}1}",
//     r"\color{Blue}x^2\color{Black}+\color{Orange}2x\color{Black}-\color{LimeGreen}1"
// );

// Examples

round_trip!(quadratic_polynomial, r"ax^2 + bx + c = 0");

round_trip!(quadratic_formula, r"x = \frac{-b\pm\sqrt{b^2-4ac}}{2a}");

round_trip!(
    tall_parentheses_and_fractions,
    r"2 = \left( \frac{\left(3-x\right) \times 2}{3-x} \right)",
    r"S_{\text{new}} = S_{\text{old}} - \frac{ \left( 5-T \right) ^2} {2}",
    r"\phi_n(\kappa) = 0.033C_n^2\kappa^{-11/3},\quad \frac{1}{L_0}\ll\kappa\ll\frac{1}{l_0}"
);

round_trip!(
    integrals,
    r"\int_a^x \int_a^s f(y)\,dy\,ds = \int_a^x f(y)(x-y)\,dy",
    r"\int_e^{\infty}\frac {1}{t(\ln t)^2}dt = \left. \frac{-1}{\ln t} \right\vert_e^\infty = 1"
);

round_trip!(
    matrices_and_determinants,
    r"\det(\mathsf{A}-\lambda\mathsf{I}) = 0"
);

round_trip!(
    summation,
    r"\sum_{i=0}^{n-1} i",
    r"\sum_{m=1}^\infty\sum_{n=1}^\infty\frac{m^2 n}{3^m\left(m 3^n + n 3^m\right)}"
);

round_trip!(
    differential_equations,
    r"u'' + p(x)u' + q(x)u=f(x),\quad x>a"
);

round_trip!(
    complex_numbers,
    r"|\bar{z}| = |z|,
    |(\bar{z})^n| = |z|^n,
    \arg(z^n) = n \arg(z)"
);

round_trip!(limits, r"\lim_{z\to z_0} f(z)=f(z_0)");

round_trip!(
    integral_equation,
    r"\phi_n(\kappa) =
    \frac{1}{4\pi^2\kappa^2} \int_0^\infty
    \frac{\sin(\kappa R)}{\kappa R}
    \frac{\partial}{\partial R}
    \left [ R^2\frac{\partial D_n(R)}{\partial R} \right ] \,dR"
);

round_trip!(
    continuation_and_cases,
    r"f(x) =
      \begin{cases}
        1 & -1 \le x < 0 \\
        \frac{1}{2} & x = 0 \\
        1 - x^2 & \text{otherwise}
      \end{cases}"
);

round_trip!(
    prefixed_subscript,
    r"{}_pF_q(a_1,\dots,a_p;c_1,\dots,c_q;z)
    = \sum_{n=0}^\infty
    \frac{(a_1)_n\cdots(a_p)_n}{(c_1)_n\cdots(c_q)_n}
    \frac{z^n}{n!}"
);

round_trip!(fraction_and_small_fraction, r"\frac{a}{b}\ \tfrac{a}{b}");

round_trip!(area_of_quadrilateral, r"S=dD\sin\alpha");

round_trip!(
    volume_of_sphere_stand,
    r"V = \frac{1}{6} \pi h \left [ 3 \left ( r_1^2 + r_2^2 \right ) + h^2 \right ]"
);

round_trip!(
    multiple_equations,
    r"\begin{align}
    u & = \tfrac{1}{\sqrt{2}}(x+y) \qquad & x &= \tfrac{1}{\sqrt{2}}(u+v) \\[0.6ex]
    v & = \tfrac{1}{\sqrt{2}}(x-y) \qquad & y &= \tfrac{1}{\sqrt{2}}(u-v)
    \end{align}"
);
