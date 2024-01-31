use crate::Error;

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Condition {
    EQ = 0,
    NE = 1,
    CS = 2,
    CC = 3,
    MI = 4,
    PL = 5,
    VS = 6,
    VC = 7,
    HI = 8,
    LS = 9,
    GE = 10,
    LT = 11,
    GT = 12,
    LE = 13,
    None = 14,
}

impl TryFrom<u8> for Condition {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Condition::EQ),
            1 => Ok(Condition::NE),
            2 => Ok(Condition::CS),
            3 => Ok(Condition::CC),
            4 => Ok(Condition::MI),
            5 => Ok(Condition::PL),
            6 => Ok(Condition::VS),
            7 => Ok(Condition::VC),
            8 => Ok(Condition::HI),
            9 => Ok(Condition::LS),
            10 => Ok(Condition::GE),
            11 => Ok(Condition::LT),
            12 => Ok(Condition::GT),
            13 => Ok(Condition::LE),
            14 => Ok(Condition::None),
            _ => Err(Error::InvalidCondition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_u8_to_condition() {
        for n in 0..15 {
            let cond: Condition = n.try_into().unwrap();
            assert_eq!(cond as u8, n)
        }

        assert_eq!(15.try_into(), Err::<Condition, &'static str>("Invalid condition"))
    }
}
