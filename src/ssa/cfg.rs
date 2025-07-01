use super::*;

pub struct Cfg {
    nodes: HashSet<CodeBlock>,
    edges: HashSet<(Label, Label)>,
    start: Function,
    stop: Label,
}

#[cfg(test)]
pub mod tests {
    use crate::ssa::*;

    #[test]
    fn test_cfg() {
        let arg1 = Variable("Arg1".into());
        let thing = Variable("Arg1".into());
        let main = Function {
            name: "Name".to_owned(),
            label: Label(0),
            ret: Type::U32,
            args: vec![
                StaticValue(
                    arg1,
                    0,
                    Type::Struct(vec![
                        ("my_int".to_owned(), Type::I32),
                        ("my_fl".into(), Type::F64),
                    ]),
                ),
                StaticValue(thing, 0, Type::U32),
            ],
            blocks: HashSet::new(),
        };
        // struct ident {
        //     my_int i32;
        //     my_fl f64;
        // };
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
