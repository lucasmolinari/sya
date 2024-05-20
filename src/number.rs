use crate::errors::SyaError;
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
    pub fn as_u32(self) -> Result<u32, SyaError> {
        match self {
            Number::Integer(i) => i
                .try_into()
                .map_err(|_| SyaError::NumberOverflow(i.to_string())),
            Number::Float(f) => Err(SyaError::Custom(format!("Unsafe operation {} as u32", f))), // Floats can't be directly converted to u32 safely
        }
    }

    pub fn negate(self) -> Self {
        match self {
            Number::Integer(i) => Number::Integer(-i),
            Number::Float(f) => Number::Float(-f),
        }
    }

    pub fn checked_add(self, other: Self) -> Result<Self, SyaError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => Ok(Number::Integer(a + b)),
            (Number::Float(a), Number::Float(b)) => Ok(Number::Float(a + b)),
            (Number::Integer(a), Number::Float(b)) => Ok(Number::Float(a as f64 + b)),
            (Number::Float(a), Number::Integer(b)) => Ok(Number::Float(a + b as f64)),
        }
    }

    pub fn checked_sub(self, other: Self) -> Result<Self, SyaError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => Ok(Number::Integer(a - b)),
            (Number::Float(a), Number::Float(b)) => Ok(Number::Float(a - b)),
            (Number::Integer(a), Number::Float(b)) => Ok(Number::Float(a as f64 - b)),
            (Number::Float(a), Number::Integer(b)) => Ok(Number::Float(a - b as f64)),
        }
    }

    pub fn checked_mul(self, other: Self) -> Result<Self, SyaError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => Ok(Number::Integer(a * b)),
            (Number::Float(a), Number::Float(b)) => Ok(Number::Float(a * b)),
            (Number::Integer(a), Number::Float(b)) => Ok(Number::Float(a as f64 * b)),
            (Number::Float(a), Number::Integer(b)) => Ok(Number::Float(a * b as f64)),
        }
    }

    pub fn checked_div(self, other: Self) -> Result<Self, SyaError> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => {
                if b == 0 {
                    Err(SyaError::DivisionByZero)
                } else if a % b == 0 {
                    Ok(Number::Integer(a / b))
                } else {
                    Ok(Number::Float(a as f64 / b as f64))
                }
            }
            (Number::Float(a), Number::Float(b)) => {
                if b == 0.0 {
                    Err(SyaError::DivisionByZero)
                } else {
                    Ok(Number::Float(a / b))
                }
            }
            (Number::Integer(a), Number::Float(b)) => {
                if b == 0.0 {
                    Err(SyaError::DivisionByZero)
                } else {
                    Ok(Number::Float(a as f64 / b))
                }
            }
            (Number::Float(a), Number::Integer(b)) => {
                if b == 0 {
                    Err(SyaError::DivisionByZero)
                } else {
                    Ok(Number::Float(a / b as f64))
                }
            }
        }
    }

    pub fn checked_pow(self, exp: u32) -> Result<Self, SyaError> {
        match self {
            Number::Integer(a) => {
                let p = match a.checked_pow(exp) {
                    Some(p) => p,
                    None => return Err(SyaError::NumberOverflow(exp.to_string())),
                };
                Ok(Number::Integer(p))
            }
            Number::Float(a) => Ok(Number::Float(a.powi(exp as i32))),
        }
    }
}
