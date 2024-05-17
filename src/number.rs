use core::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Number {
    Integer(i64),
    Float(f64),
}
impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Float(fl) => write!(f, "{}", fl),
        }
    }
}
impl Number {
    pub fn as_u32(self) -> u32 {
        match self {
            Number::Integer(i) => i as u32,
            Number::Float(f) => f as u32,
        }
    }
    pub fn negate(self) -> Self {
        match self {
            Number::Integer(i) => Number::Integer(-i),
            Number::Float(f) => Number::Float(-f),
        }
    }
    pub fn checked_add(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => a.checked_add(b).map(Number::Integer),
            (Number::Float(a), Number::Float(b)) => Some(Number::Float(a + b)),
            (Number::Integer(a), Number::Float(b)) => Some(Number::Float(a as f64 + b)),
            (Number::Float(a), Number::Integer(b)) => Some(Number::Float(a + b as f64)),
        }
    }

    pub fn checked_sub(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => a.checked_sub(b).map(Number::Integer),
            (Number::Float(a), Number::Float(b)) => Some(Number::Float(a - b)),
            (Number::Integer(a), Number::Float(b)) => Some(Number::Float(a as f64 - b)),
            (Number::Float(a), Number::Integer(b)) => Some(Number::Float(a - b as f64)),
        }
    }

    pub fn checked_mul(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => a.checked_mul(b).map(Number::Integer),
            (Number::Float(a), Number::Float(b)) => Some(Number::Float(a * b)),
            (Number::Integer(a), Number::Float(b)) => Some(Number::Float(a as f64 * b)),
            (Number::Float(a), Number::Integer(b)) => Some(Number::Float(a * b as f64)),
        }
    }

    pub fn checked_div(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                if a == 0 || b == 0 {
                    None
                } else if a % b == 0 {
                    Some(Number::Integer(a / b))
                } else {
                    Some(Number::Float(a as f64 / b as f64))
                }
            }
            (Number::Float(a), Number::Float(b)) => Some(Number::Float(a / b)),
            (Number::Integer(a), Number::Float(b)) => Some(Number::Float(a as f64 / b)),
            (Number::Float(a), Number::Integer(b)) => Some(Number::Float(a / b as f64)),
        }
    }

    pub fn checked_pow(self, exp: u32) -> Option<Self> {
        match self {
            Number::Integer(a) => a.checked_pow(exp).map(Number::Integer),
            Number::Float(a) => Some(Number::Float(a.powi(exp as i32))),
        }
    }
}
