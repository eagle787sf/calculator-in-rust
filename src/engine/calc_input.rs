// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// Ported from CalcInput.h and CalcInput.cpp

const MAX_DIGITS: usize = 32;
const MAX_EXP_DIGITS: usize = 4;

/// Represents a section of numeric input (base or exponent).
/// Ported from CalcNumSec in C++.
#[derive(Debug, Clone)]
struct NumSection {
    value: String,
    is_negative: bool,
}

impl NumSection {
    fn new() -> Self {
        Self {
            value: String::new(),
            is_negative: false,
        }
    }

    fn clear(&mut self) {
        self.value.clear();
        self.is_negative = false;
    }

    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
}

/// Handles digit-by-digit input for the calculator.
/// Ported from CalcInput in the C++ Microsoft Calculator.
#[derive(Debug, Clone)]
pub struct CalcInput {
    base: NumSection,
    exponent: NumSection,
    has_exponent: bool,
    has_decimal: bool,
    dec_pt_index: usize,
    dec_symbol: char,
}

impl CalcInput {
    pub fn new() -> Self {
        Self {
            base: NumSection::new(),
            exponent: NumSection::new(),
            has_exponent: false,
            has_decimal: false,
            dec_pt_index: 0,
            dec_symbol: '.',
        }
    }

    pub fn clear(&mut self) {
        self.base.clear();
        self.exponent.clear();
        self.has_exponent = false;
        self.has_decimal = false;
        self.dec_pt_index = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.base.is_empty() && !self.has_exponent && self.exponent.is_empty() && !self.has_decimal
    }

    /// Toggle the sign of the current input section.
    /// Ported from CalcInput::TryToggleSign.
    pub fn try_toggle_sign(&mut self) -> bool {
        if self.base.is_empty() {
            self.base.is_negative = false;
            self.exponent.is_negative = false;
        } else if self.has_exponent {
            self.exponent.is_negative = !self.exponent.is_negative;
        } else {
            self.base.is_negative = !self.base.is_negative;
        }
        true
    }

    /// Try to add a digit (0-9) to the input.
    /// Ported from CalcInput::TryAddDigit.
    pub fn try_add_digit(&mut self, value: u32, max_digits: usize) -> bool {
        let ch = if value < 10 {
            (b'0' + value as u8) as char
        } else {
            (b'A' + value as u8 - 10) as char
        };

        let (section, max_count) = if self.has_exponent {
            (&mut self.exponent, MAX_EXP_DIGITS)
        } else {
            let mut max = max_digits;
            // Don't count decimal point in digit limit
            if self.has_decimal {
                max += 1;
            }
            // Don't count leading zero
            if !self.base.is_empty() && self.base.value.starts_with('0') {
                max += 1;
            }
            (&mut self.base, max)
        };

        // Ignore leading zeros
        if section.is_empty() && value == 0 {
            return true;
        }

        if section.value.len() < max_count {
            section.value.push(ch);
            return true;
        }

        false
    }

    /// Try to add a decimal point.
    /// Ported from CalcInput::TryAddDecimalPt.
    pub fn try_add_decimal_pt(&mut self) -> bool {
        if self.has_decimal || self.has_exponent {
            return false;
        }

        if self.base.is_empty() {
            self.base.value.push('0');
        }

        self.dec_pt_index = self.base.value.len();
        self.base.value.push(self.dec_symbol);
        self.has_decimal = true;
        true
    }

    /// Try to begin exponent entry.
    /// Ported from CalcInput::TryBeginExponent.
    pub fn try_begin_exponent(&mut self) -> bool {
        self.try_add_decimal_pt();

        if self.has_exponent {
            return false;
        }

        self.has_exponent = true;
        true
    }

    /// Remove the last character from input.
    /// Ported from CalcInput::Backspace.
    pub fn backspace(&mut self) {
        if self.has_exponent {
            if !self.exponent.is_empty() {
                self.exponent.value.pop();
                if self.exponent.is_empty() {
                    self.exponent.clear();
                }
            } else {
                self.has_exponent = false;
            }
        } else {
            if !self.base.is_empty() {
                self.base.value.pop();
                if self.base.value == "0" {
                    self.base.value.pop();
                }
            }

            if self.base.value.len() <= self.dec_pt_index {
                self.has_decimal = false;
                self.dec_pt_index = 0;
            }

            if self.base.is_empty() {
                self.base.clear();
            }
        }
    }

    /// Convert the input to a string representation.
    /// Ported from CalcInput::ToString.
    pub fn to_string(&self) -> String {
        let mut result = String::new();

        if self.base.is_negative {
            result.push('-');
        }

        if self.base.is_empty() {
            result.push('0');
        } else {
            result.push_str(&self.base.value);
        }

        if self.has_exponent {
            if !self.has_decimal {
                result.push(self.dec_symbol);
            }
            result.push('e');
            result.push(if self.exponent.is_negative { '-' } else { '+' });

            if self.exponent.is_empty() {
                result.push('0');
            } else {
                result.push_str(&self.exponent.value);
            }
        }

        result
    }

    /// Convert the input string to an f64 value.
    /// Ported from CalcInput::ToRational, simplified for f64.
    pub fn to_f64(&self) -> f64 {
        let s = self.to_string();
        s.parse::<f64>().unwrap_or(0.0)
    }

    pub fn has_decimal_pt(&self) -> bool {
        self.has_decimal
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_input() {
        let mut input = CalcInput::new();
        assert!(input.is_empty());

        input.try_add_digit(1, MAX_DIGITS);
        input.try_add_digit(2, MAX_DIGITS);
        input.try_add_digit(3, MAX_DIGITS);
        assert_eq!(input.to_string(), "123");
        assert_eq!(input.to_f64(), 123.0);
    }

    #[test]
    fn test_decimal_input() {
        let mut input = CalcInput::new();
        input.try_add_digit(3, MAX_DIGITS);
        input.try_add_decimal_pt();
        input.try_add_digit(1, MAX_DIGITS);
        input.try_add_digit(4, MAX_DIGITS);
        assert_eq!(input.to_string(), "3.14");
    }

    #[test]
    fn test_sign_toggle() {
        let mut input = CalcInput::new();
        input.try_add_digit(5, MAX_DIGITS);
        input.try_toggle_sign();
        assert_eq!(input.to_string(), "-5");
        assert_eq!(input.to_f64(), -5.0);
    }

    #[test]
    fn test_backspace() {
        let mut input = CalcInput::new();
        input.try_add_digit(1, MAX_DIGITS);
        input.try_add_digit(2, MAX_DIGITS);
        input.try_add_digit(3, MAX_DIGITS);
        input.backspace();
        assert_eq!(input.to_string(), "12");
    }

    #[test]
    fn test_leading_zero() {
        let mut input = CalcInput::new();
        input.try_add_digit(0, MAX_DIGITS);
        assert!(input.is_empty()); // Leading zero ignored
        assert_eq!(input.to_string(), "0");
    }
}
