use rand::prelude::*;
use rand::seq::SliceRandom;

// Les opérations primitives que la stratégie peut enchaîner
#[derive(Clone, Debug)]
pub enum Op {
    Upper,
    Lower,
    Reverse,
    Double,
    Exclamation,
    Replace(char, char),
}

impl Op {
    fn all() -> Vec<Op> {
        vec![
            Op::Upper,
            Op::Lower,
            Op::Reverse,
            Op::Double,
            Op::Exclamation,
            Op::Replace('a', 'z'),
            Op::Replace('e', 'i'),
            Op::Replace('o', '0'),
        ]
    }
}
