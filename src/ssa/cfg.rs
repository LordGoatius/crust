use super::*;

pub struct Cfg {
    nodes: HashSet<(Label, CodeBlock)>,
    edges: HashSet<(Label, Label)>,
    start: Label,
    stop: Label,
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn test_cfg() {
        // u32 name(struct ident arg1, u32 thing);
        // u32 name(struct ident arg1, u32 thing) {
        //     if ($arg1 $field .) {
        //         thing = 1;
        //     } else if ($arg1 $filrd ->) {
        //         usize um = (1 9 +);
        //     } else {
        //         thing += 7;
        //     }

        //     while (18 19 !=) {
        //         thing += 2;
        //     }

        //     for (;;) {
        //         {print}(17);
        //     }

        //     match $ident {
        //         15: { return 16; },
        //         _: {},
        //     }

        //     return 17;
        // }
    }
}
