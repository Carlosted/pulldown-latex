#[rustfmt::skip]
static OPERATOR_TABLE: &[(u16, u16)] = &[
    (33, 34), (37, 47), (58, 59), (63, 64), (91, 96), (123, 126), (168, 168), (172, 172), (175, 180), (183, 185), (215, 215), (247, 247), (710, 711), (713, 715), (717, 717), (728, 730), (732, 733), (759, 759), (770, 770), (785, 785), (800, 800), (802, 803), (805, 805), (807, 807), (814, 814), (817, 817), (8214, 8214), (8216, 8223), (8226, 8226), (8242, 8247), (8254, 8254), (8259, 8260), (8279, 8279), (8289, 8292), (8411, 8412), (8517, 8518), (8592, 8597), (8602, 8622), (8624, 8629), (8633, 8633), (8636, 8661), (8666, 8688), (8691, 8708), (8710, 8711), (8719, 8732), (8735, 8738), (8743, 8758), (8760, 8760), (8764, 8764), (8768, 8768), (8844, 8846), (8851, 8859), (8861, 8865), (8890, 8903), (8905, 8908), (8910, 8911), (8914, 8915), (8965, 8966), (8968, 8971), (8976, 8976), (8985, 8985), (8994, 8995), (9001, 9002), (9140, 9141), (9165, 9165), (9180, 9185), (10098, 10099), (10132, 10135), (10137, 10137), (10139, 10145), (10149, 10150), (10152, 10159), (10161, 10161), (10163, 10163), (10165, 10165), (10168, 10168), (10170, 10174), (10176, 10176), (10187, 10187), (10189, 10189), (10214, 10225), (10228, 10239), (10496, 10528), (10548, 10551), (10562, 10613), (10620, 10624), (10627, 10649), (10651, 10671), (10680, 10680), (10684, 10684), (10692, 10696), (10708, 10715), (10722, 10722), (10741, 10749), (10752, 10853), (10971, 10973), (10988, 10989), (10998, 10998), (11003, 11007), (11012, 11015), (11020, 11025), (11056, 11070), (11072, 11084), (11104, 11109), (11114, 11117), (11120, 11123), (11130, 11133), (11136, 11143), (11157, 11157), (11168, 11183), (11192, 11192)
];

fn is_operator(c: char) -> bool {
    let c = c as u16;

    let mut index = OPERATOR_TABLE.len() / 2;

    loop {
        let (start, end) = OPERATOR_TABLE[index];

        if c < start {
            if index == 0 {
                break;
            }

            index /= 2;
        } else if c > end {
            if index == OPERATOR_TABLE.len() - 1 {
                break;
            }

            index += (OPERATOR_TABLE.len() - index) / 2;
        } else {
            return true;
        }
    }

    false
}