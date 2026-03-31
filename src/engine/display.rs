// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// Ported from ICalcDisplay.h

/// Callback interface for calculator display updates.
/// Ported from the C++ ICalcDisplay pure virtual interface.
pub trait CalcDisplay {
    fn set_primary_display(&mut self, text: &str, is_error: bool);
    fn set_is_in_error(&mut self, is_error: bool);
    fn set_expression_display(&mut self, tokens: &[(String, i32)]);
    fn set_parenthesis_number(&mut self, count: u32);
    fn max_digits_reached(&mut self);
    fn binary_operator_received(&mut self);
    fn on_no_right_paren_added(&mut self);
    fn on_history_item_added(&mut self, _add_to_history: u32) {}
    fn input_changed(&mut self) {}
}

/// Error string IDs matching EngineStrings.h
pub const IDS_DIVBYZERO: u32 = 0;
pub const IDS_DOMAIN: u32 = 1;
pub const IDS_UNDEFINED: u32 = 2;
pub const IDS_POS_INFINITY: u32 = 3;
pub const IDS_NEG_INFINITY: u32 = 4;
pub const IDS_NOMEM: u32 = 6;
pub const IDS_TOOMANY: u32 = 7;
pub const IDS_OVERFLOW: u32 = 8;
pub const IDS_NORESULT: u32 = 9;
pub const IDS_INSUFFICIENT_DATA: u32 = 10;

// CALC_E_ error codes matching winerror_cross_platform.h
pub const CALC_E_DIVIDEBYZERO: u32 = IDS_DIVBYZERO;
pub const CALC_E_DOMAIN: u32 = IDS_DOMAIN;
pub const CALC_E_OVERFLOW: u32 = IDS_OVERFLOW;
pub const CALC_E_NORESULT: u32 = IDS_NORESULT;
pub const CALC_E_UNDEFINED: u32 = IDS_UNDEFINED;

pub fn error_string(code: u32) -> &'static str {
    match code {
        IDS_DIVBYZERO => "Cannot divide by zero",
        IDS_DOMAIN => "Invalid input",
        IDS_UNDEFINED => "Result is undefined",
        IDS_POS_INFINITY => "Positive infinity",
        IDS_NEG_INFINITY => "Negative infinity",
        IDS_OVERFLOW => "Overflow",
        IDS_NORESULT => "Invalid input",
        IDS_INSUFFICIENT_DATA => "Insufficient data",
        _ => "Error",
    }
}

/// Format a f64 value for display, removing trailing zeros.
/// Ported from scidisp.cpp DisplayNum logic.
pub fn format_number(value: f64, precision: u32) -> String {
    if value.is_nan() {
        return "Invalid input".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 {
            "Positive infinity".to_string()
        } else {
            "Negative infinity".to_string()
        };
    }

    // If it's an integer value and fits in reasonable range, show without decimal
    if value == value.floor() && value.abs() < 1e15 {
        if value == 0.0 {
            return "0".to_string();
        }
        return format!("{}", value as i64);
    }

    // Show with precision, trim trailing zeros
    let s = format!("{:.prec$}", value, prec = precision as usize);
    let s = s.trim_end_matches('0');
    let s = s.trim_end_matches('.');
    s.to_string()
}

/// Separate digit grouping with commas for readability.
pub fn group_digits(s: &str) -> String {
    // Find the integer part (before decimal point)
    let (sign, rest) = if s.starts_with('-') {
        ("-", &s[1..])
    } else {
        ("", s)
    };

    let (integer_part, decimal_part) = if let Some(dot_pos) = rest.find('.') {
        (&rest[..dot_pos], Some(&rest[dot_pos..]))
    } else {
        (rest, None)
    };

    // Group integer digits by 3s from right
    let mut grouped = String::new();
    let chars: Vec<char> = integer_part.chars().collect();
    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(*ch);
    }

    let mut result = format!("{}{}", sign, grouped);
    if let Some(dec) = decimal_part {
        result.push_str(dec);
    }
    result
}
