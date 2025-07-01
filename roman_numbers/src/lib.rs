use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            _ => RomanDigit::M,
        }
    }
}

impl From<u32> for RomanNumber {
    fn from(mut value: u32) -> Self {
        let mut res: Vec<RomanDigit> = Vec::new();
        if value == 0 {
            res.push(0.into());
            return RomanNumber(res);
        }

        while value > 0 {
            let (minus, opt) = match value {
                1..4 => (1, None),
                4 => (1, Some(5)),
                5..9 => (5, None),
                9 => (1, Some(10)),
                10..40 => (10, None),
                40..50 => (10, Some(50)),
                50..90 => (50, None),
                90..100 => (10, Some(100)),
                100..400 => (100, None),
                400..500 => (100, Some(500)),
                500..900 => (500, None),
                900..1000 => (100, Some(1000)),
                _ => (1000, None),
            };

            res.push(minus.into());
            value -= minus;

            if let Some(num) = opt {
                res.push(num.into());
                value -= num - (minus * 2);
            }
        }

        RomanNumber(res)
    }
}