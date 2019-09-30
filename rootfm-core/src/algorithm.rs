use crate::NUM_OPERATORS;

pub const OPERATOR_01: u8 = 1 << 00;
pub const OPERATOR_02: u8 = 1 << 01;
pub const OPERATOR_03: u8 = 1 << 02;
pub const OPERATOR_04: u8 = 1 << 03;
pub const OPERATOR_05: u8 = 1 << 04;
pub const OPERATOR_06: u8 = 1 << 05;
pub const NONE: u8 = 0;

pub const ALGORITHM_1: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_04,
        OPERATOR_05,
        OPERATOR_06,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_2: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03,
    modulators: [
        OPERATOR_02,
        OPERATOR_02,
        OPERATOR_04,
        OPERATOR_05,
        OPERATOR_06,
        NONE,
    ],
};

pub const ALGORITHM_3: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_04,
    modulators: [
        OPERATOR_02,
        OPERATOR_03,
        NONE,
        OPERATOR_04,
        OPERATOR_05,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_4: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_04,
    modulators: [
        OPERATOR_02,
        OPERATOR_03,
        NONE,
        OPERATOR_04,
        OPERATOR_05,
        OPERATOR_04,
    ],
};

pub const ALGORITHM_5: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03 | OPERATOR_05,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_04,
        NONE,
        OPERATOR_06,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_6: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03 | OPERATOR_05,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_04,
        NONE,
        OPERATOR_06,
        OPERATOR_05,
    ],
};

pub const ALGORITHM_7: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03 | OPERATOR_05,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_04 | OPERATOR_05,
        NONE,
        OPERATOR_06,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_8: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_04 | OPERATOR_05,
        OPERATOR_04,
        OPERATOR_06,
        NONE,
    ],
};

pub const ALGORITHM_9: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03,
    modulators: [
        OPERATOR_02,
        OPERATOR_02,
        OPERATOR_04 | OPERATOR_05,
        NONE,
        OPERATOR_06,
        NONE,
    ],
};

pub const ALGORITHM_10: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_04,
    modulators: [
        OPERATOR_02,
        OPERATOR_03,
        OPERATOR_03,
        OPERATOR_05 | OPERATOR_06,
        NONE,
        NONE,
    ],
};

pub const ALGORITHM_11: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_04,
    modulators: [
        OPERATOR_02,
        OPERATOR_03,
        NONE,
        OPERATOR_05 | OPERATOR_06,
        NONE,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_12: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03,
    modulators: [
        OPERATOR_02,
        OPERATOR_02,
        OPERATOR_04 | OPERATOR_05 | OPERATOR_06,
        NONE,
        NONE,
        NONE,
    ],
};

pub const ALGORITHM_13: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_04 | OPERATOR_05 | OPERATOR_06,
        NONE,
        NONE,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_14: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_04,
        OPERATOR_05 | OPERATOR_06,
        NONE,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_15: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03,
    modulators: [
        OPERATOR_02 | OPERATOR_06,
        OPERATOR_05,
        OPERATOR_04,
        OPERATOR_06,
        NONE,
        NONE,
    ],
};

pub const ALGORITHM_16: Algorithm = Algorithm {
    carriers: OPERATOR_01,
    modulators: [
        OPERATOR_02 | OPERATOR_03 | OPERATOR_05,
        NONE,
        OPERATOR_04,
        NONE,
        OPERATOR_06,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_17: Algorithm = Algorithm {
    carriers: OPERATOR_01,
    modulators: [
        OPERATOR_02 | OPERATOR_03 | OPERATOR_05,
        OPERATOR_02,
        OPERATOR_04,
        NONE,
        OPERATOR_06,
        NONE,
    ],
};

pub const ALGORITHM_18: Algorithm = Algorithm {
    carriers: OPERATOR_01,
    modulators: [
        OPERATOR_02 | OPERATOR_03 | OPERATOR_04,
        NONE,
        OPERATOR_03,
        OPERATOR_05,
        OPERATOR_06,
        NONE,
    ],
};

pub const ALGORITHM_19: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_04 | OPERATOR_05,
    modulators: [
        OPERATOR_02,
        OPERATOR_03,
        NONE,
        OPERATOR_06,
        OPERATOR_06,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_20: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_04,
    modulators: [
        OPERATOR_03,
        OPERATOR_03,
        OPERATOR_03,
        OPERATOR_05 | OPERATOR_06,
        NONE,
        NONE,
    ],
};

pub const ALGORITHM_21: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_04 | OPERATOR_05,
    modulators: [
        OPERATOR_03,
        OPERATOR_03,
        OPERATOR_03,
        OPERATOR_06,
        OPERATOR_06,
        NONE,
    ],
};

pub const ALGORITHM_22: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03 | OPERATOR_04 | OPERATOR_05,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_06,
        OPERATOR_06,
        OPERATOR_06,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_23: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_04 | OPERATOR_05,
    modulators: [
        NONE,
        OPERATOR_03,
        NONE,
        OPERATOR_06,
        OPERATOR_06,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_24: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_03 | OPERATOR_04 | OPERATOR_05,
    modulators: [
        NONE,
        NONE,
        OPERATOR_06,
        OPERATOR_06,
        OPERATOR_06,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_25: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_03 | OPERATOR_04 | OPERATOR_05,
    modulators: [NONE, NONE, NONE, OPERATOR_06, OPERATOR_06, OPERATOR_06],
};

pub const ALGORITHM_26: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_04,
    modulators: [
        NONE,
        OPERATOR_03,
        NONE,
        OPERATOR_05 | OPERATOR_06,
        NONE,
        OPERATOR_06,
    ],
};

pub const ALGORITHM_27: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_04,
    modulators: [
        NONE,
        OPERATOR_03,
        OPERATOR_03,
        OPERATOR_05 | OPERATOR_06,
        NONE,
        NONE,
    ],
};

pub const ALGORITHM_28: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_03 | OPERATOR_06,
    modulators: [
        OPERATOR_02,
        NONE,
        OPERATOR_04,
        OPERATOR_05,
        OPERATOR_05,
        NONE,
    ],
};

pub const ALGORITHM_29: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_03 | OPERATOR_05,
    modulators: [NONE, NONE, OPERATOR_04, NONE, OPERATOR_06, OPERATOR_06],
};

pub const ALGORITHM_30: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_03 | OPERATOR_05,
    modulators: [NONE, NONE, OPERATOR_04, OPERATOR_05, OPERATOR_05, NONE],
};

pub const ALGORITHM_31: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_03 | OPERATOR_04 | OPERATOR_05,
    modulators: [NONE, NONE, NONE, NONE, OPERATOR_06, OPERATOR_06],
};

pub const ALGORITHM_32: Algorithm = Algorithm {
    carriers: OPERATOR_01 | OPERATOR_02 | OPERATOR_03 | OPERATOR_04 | OPERATOR_05 | OPERATOR_06,
    modulators: [NONE, NONE, NONE, NONE, NONE, OPERATOR_06],
};

#[derive(Copy, Clone, Debug)]
pub struct Algorithm {
    carriers: u8,
    modulators: [u8; NUM_OPERATORS],
}

impl Algorithm {
    pub const fn is_carrier(&self, index: u32) -> bool {
        self.carriers & (1 << index) != 0
    }

    pub fn is_modulator(&self, index: u32) -> bool {
        let bit = 1 << index;
        self.modulators.iter().any(|x| x & bit != 0)
    }

    pub const fn get_modulators_for(&self, index: u32) -> u8 {
        self.modulators[index as usize]
    }
}
