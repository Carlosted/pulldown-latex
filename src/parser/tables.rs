// TODO: Consider using `phf`s for this

use crate::event::DelimiterType;

use super::Token;

pub fn is_binary(c: char) -> bool {
    matches!(
        c,
        '\u{002B}' | '\u{002F}' | '\u{00B1}' | '\u{005C}' | '\u{00B7}' | '\u{00D7}' | '\u{00F7}' | '\u{2022}' |
        '\u{2040}' | '\u{2044}' | '\u{204E}' | '\u{2061}' | '\u{2062}' | '\u{2213}' | '\u{2214}' | '\u{2215}' |
        '\u{2216}' | '\u{2217}' | '\u{2218}' | '\u{2219}' | '\u{2227}' | '\u{2228}' | '\u{2229}' | '\u{222A}' |
        '\u{2238}' | '\u{223E}' | '\u{2240}' | '\u{228C}' | '\u{228D}' | '\u{228E}' | '\u{2293}' | '\u{2294}' |
        '\u{2295}' | '\u{2296}' | '\u{2297}' | '\u{2298}' | '\u{2299}' | '\u{229A}' | '\u{229B}' | '\u{229C}' |
        '\u{229D}' | '\u{229E}' | '\u{229F}' | '\u{22A0}' | '\u{22A1}' | '\u{22B9}' | '\u{22BA}' | '\u{22BB}' |
        '\u{22BC}' | '\u{22BD}' | '\u{22C4}' | '\u{22C5}' | '\u{22C6}' | '\u{22C7}' | '\u{22C9}' | '\u{22CA}' |
        '\u{22CB}' | '\u{22CC}' | '\u{22CE}' | '\u{22CF}' | '\u{22D2}' | '\u{22D3}' | '\u{2305}' | '\u{2306}' |
        '\u{233D}' | '\u{25B2}' | '\u{25B3}' | '\u{25B4}' | '\u{25B5}' | '\u{25B6}' | '\u{25B7}' | '\u{25B8}' |
        '\u{25B9}' | '\u{25BC}' | '\u{25BD}' | '\u{25BE}' | '\u{25BF}' | '\u{25C0}' | '\u{25C1}' | '\u{25C2}' |
        '\u{25C3}' | '\u{25C4}' | '\u{25C5}' | '\u{25CA}' | '\u{25CB}' | '\u{25E6}' | '\u{25EB}' | '\u{25EC}' |
        '\u{25F8}' | '\u{25F9}' | '\u{25FA}' | '\u{25FB}' | '\u{25FC}' | '\u{25FD}' | '\u{25FE}' | '\u{25FF}' |
        '\u{2605}' | '\u{2606}' | '\u{27CE}' | '\u{27CF}' | '\u{27D1}' | '\u{27E0}' | '\u{27E1}' | '\u{27E2}' |
        '\u{27E3}' | '\u{29B6}' | '\u{29B7}' | '\u{29B8}' | '\u{29B9}' | '\u{29C0}' | '\u{29C1}' | '\u{29C4}' |
        '\u{29C5}' | '\u{29C6}' | '\u{29C7}' | '\u{29C8}' | '\u{27E4}' | '\u{27E5}' | '\u{29D6}' | '\u{29D7}' |
        '\u{29E2}' | '\u{29EB}' | '\u{29F5}' | '\u{29F6}' | '\u{29F7}' | '\u{29FA}' | '\u{29FB}' | '\u{29FE}' |
        '\u{29FF}' | '\u{2A22}' | '\u{2A23}' | '\u{2A24}' | '\u{2A25}' | '\u{2A26}' | '\u{2A27}' | '\u{2A28}' |
        '\u{2A29}' | '\u{2A2A}' | '\u{2A2B}' | '\u{2A2C}' | '\u{2A2D}' | '\u{2A2E}' | '\u{2A2F}' | '\u{2A30}' |
        '\u{2A31}' | '\u{2A32}' | '\u{2A33}' | '\u{2A34}' | '\u{2A35}' | '\u{2A36}' | '\u{2A37}' | '\u{2A38}' |
        '\u{2A39}' | '\u{2A3A}' | '\u{2A3B}' | '\u{2A3C}' | '\u{2A3D}' | '\u{2A3E}' | '\u{2A3F}' | '\u{2A40}' |
        '\u{2A41}' | '\u{2A42}' | '\u{2A43}' | '\u{2A44}' | '\u{2A45}' | '\u{2A46}' | '\u{2A47}' | '\u{2A48}' |
        '\u{2A49}' | '\u{2A4A}' | '\u{2A4B}' | '\u{2A4C}' | '\u{2A4D}' | '\u{2A4E}' | '\u{2A4F}' | '\u{2A50}' |
        '\u{2A51}' | '\u{2A52}' | '\u{2A53}' | '\u{2A54}' | '\u{2A55}' | '\u{2A56}' | '\u{2A57}' | '\u{2A58}' |
        '\u{2A71}' | '\u{2A72}' | '\u{2AF4}' | '\u{2AF5}' | '\u{2AF6}' | '\u{2AFB}' | '\u{2AFD}' | '\u{2AFE}' 
    )
}

pub fn is_relation(c: char) -> bool {
    matches!(
        c,
        '\u{003A}' | '\u{003C}' | '\u{003D}' | '\u{003E}' | '\u{2020}' | '\u{2021}' | '\u{204F}' | '\u{2050}' |
        '\u{2190}'..='\u{21B3}' | '\u{21B6}' | '\u{21B7}' | '\u{21BA}'..='\u{21FF}' | '\u{2208}' | '\u{2209}' |
        '\u{220A}' | '\u{220B}' | '\u{220C}' | '\u{220D}' | '\u{221D}' | '\u{2223}' | '\u{2224}' | '\u{2225}' |
        '\u{2226}' | '\u{2234}' | '\u{2235}' | '\u{2236}' | '\u{2237}' | '\u{2239}' | '\u{223A}' | '\u{223B}' |
        '\u{223C}' | '\u{223D}' | '\u{2241}' | '\u{2242}' | '\u{2243}' | '\u{2244}' | '\u{2245}' | '\u{2246}' |
        '\u{2247}' | '\u{2248}' | '\u{2249}' | '\u{224A}' | '\u{224B}' | '\u{224C}' | '\u{224D}' | '\u{224E}' |
        '\u{224F}' | '\u{2250}' | '\u{2251}' | '\u{2252}' | '\u{2253}' | '\u{2254}' | '\u{2255}' | '\u{2256}' |
        '\u{2257}' | '\u{2258}' | '\u{2259}' | '\u{225A}' | '\u{225B}' | '\u{225C}' | '\u{225D}' | '\u{225E}' |
        '\u{225F}' | '\u{2260}' | '\u{2261}' | '\u{2262}' | '\u{2263}' | '\u{2264}' | '\u{2265}' | '\u{2266}' |
        '\u{2267}' | '\u{2268}' | '\u{2269}' | '\u{226A}' | '\u{226B}' | '\u{226C}' | '\u{226D}' | '\u{226E}' |
        '\u{226F}' | '\u{2270}' | '\u{2271}' | '\u{2272}' | '\u{2273}' | '\u{2274}' | '\u{2275}' | '\u{2276}' |
        '\u{2277}' | '\u{2278}' | '\u{2279}' | '\u{227A}' | '\u{227B}' | '\u{227C}' | '\u{227D}' | '\u{227E}' |
        '\u{227F}' | '\u{2280}' | '\u{2281}' | '\u{2282}' | '\u{2283}' | '\u{2284}' | '\u{2285}' | '\u{2286}' |
        '\u{2287}' | '\u{2288}' | '\u{2289}' | '\u{228A}' | '\u{228B}' | '\u{228F}' | '\u{2290}' | '\u{2291}' |
        '\u{2292}' | '\u{22A2}' | '\u{22A3}' | '\u{22A5}' | '\u{22A6}' | '\u{22A7}' | '\u{22A8}' | '\u{22A9}' |
        '\u{22AA}' | '\u{22AB}' | '\u{22AC}' | '\u{22AD}' | '\u{22AE}' | '\u{22AF}' | '\u{22B0}' | '\u{22B1}' |
        '\u{22B2}' | '\u{22B3}' | '\u{22B4}' | '\u{22B5}' | '\u{22B6}' | '\u{22B7}' | '\u{22B8}' | '\u{22C8}' |
        '\u{22CD}' | '\u{22D0}' | '\u{22D1}' | '\u{22D4}' | '\u{22D5}' | '\u{22D6}' | '\u{22D7}' | '\u{22D8}' |
        '\u{22D9}' | '\u{22DA}' | '\u{22DB}' | '\u{22DC}' | '\u{22DD}' | '\u{22DE}' | '\u{22DF}' | '\u{22E0}' |
        '\u{22E1}' | '\u{22E2}' | '\u{22E3}' | '\u{22E4}' | '\u{22E5}' | '\u{22E6}' | '\u{22E7}' | '\u{22E8}' |
        '\u{22E9}' | '\u{22EA}' | '\u{22EB}' | '\u{22EC}' | '\u{22ED}' | '\u{22EE}' | '\u{22EF}' | '\u{22F0}' |
        '\u{22F1}' | '\u{22F2}' | '\u{22F3}' | '\u{22F4}' | '\u{22F5}' | '\u{22F6}' | '\u{22F7}' | '\u{22F8}' |
        '\u{22F9}' | '\u{22FA}' | '\u{22FB}' | '\u{22FC}' | '\u{22FD}' | '\u{22FE}' | '\u{22FF}' | '\u{2322}' |
        '\u{2323}' | '\u{233F}' | '\u{237C}' | '\u{23B0}' | '\u{23B1}' | '\u{27C2}' | '\u{27C3}' | '\u{27C4}' |
        '\u{27C5}' | '\u{27C6}' | '\u{27C7}' | '\u{27C8}' | '\u{27C9}' | '\u{27CA}' | '\u{27CB}' | '\u{27CD}' |
        '\u{27D2}' | '\u{27D3}' | '\u{27D4}' | '\u{27DA}' | '\u{27DB}' | '\u{27DC}' | '\u{27DD}' | '\u{27DE}' |
        '\u{27DF}' | '\u{27F0}' | '\u{27F1}' | '\u{27F2}' | '\u{27F3}' | '\u{27F4}' | '\u{27F5}' | '\u{27F6}' |
        '\u{27F7}' | '\u{27F8}' | '\u{27F9}' | '\u{27FA}' | '\u{27FB}' | '\u{27FC}' | '\u{27FD}' | '\u{27FE}' |
        '\u{27FF}' | '\u{2900}' | '\u{2901}' | '\u{2902}' | '\u{2903}' | '\u{2904}' | '\u{2905}' | '\u{2906}' |
        '\u{2907}' | '\u{2908}' | '\u{2909}' | '\u{290A}' | '\u{290B}' | '\u{290C}' | '\u{290D}' | '\u{290E}' |
        '\u{290F}' | '\u{2910}' | '\u{2911}' | '\u{2912}' | '\u{2913}' | '\u{2914}' | '\u{2915}' | '\u{2916}' |
        '\u{2917}' | '\u{2918}' | '\u{2919}' | '\u{291A}' | '\u{291B}' | '\u{291C}' | '\u{291D}' | '\u{291E}' |
        '\u{291F}' | '\u{2920}' | '\u{2921}' | '\u{2922}' | '\u{2923}' | '\u{2924}' | '\u{2925}' | '\u{2926}' |
        '\u{2927}' | '\u{2928}' | '\u{2929}' | '\u{292A}' | '\u{292B}' | '\u{292C}' | '\u{292D}' | '\u{292E}' |
        '\u{292F}' | '\u{2930}' | '\u{2931}' | '\u{2932}' | '\u{2933}' | '\u{2934}' | '\u{2935}' | '\u{2936}' |
        '\u{2937}' | '\u{2938}' | '\u{2939}' | '\u{293A}' | '\u{293B}' | '\u{293C}' | '\u{293D}' | '\u{293E}' |
        '\u{293F}' | '\u{2940}' | '\u{2941}' | '\u{2942}' | '\u{2943}' | '\u{2944}' | '\u{2945}' | '\u{2946}' |
        '\u{2947}' | '\u{2948}' | '\u{2949}' | '\u{294A}' | '\u{294B}' | '\u{294C}' | '\u{294D}' | '\u{294E}' |
        '\u{294F}' | '\u{2950}' | '\u{2951}' | '\u{2952}' | '\u{2953}' | '\u{2954}' | '\u{2955}' | '\u{2956}' |
        '\u{2957}' | '\u{2958}' | '\u{2959}' | '\u{295A}' | '\u{295B}' | '\u{295C}' | '\u{295D}' | '\u{295E}' |
        '\u{295F}' | '\u{2960}' | '\u{2961}' | '\u{2962}' | '\u{2963}' | '\u{2964}' | '\u{2965}' | '\u{2966}' |
        '\u{2967}' | '\u{2968}' | '\u{2969}' | '\u{296A}' | '\u{296B}' | '\u{296C}' | '\u{296D}' | '\u{296E}' |
        '\u{296F}' | '\u{2970}' | '\u{2971}' | '\u{2972}' | '\u{2973}' | '\u{2974}' | '\u{2975}' | '\u{2976}' |
        '\u{2977}' | '\u{2978}' | '\u{2979}' | '\u{297A}' | '\u{297B}' | '\u{297C}' | '\u{297D}' | '\u{297E}' |
        '\u{297F}' | '\u{29CE}' | '\u{29CF}' | '\u{29D0}' | '\u{29D1}' | '\u{29D2}' | '\u{29D3}' | '\u{29D4}' |
        '\u{29D5}' | '\u{29DF}' | '\u{29E1}' | '\u{29E3}' | '\u{29E4}' | '\u{29E5}' | '\u{29E6}' | '\u{29F4}' |
        '\u{2A59}' | '\u{2A66}' | '\u{2A67}' | '\u{2A68}' | '\u{2A69}' | '\u{2A6A}' | '\u{2A6B}' | '\u{2A6C}' |
        '\u{2A6D}' | '\u{2A6E}' | '\u{2A6F}' | '\u{2A70}' | '\u{2A73}' | '\u{2A74}' | '\u{2A75}' | '\u{2A76}' |
        '\u{2A77}' | '\u{2A78}' | '\u{2A79}' | '\u{2A7A}' | '\u{2A7B}' | '\u{2A7C}' | '\u{2A7D}' | '\u{2A7E}' |
        '\u{2A7F}' | '\u{2A80}' | '\u{2A81}' | '\u{2A82}' | '\u{2A83}' | '\u{2A84}' | '\u{2A85}' | '\u{2A86}' |
        '\u{2A87}' | '\u{2A88}' | '\u{2A89}' | '\u{2A8A}' | '\u{2A8B}' | '\u{2A8C}' | '\u{2A8D}' | '\u{2A8E}' |
        '\u{2A8F}' | '\u{2A90}' | '\u{2A91}' | '\u{2A92}' | '\u{2A93}' | '\u{2A94}' | '\u{2A95}' | '\u{2A96}' |
        '\u{2A97}' | '\u{2A98}' | '\u{2A99}' | '\u{2A9A}' | '\u{2A9B}' | '\u{2A9C}' | '\u{2A9D}' | '\u{2A9E}' |
        '\u{2A9F}' | '\u{2AA0}' | '\u{2AA1}' | '\u{2AA2}' | '\u{2AA3}' | '\u{2AA4}' | '\u{2AA5}' | '\u{2AA6}' |
        '\u{2AA7}' | '\u{2AA8}' | '\u{2AA9}' | '\u{2AAA}' | '\u{2AAB}' | '\u{2AAC}' | '\u{2AAD}' | '\u{2AAE}' |
        '\u{2AAF}' | '\u{2AB0}' | '\u{2AB1}' | '\u{2AB2}' | '\u{2AB3}' | '\u{2AB4}' | '\u{2AB5}' | '\u{2AB6}' |
        '\u{2AB7}' | '\u{2AB8}' | '\u{2AB9}' | '\u{2ABA}' | '\u{2ABB}' | '\u{2ABC}' | '\u{2ABD}' | '\u{2ABE}' |
        '\u{2ABF}' | '\u{2AC0}' | '\u{2AC1}' | '\u{2AC2}' | '\u{2AC3}' | '\u{2AC4}' | '\u{2AC5}' | '\u{2AC6}' |
        '\u{2AC7}' | '\u{2AC8}' | '\u{2AC9}' | '\u{2ACA}' | '\u{2ACB}' | '\u{2ACC}' | '\u{2ACD}' | '\u{2ACE}' |
        '\u{2ACF}' | '\u{2AD0}' | '\u{2AD1}' | '\u{2AD2}' | '\u{2AD3}' | '\u{2AD4}' | '\u{2AD5}' | '\u{2AD6}' |
        '\u{2AD7}' | '\u{2AD8}' | '\u{2AD9}' | '\u{2ADA}' | '\u{2ADB}' | '\u{2ADC}' | '\u{2ADD}' | '\u{2ADE}' |
        '\u{2ADF}' | '\u{2AE0}' | '\u{2AE2}' | '\u{2AE3}' | '\u{2AE4}' | '\u{2AE5}' | '\u{2AE6}' | '\u{2AE7}' |
        '\u{2AE8}' | '\u{2AE9}' | '\u{2AEA}' | '\u{2AEB}' | '\u{2AEC}' | '\u{2AED}' | '\u{2AEE}' | '\u{2AEF}' |
        '\u{2AF0}' | '\u{2AF2}' | '\u{2AF3}' | '\u{2AF7}' | '\u{2AF8}' | '\u{2AF9}' | '\u{2AFA}' | '\u{2B95}' |
        '\u{2B00}'..='\u{2B11}' | '\u{2B30}'..='\u{2B44}' | '\u{2B45}' | '\u{2B46}' | '\u{2B47}'..='\u{2B4C}'
    )
}

// TODO: check if we have missing ones in this:
// F - Fence - unpaired delimiter (often used as opening or closing)
// 007C; 2016; 2980; 2982; 2999; 299A;

// O - Opening - usually paired with closing delimiter
// 0028; 005B; 007B; 301A; 2308; 230A; 231C; 231E; 2329;
// 2772; 27E6; 27E8; 27EA; 27EC; 27EE; 2983; 2985; 2987;
// 2989; 298D; 298F; 2991; 2993; 2995; 2997; 29D8; 29DA;
// 29FC; 3008; 301A;

// C - Closing - usually paired with opening delimiter
// 0029; 005D; 007D; 2309; 230B; 231D; 231F; 232A; 2773;
// 27E7; 27E9; 27EB; 27ED; 27EF; 2984; 2986; 2988; 298A;
// 298C; 298E; 2990; 2992; 2994; 2996; 2998; 29D9; 29DB;
// 29FD; 3009; 301B;


#[rustfmt::skip]
pub fn char_delimiter_map(c: char) -> Option<(char, DelimiterType)> {
    Some(match c {
        '(' => (c, DelimiterType::Open),
        ')' => (c, DelimiterType::Close),
        '⦇' => (c, DelimiterType::Open),
        '⦈' => (c, DelimiterType::Close),
        '⟮' => (c, DelimiterType::Open),
        '⟯' => (c, DelimiterType::Close),
        '[' => (c, DelimiterType::Open),
        ']' => (c, DelimiterType::Close),
        '⟦' => (c, DelimiterType::Open),
        '⟧' => (c, DelimiterType::Close),
        '⦃' => (c, DelimiterType::Open),
        '⦄' => (c, DelimiterType::Close),
        '⟨' => (c, DelimiterType::Open),
        '⟩' => (c, DelimiterType::Close),
        '⟪' => (c, DelimiterType::Open),
        '⟫' => (c, DelimiterType::Close),
        '⦉' => (c, DelimiterType::Open),
        '⦊' => (c, DelimiterType::Close),
        '⌊' => (c, DelimiterType::Open),
        '⌋' => (c, DelimiterType::Close),
        '⌈' => (c, DelimiterType::Open),
        '⌉' => (c, DelimiterType::Close),
        '┌' => (c, DelimiterType::Open),
        '┐' => (c, DelimiterType::Close),
        '└' => (c, DelimiterType::Open),
        '┘' => (c, DelimiterType::Close),
        '⎰' => (c, DelimiterType::Open),
        '⎱' => (c, DelimiterType::Close),
        '|' => (c, DelimiterType::Fence),
        '‖' => (c, DelimiterType::Fence),
        '↑' => (c, DelimiterType::Fence),
        '⇑' => (c, DelimiterType::Fence),
        '↓' => (c, DelimiterType::Fence),
        '⇓' => (c, DelimiterType::Fence),
        '↕' => (c, DelimiterType::Fence),
        '⇕' => (c, DelimiterType::Fence),
        '/' => (c, DelimiterType::Fence),
        _ => return None,
    })
}

/// Returns the matching delimiter for the given control sequence, if it exists.
pub fn control_sequence_delimiter_map(cs: &str) -> Option<(char, DelimiterType)> {
    Some(match cs {
        "lparen" => ('(', DelimiterType::Open),
        "rparen" => (')', DelimiterType::Close),
        "llparenthesis" => ('⦇', DelimiterType::Open),
        "rrparenthesis" => ('⦈', DelimiterType::Close),
        "lgroup" => ('⟮', DelimiterType::Open),
        "rgroup" => ('⟯', DelimiterType::Close),

        "lbrack" => ('[', DelimiterType::Open),
        "rbrack" => (']', DelimiterType::Close),
        "lBrack" => ('⟦', DelimiterType::Open),
        "rBrack" => ('⟧', DelimiterType::Close),

        "{" | "lbrace" => ('{', DelimiterType::Open),
        "}" | "rbrace" => ('}', DelimiterType::Close),
        "lBrace" => ('⦃', DelimiterType::Open),
        "rBrace" => ('⦄', DelimiterType::Close),

        "langle" => ('⟨', DelimiterType::Open),
        "rangle" => ('⟩', DelimiterType::Close),
        "lAngle" => ('⟪', DelimiterType::Open),
        "rAngle" => ('⟫', DelimiterType::Close),
        "llangle" => ('⦉', DelimiterType::Open),
        "rrangle" => ('⦊', DelimiterType::Close),

        "lfloor" => ('⌊', DelimiterType::Open),
        "rfloor" => ('⌋', DelimiterType::Close),
        "lceil" => ('⌈', DelimiterType::Open),
        "rceil" => ('⌉', DelimiterType::Close),
        "ulcorner" => ('┌', DelimiterType::Open),
        "urcorner" => ('┐', DelimiterType::Close),
        "llcorner" => ('└', DelimiterType::Open),
        "lrcorner" => ('┘', DelimiterType::Close),

        "lmoustache" => ('⎰', DelimiterType::Open),
        "rmoustache" => ('⎱', DelimiterType::Close),
        "backslash" => ('\\', DelimiterType::Fence),

        "vert" => ('|', DelimiterType::Fence),
        "lvert" => ('|', DelimiterType::Open),
        "rvert"  => ('|', DelimiterType::Close),
        "|" | "Vert"  => ('‖', DelimiterType::Fence),
        "lVert" => ('‖', DelimiterType::Open),
        "rVert" => ('‖', DelimiterType::Close),
        "uparrow" => ('↑', DelimiterType::Fence),
        "Uparrow" => ('⇑', DelimiterType::Fence),
        "downarrow" => ('↓', DelimiterType::Fence),
        "Downarrow" => ('⇓', DelimiterType::Fence),
        "updownarrow" => ('↕', DelimiterType::Fence),
        "Updownarrow" => ('⇕', DelimiterType::Fence),
        _ => return None,
    })
}

/// Returns the matching delimiter character for the given token, if it exists, along with whether
/// the delimiter is an opening (left) delimiter.
pub fn token_to_delim(token: Token) -> Option<(char, DelimiterType)> {
    match token {
        Token::ControlSequence(cs) => control_sequence_delimiter_map(cs),
        Token::Character(c) => char_delimiter_map(c.into()),
    }
}

/// Returns whether the given string is a valid primitive color.
///
/// Named colors come from the [MDN docs](https://developer.mozilla.org/en-US/docs/Web/CSS/named-color#value),
/// which is a list of about 150 official css color names.
pub fn is_primitive_color(color: &str) -> bool {
    color.starts_with('#')
        && color.len() == 7
        && color.as_bytes()[1..].iter().all(|&c| c.is_ascii_hexdigit())
        || matches!(
            color,
            "aliceblue"
                | "antiquewhite"
                | "aqua"
                | "aquamarine"
                | "azure"
                | "beige"
                | "bisque"
                | "black"
                | "blanchedalmond"
                | "blue"
                | "blueviolet"
                | "brown"
                | "burlywood"
                | "cadetblue"
                | "chartreuse"
                | "chocolate"
                | "coral"
                | "cornflowerblue"
                | "cornsilk"
                | "crimson"
                | "cyan"
                | "darkblue"
                | "darkcyan"
                | "darkgoldenrod"
                | "darkgray"
                | "darkgreen"
                | "darkgrey"
                | "darkkhaki"
                | "darkmagenta"
                | "darkolivegreen"
                | "darkorange"
                | "darkorchid"
                | "darkred"
                | "darksalmon"
                | "darkseagreen"
                | "darkslateblue"
                | "darkslategray"
                | "darkslategrey"
                | "darkturquoise"
                | "darkviolet"
                | "deeppink"
                | "deepskyblue"
                | "dimgray"
                | "dimgrey"
                | "dodgerblue"
                | "firebrick"
                | "floralwhite"
                | "forestgreen"
                | "fuchsia"
                | "gainsboro"
                | "ghostwhite"
                | "gold"
                | "goldenrod"
                | "gray"
                | "green"
                | "greenyellow"
                | "grey"
                | "honeydew"
                | "hotpink"
                | "indianred"
                | "indigo"
                | "ivory"
                | "khaki"
                | "lavender"
                | "lavenderblush"
                | "lawngreen"
                | "lemonchiffon"
                | "lightblue"
                | "lightcoral"
                | "lightcyan"
                | "lightgoldenrodyellow"
                | "lightgray"
                | "lightgreen"
                | "lightgrey"
                | "lightpink"
                | "lightsalmon"
                | "lightseagreen"
                | "lightskyblue"
                | "lightslategray"
                | "lightslategrey"
                | "lightsteelblue"
                | "lightyellow"
                | "lime"
                | "limegreen"
                | "linen"
                | "magenta"
                | "maroon"
                | "mediumaquamarine"
                | "mediumblue"
                | "mediumorchid"
                | "mediumpurple"
                | "mediumseagreen"
                | "mediumslateblue"
                | "mediumspringgreen"
                | "mediumturquoise"
                | "mediumvioletred"
                | "midnightblue"
                | "mintcream"
                | "mistyrose"
                | "moccasin"
                | "navajowhite"
                | "navy"
                | "oldlace"
                | "olive"
                | "olivedrab"
                | "orange"
                | "orangered"
                | "orchid"
                | "palegoldenrod"
                | "palegreen"
                | "paleturquoise"
                | "palevioletred"
                | "papayawhip"
                | "peachpuff"
                | "peru"
                | "pink"
                | "plum"
                | "powderblue"
                | "purple"
                | "rebeccapurple"
                | "red"
                | "rosybrown"
                | "royalblue"
                | "saddlebrown"
                | "salmon"
                | "sandybrown"
                | "seagreen"
                | "seashell"
                | "sienna"
                | "silver"
                | "skyblue"
                | "slateblue"
                | "slategray"
                | "slategrey"
                | "snow"
                | "springgreen"
                | "steelblue"
                | "tan"
                | "teal"
                | "thistle"
                | "tomato"
                | "transparent"
                | "turquoise"
                | "violet"
                | "wheat"
                | "white"
                | "whitesmoke"
                | "yellow"
                | "yellowgreen"
        )
}
