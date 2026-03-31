// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// Ported from scioper.cpp and scifunc.cpp

use super::command::*;
use super::display::*;

/// Perform a binary operation. Ported from CCalcEngine::DoOperation in scioper.cpp.
/// Parameters follow the C++ convention: operation on lhs and rhs.
/// Note: In the C++ code, for SUB, it computes rhs - result (i.e., lhs is the right operand).
/// The caller sets up lhs = current_val and rhs = last_val accordingly.
pub fn do_operation(operation: u32, lhs: f64, rhs: f64) -> Result<f64, u32> {
    let result = if lhs != 0.0 { lhs } else { 0.0 };

    match operation {
        IDC_AND => Ok((rhs as i64 & result as i64) as f64),
        IDC_OR => Ok((rhs as i64 | result as i64) as f64),
        IDC_XOR => Ok((rhs as i64 ^ result as i64) as f64),

        IDC_ADD => Ok(result + rhs),

        IDC_SUB => Ok(rhs - result),

        IDC_MUL => Ok(result * rhs),

        IDC_DIV => {
            if result == 0.0 {
                Err(CALC_E_DIVIDEBYZERO)
            } else {
                Ok(rhs / result)
            }
        }

        IDC_MOD => {
            if result == 0.0 {
                Err(CALC_E_DIVIDEBYZERO)
            } else {
                Ok(rhs % result)
            }
        }

        IDC_PWR => Ok(rhs.powf(result)),

        IDC_ROOT => {
            if result == 0.0 {
                Err(CALC_E_DIVIDEBYZERO)
            } else {
                Ok(rhs.powf(1.0 / result))
            }
        }

        IDC_LOGBASEY => {
            if result <= 0.0 || result == 1.0 || rhs <= 0.0 {
                Err(CALC_E_DOMAIN)
            } else {
                Ok(rhs.ln() / result.ln())
            }
        }

        IDC_LSHF => Ok(((rhs as i64) << (result as u32)) as f64),
        IDC_RSHF | IDC_RSHFL => Ok(((rhs as i64) >> (result as u32)) as f64),

        IDC_NAND => Ok(!((rhs as i64) & (result as i64)) as f64),
        IDC_NOR => Ok(!((rhs as i64) | (result as i64)) as f64),

        _ => Ok(lhs),
    }
}

/// Angle type enum matching the C++ AngleType.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AngleType {
    Degrees,
    Radians,
    Gradians,
}

/// Convert angle to radians based on angle type.
fn to_radians(val: f64, angle_type: AngleType) -> f64 {
    match angle_type {
        AngleType::Radians => val,
        AngleType::Degrees => val.to_radians(),
        AngleType::Gradians => val * std::f64::consts::PI / 200.0,
    }
}

/// Convert radians to the current angle type.
fn from_radians(val: f64, angle_type: AngleType) -> f64 {
    match angle_type {
        AngleType::Radians => val,
        AngleType::Degrees => val.to_degrees(),
        AngleType::Gradians => val * 200.0 / std::f64::consts::PI,
    }
}

/// Factorial function.
fn factorial(n: f64) -> Result<f64, u32> {
    if n < 0.0 || n != n.floor() {
        return Err(CALC_E_DOMAIN);
    }
    let n = n as u64;
    if n > 170 {
        return Err(CALC_E_OVERFLOW);
    }
    let mut result = 1.0_f64;
    for i in 2..=n {
        result *= i as f64;
    }
    Ok(result)
}

/// Perform a unary (scientific) function. Ported from CCalcEngine::SciCalcFunctions in scifunc.cpp.
pub fn sci_calc_functions(
    rat: f64,
    op: u32,
    is_inv: bool,
    angle_type: AngleType,
    last_val: f64,
    current_op: u32,
) -> Result<f64, u32> {
    match op {
        IDC_CHOP => {
            // Int/Frac toggle based on inverse
            Ok(if is_inv { rat.fract() } else { rat.trunc() })
        }

        IDC_COM => {
            // Complement: -(int(x) + 1) in decimal mode
            Ok(-(rat.trunc() + 1.0))
        }

        IDC_PERCENT => {
            // If operator is * or /, evaluate as "X [op] (Y%)"
            // Otherwise as "X [op] (X * Y%)"
            if current_op == IDC_MUL || current_op == IDC_DIV {
                Ok(rat / 100.0)
            } else {
                Ok(rat * (last_val / 100.0))
            }
        }

        IDC_SIN => {
            let r = to_radians(rat, angle_type);
            Ok(if is_inv {
                from_radians(r.asin(), angle_type)
            } else {
                r.sin()
            })
        }
        IDC_COS => {
            let r = to_radians(rat, angle_type);
            Ok(if is_inv {
                from_radians(r.acos(), angle_type)
            } else {
                r.cos()
            })
        }
        IDC_TAN => {
            let r = to_radians(rat, angle_type);
            Ok(if is_inv {
                from_radians(r.atan(), angle_type)
            } else {
                r.tan()
            })
        }

        IDC_SINH => Ok(if is_inv { rat.asinh() } else { rat.sinh() }),
        IDC_COSH => Ok(if is_inv { rat.acosh() } else { rat.cosh() }),
        IDC_TANH => Ok(if is_inv { rat.atanh() } else { rat.tanh() }),

        IDC_SEC => {
            let r = to_radians(rat, angle_type);
            Ok(if is_inv {
                from_radians((1.0 / rat).acos(), angle_type)
            } else {
                1.0 / r.cos()
            })
        }
        IDC_CSC => {
            let r = to_radians(rat, angle_type);
            Ok(if is_inv {
                from_radians((1.0 / rat).asin(), angle_type)
            } else {
                1.0 / r.sin()
            })
        }
        IDC_COT => {
            let r = to_radians(rat, angle_type);
            Ok(if is_inv {
                from_radians((1.0 / rat).atan(), angle_type)
            } else {
                1.0 / r.tan()
            })
        }

        IDC_SECH => Ok(if is_inv { (1.0 / rat).acosh() } else { 1.0 / rat.cosh() }),
        IDC_CSCH => Ok(if is_inv { (1.0 / rat).asinh() } else { 1.0 / rat.sinh() }),
        IDC_COTH => Ok(if is_inv { (1.0 / rat).atanh() } else { 1.0 / rat.tanh() }),

        IDC_REC => {
            if rat == 0.0 {
                Err(CALC_E_DIVIDEBYZERO)
            } else {
                Ok(1.0 / rat)
            }
        }

        IDC_SQR => Ok(rat * rat),

        IDC_SQRT => {
            if rat < 0.0 {
                Err(CALC_E_DOMAIN)
            } else {
                Ok(rat.sqrt())
            }
        }

        IDC_CUB => Ok(rat * rat * rat),
        IDC_CUBEROOT => Ok(rat.cbrt()),

        IDC_LOG => {
            if rat <= 0.0 {
                Err(CALC_E_DOMAIN)
            } else {
                Ok(rat.log10())
            }
        }

        IDC_POW10 => Ok(10.0_f64.powf(rat)),
        IDC_POW2 => Ok(2.0_f64.powf(rat)),

        IDC_LN => {
            if is_inv {
                Ok(rat.exp())
            } else if rat <= 0.0 {
                Err(CALC_E_DOMAIN)
            } else {
                Ok(rat.ln())
            }
        }

        IDC_FAC => factorial(rat),

        IDC_ABS => Ok(rat.abs()),
        IDC_FLOOR => Ok(if rat.fract() < 0.0 { (rat - 1.0).trunc() } else { rat.trunc() }),
        IDC_CEIL => Ok(if rat.fract() > 0.0 { (rat + 1.0).trunc() } else { rat.trunc() }),

        _ => Ok(rat),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        // do_operation convention: result = lhs + rhs for ADD
        assert_eq!(do_operation(IDC_ADD, 3.0, 5.0).unwrap(), 8.0);
    }

    #[test]
    fn test_subtraction() {
        // SUB: rhs - lhs (C++ convention: rhs - result)
        assert_eq!(do_operation(IDC_SUB, 3.0, 10.0).unwrap(), 7.0);
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(do_operation(IDC_MUL, 4.0, 5.0).unwrap(), 20.0);
    }

    #[test]
    fn test_division() {
        assert_eq!(do_operation(IDC_DIV, 4.0, 20.0).unwrap(), 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(do_operation(IDC_DIV, 0.0, 5.0).is_err());
    }

    #[test]
    fn test_power() {
        assert_eq!(do_operation(IDC_PWR, 3.0, 2.0).unwrap(), 8.0);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(
            sci_calc_functions(9.0, IDC_SQRT, false, AngleType::Degrees, 0.0, 0).unwrap(),
            3.0
        );
    }

    #[test]
    fn test_factorial() {
        assert_eq!(
            sci_calc_functions(5.0, IDC_FAC, false, AngleType::Degrees, 0.0, 0).unwrap(),
            120.0
        );
    }

    #[test]
    fn test_percent_mul() {
        // 200 * 50% = 200 * 0.5 = 100
        assert_eq!(
            sci_calc_functions(50.0, IDC_PERCENT, false, AngleType::Degrees, 200.0, IDC_MUL)
                .unwrap(),
            0.5
        );
    }

    #[test]
    fn test_percent_add() {
        // 200 + 10% => 200 + (200 * 10/100) = 200 + 20 = 220
        // The percent function returns 200 * 10/100 = 20
        assert_eq!(
            sci_calc_functions(10.0, IDC_PERCENT, false, AngleType::Degrees, 200.0, IDC_ADD)
                .unwrap(),
            20.0
        );
    }

    #[test]
    fn test_reciprocal() {
        assert_eq!(
            sci_calc_functions(4.0, IDC_REC, false, AngleType::Degrees, 0.0, 0).unwrap(),
            0.25
        );
    }

    #[test]
    fn test_square() {
        assert_eq!(
            sci_calc_functions(7.0, IDC_SQR, false, AngleType::Degrees, 0.0, 0).unwrap(),
            49.0
        );
    }

    #[test]
    fn test_negate_complement() {
        // COM: -(int(x) + 1)
        assert_eq!(
            sci_calc_functions(5.0, IDC_COM, false, AngleType::Degrees, 0.0, 0).unwrap(),
            -6.0
        );
    }
}
