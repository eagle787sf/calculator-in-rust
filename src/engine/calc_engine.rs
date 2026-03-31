// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// Ported from CalcEngine.h, scicomm.cpp, calc.cpp, scidisp.cpp

use super::calc_input::CalcInput;
use super::command::*;
use super::display::{self, CalcDisplay};
use super::operations::{self, AngleType};

/// Precedence of binary operators.
/// Ported from NPrecedenceOfOp in scicomm.cpp.
fn precedence_of_op(op: u32) -> u32 {
    match op {
        IDC_OR | IDC_XOR => 0,
        IDC_AND | IDC_NAND | IDC_NOR => 1,
        IDC_ADD | IDC_SUB => 2,
        IDC_LSHF | IDC_RSHF | IDC_RSHFL | IDC_MOD | IDC_DIV | IDC_MUL => 3,
        IDC_PWR | IDC_ROOT | IDC_LOGBASEY => 4,
        _ => 0,
    }
}

/// The core calculator engine, ported from CCalcEngine in the Microsoft Calculator.
/// Implements the complete state machine from scicomm.cpp ProcessCommandWorker.
pub struct CalcEngine {
    // Precedence mode
    use_precedence: bool,

    // Current state
    current_val: f64,
    last_val: f64,
    hold_val: f64,
    memory_val: f64,

    // Operator state
    op_code: u32,
    prev_op_code: u32,
    last_com: u32,
    temp_com: u32,

    // Flags
    change_op: bool,
    is_recording: bool,
    is_error: bool,
    is_inv: bool,
    no_prev_equ: bool,

    // Input
    input: CalcInput,
    number_string: String,
    precision: u32,
    angle_type: AngleType,

    // Parenthesis and precedence stacks (from CalcEngine.h)
    open_paren_count: usize,
    paren_vals: [f64; MAXPRECDEPTH],
    paren_ops: [u32; MAXPRECDEPTH],
    precedence_vals: [f64; MAXPRECDEPTH],
    precedence_ops: [u32; MAXPRECDEPTH],
    precedence_op_count: usize,

    // Expression tracking for display
    expression: String,
}

impl CalcEngine {
    /// Create a new CalcEngine. `use_precedence` enables operator precedence (standard mode).
    /// Ported from CCalcEngine constructor in calc.cpp.
    pub fn new(use_precedence: bool, precision: u32) -> Self {
        Self {
            use_precedence,
            current_val: 0.0,
            last_val: 0.0,
            hold_val: 0.0,
            memory_val: 0.0,
            op_code: 0,
            prev_op_code: 0,
            last_com: 0,
            temp_com: 0,
            change_op: false,
            is_recording: true,
            is_error: false,
            is_inv: false,
            no_prev_equ: true,
            input: CalcInput::new(),
            number_string: "0".to_string(),
            precision,
            angle_type: AngleType::Degrees,
            open_paren_count: 0,
            paren_vals: [0.0; MAXPRECDEPTH],
            paren_ops: [0; MAXPRECDEPTH],
            precedence_vals: [0.0; MAXPRECDEPTH],
            precedence_ops: [0; MAXPRECDEPTH],
            precedence_op_count: 0,
            expression: String::new(),
        }
    }

    /// Process a calculator command. Main entry point.
    /// Ported from CCalcEngine::ProcessCommand.
    pub fn process_command(&mut self, cmd: u32, display: &mut dyn CalcDisplay) {
        let cmd = if cmd == IDC_SET_RESULT {
            IDC_RECALL
        } else {
            cmd
        };

        self.process_command_worker(cmd, display);
    }

    /// Get the current display value as a formatted string.
    pub fn get_display(&self) -> String {
        if self.is_recording {
            let s = self.input.to_string();
            if s.is_empty() || s == "0" {
                "0".to_string()
            } else {
                s
            }
        } else {
            display::format_number(self.current_val, self.precision)
        }
    }

    /// Get the expression string for the secondary display.
    pub fn get_expression(&self) -> &str {
        &self.expression
    }

    /// Whether the engine is in error state.
    pub fn is_in_error(&self) -> bool {
        self.is_error
    }

    /// Get the current memory value.
    pub fn get_memory(&self) -> f64 {
        self.memory_val
    }

    /// Whether memory has a stored value.
    pub fn has_memory(&self) -> bool {
        self.memory_val != 0.0
    }

    /// Get open parenthesis count.
    pub fn get_open_paren_count(&self) -> usize {
        self.open_paren_count
    }

    // -----------------------------------------------------------------------
    // Private: the main state machine
    // Ported from CCalcEngine::ProcessCommandWorker in scicomm.cpp
    // -----------------------------------------------------------------------

    fn process_command_worker(&mut self, cmd: u32, display: &mut dyn CalcDisplay) {
        // Save last command (skip GUI settings)
        if !is_gui_setting_op_code(cmd) {
            self.last_com = self.temp_com;
            self.temp_com = cmd;
        }

        // Clear expression display after = was pressed
        if !self.no_prev_equ {
            self.expression.clear();
            display.set_expression_display(&[]);
        }

        // Error state handling
        if self.is_error {
            if cmd == IDC_CLEAR {
                // handle "C" normally
            } else if cmd == IDC_CENTR {
                // treat "CE" as "C"
                return self.process_command_worker(IDC_CLEAR, display);
            } else {
                self.handle_error_command(cmd);
                return;
            }
        }

        // Toggle Record/Display mode
        if self.is_recording {
            if is_bin_op_code(cmd)
                || is_unary_op_code(cmd)
                || is_op_in_range(cmd, IDC_FE, IDC_MMINUS)
                || is_op_in_range(cmd, IDC_OPENP, IDC_CLOSEP)
                || cmd == IDC_INV
                || cmd == IDC_SIGN
                || cmd == IDC_RAND
                || cmd == IDC_EULER
            {
                self.is_recording = false;
                self.current_val = self.input.to_f64();
                self.update_display(display);
            }
        } else if is_digit_op_code(cmd) || cmd == IDC_PNT {
            self.is_recording = true;
            self.input.clear();
            if self.last_com != IDC_CLOSEP {
                self.add_current_to_expression();
            }
        }

        // --- DIGIT KEYS ---
        if is_digit_op_code(cmd) {
            let value = cmd - IDC_0;

            if !self.input.try_add_digit(value, self.precision as usize) {
                self.handle_error_command(cmd);
                display.max_digits_reached();
                return;
            }

            // Implicit multiplication after close paren: e.g. (8)2 = 16
            if self.last_com == IDC_CLOSEP {
                self.op_code = IDC_MUL;
                self.last_val = self.current_val;
                self.hold_val = 0.0;
                self.no_prev_equ = true;
                self.expression.push_str(" \u{00D7} ");
                self.change_op = true;
                self.prev_op_code = 0;
                self.precedence_op_count = 0;
            }
            self.update_display(display);
            return;
        }

        // --- BINARY OPERATORS ---
        if is_bin_op_code(cmd) {
            // If last input was also a binary op, just change it
            if is_bin_op_code(self.last_com) {
                self.op_code = cmd;
                self.update_expression_last_op(cmd);
                display.binary_operator_received();
                return;
            }

            self.append_operand_to_expression();

            if self.change_op {
                // Precedence handling
                loop {
                    let nx = precedence_of_op(cmd);
                    let ni = precedence_of_op(self.op_code);

                    if nx > ni && self.use_precedence {
                        // Higher precedence: push current state
                        if self.precedence_op_count < MAXPRECDEPTH {
                            self.precedence_vals[self.precedence_op_count] = self.last_val;
                            self.precedence_ops[self.precedence_op_count] = self.op_code;
                        }
                        self.precedence_op_count += 1;
                        break;
                    } else {
                        // Execute pending operation
                        self.execute_operation(display);

                        if self.is_error {
                            return;
                        }

                        // Pop precedence stack if possible
                        if self.precedence_op_count > 0
                            && self.precedence_ops[self.precedence_op_count - 1] != 0
                        {
                            self.precedence_op_count -= 1;
                            self.op_code = self.precedence_ops[self.precedence_op_count];
                            self.last_val = self.precedence_vals[self.precedence_op_count];
                            continue;
                        }
                        break;
                    }
                }
            }

            display.binary_operator_received();
            self.last_val = self.current_val;
            self.op_code = cmd;
            self.expression.push_str(&format!(" {} ", op_to_string(cmd)));
            self.no_prev_equ = true;
            self.change_op = true;
            return;
        }

        // --- UNARY OPERATORS ---
        if is_unary_op_code(cmd) || cmd == IDC_DEGREES {
            if is_bin_op_code(self.last_com) {
                self.current_val = self.last_val;
            }

            self.append_operand_to_expression();

            // Add unary op to expression
            if cmd != IDC_PERCENT {
                let unary_str = unary_op_to_string(cmd, self.is_inv);
                self.wrap_expression_with_unary(&unary_str);
            }

            match operations::sci_calc_functions(
                self.current_val,
                cmd,
                self.is_inv,
                self.angle_type,
                self.last_val,
                self.op_code,
            ) {
                Ok(result) => {
                    self.current_val = result;
                }
                Err(err_code) => {
                    self.display_error(err_code, display);
                    return;
                }
            }

            self.update_display(display);

            if cmd == IDC_PERCENT {
                self.append_operand_to_expression();
            }

            // Reset inverse flag after use
            if self.is_inv {
                match cmd {
                    IDC_CHOP | IDC_SIN | IDC_COS | IDC_TAN | IDC_LN | IDC_DMS | IDC_DEGREES
                    | IDC_SINH | IDC_COSH | IDC_TANH | IDC_SEC | IDC_CSC | IDC_COT | IDC_SECH
                    | IDC_CSCH | IDC_COTH => {
                        self.is_inv = false;
                    }
                    _ => {}
                }
            }
            return;
        }

        // --- OTHER COMMANDS ---
        match cmd {
            IDC_CLEAR => {
                // Total clear - ported from scicomm.cpp IDC_CLEAR case
                self.last_val = 0.0;
                self.change_op = false;
                self.open_paren_count = 0;
                self.precedence_op_count = 0;
                self.temp_com = 0;
                self.last_com = 0;
                self.op_code = 0;
                self.prev_op_code = 0;
                self.no_prev_equ = true;

                self.is_inv = false;
                self.input.clear();
                self.is_recording = true;
                self.is_error = false;
                self.current_val = 0.0;
                self.number_string = "0".to_string();
                self.expression.clear();

                display.set_parenthesis_number(0);
                display.set_expression_display(&[]);
                self.update_display(display);
            }

            IDC_CENTR => {
                // Clear entry - ported from scicomm.cpp IDC_CENTR case
                self.is_inv = false;
                self.input.clear();
                self.is_recording = true;
                self.is_error = false;
                self.current_val = 0.0;
                self.update_display(display);
            }

            IDC_BACK => {
                if self.is_recording {
                    self.input.backspace();
                    self.update_display(display);
                } else {
                    self.handle_error_command(cmd);
                }
            }

            IDC_EQU => {
                // Equals - ported from scicomm.cpp IDC_EQU case
                // Auto-close all open parentheses
                while self.open_paren_count > 0 {
                    if self.is_error {
                        break;
                    }
                    self.temp_com = self.last_com;
                    self.process_command_worker(IDC_CLOSEP, display);
                    self.last_com = self.temp_com;
                    self.temp_com = cmd;
                }

                if !self.no_prev_equ {
                    self.last_val = self.current_val;
                }

                if is_bin_op_code(self.last_com) {
                    self.current_val = self.last_val;
                }

                self.append_operand_to_expression();

                // Resolve the precedence stack
                self.resolve_highest_precedence(display);
                while self.use_precedence && self.precedence_op_count > 0 {
                    self.precedence_op_count -= 1;
                    self.op_code = self.precedence_ops[self.precedence_op_count];
                    self.last_val = self.precedence_vals[self.precedence_op_count];
                    self.no_prev_equ = true;
                    self.resolve_highest_precedence(display);
                }

                if !self.is_error {
                    self.expression.push_str(" =");
                    self.last_val = self.current_val;
                    self.prev_op_code = 0;
                    self.precedence_op_count = 0;
                }

                self.change_op = false;
            }

            IDC_OPENP => {
                if self.open_paren_count >= MAXPRECDEPTH {
                    self.handle_error_command(cmd);
                    return;
                }

                // Implicit multiplication before (
                if is_digit_op_code(self.last_com)
                    || is_unary_op_code(self.last_com)
                    || self.last_com == IDC_PNT
                    || self.last_com == IDC_CLOSEP
                {
                    self.process_command_worker(IDC_MUL, display);
                }

                self.add_current_to_expression();
                self.expression.push_str("( ");

                self.paren_vals[self.open_paren_count] = self.last_val;
                self.paren_ops[self.open_paren_count] =
                    if self.change_op { self.op_code } else { 0 };
                self.open_paren_count += 1;

                if self.precedence_op_count < MAXPRECDEPTH {
                    self.precedence_ops[self.precedence_op_count] = 0;
                    self.precedence_op_count += 1;
                }

                self.last_val = 0.0;
                if is_bin_op_code(self.last_com) {
                    self.current_val = 0.0;
                }
                self.temp_com = 0;
                self.op_code = 0;
                self.change_op = false;

                display.set_parenthesis_number(self.open_paren_count as u32);
                self.update_display(display);
            }

            IDC_CLOSEP => {
                if self.open_paren_count == 0 {
                    display.on_no_right_paren_added();
                    self.handle_error_command(cmd);
                    return;
                }
                if self.precedence_op_count >= MAXPRECDEPTH
                    && self.precedence_ops[self.precedence_op_count - 1] != 0
                {
                    self.handle_error_command(cmd);
                    return;
                }

                if is_bin_op_code(self.last_com) {
                    self.current_val = self.last_val;
                }

                self.append_operand_to_expression();

                // Execute current operation
                self.execute_operation(display);

                // Pop precedence stack until we hit the open-paren marker (0)
                if self.precedence_op_count > 0 {
                    self.precedence_op_count -= 1;
                    while self.precedence_op_count > 0
                        && self.precedence_ops[self.precedence_op_count - 1] != 0
                    {
                        self.precedence_op_count -= 1;
                        let stacked_op = self.precedence_ops[self.precedence_op_count];
                        self.last_val = self.precedence_vals[self.precedence_op_count];
                        self.op_code = stacked_op;
                        self.execute_operation(display);
                    }
                    // Pop the 0 marker if present
                    if self.precedence_op_count > 0
                        && self.precedence_ops[self.precedence_op_count - 1] == 0
                    {
                        self.precedence_op_count -= 1;
                    }
                }

                self.expression.push_str(") ");

                // Restore state from before the open paren
                self.open_paren_count -= 1;
                self.last_val = self.paren_vals[self.open_paren_count];
                self.op_code = self.paren_ops[self.open_paren_count];
                self.change_op = self.op_code != 0;

                display.set_parenthesis_number(self.open_paren_count as u32);
                if !self.is_error {
                    self.update_display(display);
                }
            }

            IDC_SIGN => {
                if self.is_recording {
                    if self.input.try_toggle_sign() {
                        self.update_display(display);
                    } else {
                        self.handle_error_command(cmd);
                    }
                } else {
                    if is_bin_op_code(self.last_com) {
                        self.current_val = self.last_val;
                    }
                    self.current_val = -self.current_val;
                    self.update_display(display);
                }
            }

            IDC_RECALL => {
                self.current_val = self.memory_val;
                self.add_current_to_expression();
                self.update_display(display);
            }

            IDC_MPLUS => {
                self.memory_val += self.current_val;
            }
            IDC_MMINUS => {
                self.memory_val -= self.current_val;
            }
            IDC_STORE => {
                self.memory_val = self.current_val;
            }
            IDC_MCLEAR => {
                self.memory_val = 0.0;
            }

            IDC_PI => {
                self.add_current_to_expression();
                self.current_val = if self.is_inv {
                    2.0 * std::f64::consts::PI
                } else {
                    std::f64::consts::PI
                };
                self.update_display(display);
                self.is_inv = false;
            }

            IDC_RAND => {
                self.add_current_to_expression();
                self.current_val = rand_f64();
                self.update_display(display);
                self.is_inv = false;
            }

            IDC_EULER => {
                self.add_current_to_expression();
                self.current_val = std::f64::consts::E;
                self.update_display(display);
                self.is_inv = false;
            }

            IDC_FE => {
                // Toggle scientific notation (simplified)
                self.update_display(display);
            }

            IDC_EXP => {
                if self.is_recording && self.input.try_begin_exponent() {
                    self.update_display(display);
                } else {
                    self.handle_error_command(cmd);
                }
            }

            IDC_PNT => {
                // Implicit multiplication after close paren
                if self.last_com == IDC_CLOSEP {
                    self.op_code = IDC_MUL;
                    self.last_val = self.current_val;
                    self.hold_val = 0.0;
                    self.no_prev_equ = true;
                    self.expression.push_str(" \u{00D7} ");
                    self.change_op = true;
                    self.prev_op_code = 0;
                    self.precedence_op_count = 0;
                }

                if self.is_recording && self.input.try_add_decimal_pt() {
                    self.update_display(display);
                } else {
                    self.handle_error_command(cmd);
                }
            }

            IDC_INV => {
                self.is_inv = !self.is_inv;
            }

            _ => {}
        }
    }

    // -----------------------------------------------------------------------
    // Helper methods
    // -----------------------------------------------------------------------

    /// Execute the current pending operation (op_code on last_val and current_val).
    fn execute_operation(&mut self, display: &mut dyn CalcDisplay) {
        if self.op_code == 0 {
            return;
        }

        match operations::do_operation(self.op_code, self.current_val, self.last_val) {
            Ok(result) => {
                self.current_val = result;
                self.prev_op_code = self.op_code;
                if !self.is_error {
                    self.update_number_string();
                }
            }
            Err(err_code) => {
                self.display_error(err_code, display);
            }
        }
    }

    /// Resolve highest precedence operation (for equals).
    /// Ported from CCalcEngine::ResolveHighestPrecedenceOperation.
    fn resolve_highest_precedence(&mut self, display: &mut dyn CalcDisplay) {
        if self.op_code != 0 {
            if self.no_prev_equ {
                self.hold_val = self.current_val;
            } else {
                self.current_val = self.hold_val;
            }

            match operations::do_operation(self.op_code, self.current_val, self.last_val) {
                Ok(result) => {
                    self.current_val = result;
                    self.prev_op_code = self.op_code;
                    self.last_val = self.current_val;
                    if !self.is_error {
                        self.update_number_string();
                        self.update_display(display);
                    }
                }
                Err(err_code) => {
                    self.display_error(err_code, display);
                }
            }

            self.no_prev_equ = false;
        } else if !self.is_error {
            self.update_display(display);
        }
    }

    fn handle_error_command(&mut self, cmd: u32) {
        if !is_gui_setting_op_code(cmd) {
            self.temp_com = self.last_com;
        }
    }

    fn display_error(&mut self, err_code: u32, display: &mut dyn CalcDisplay) {
        let error_string = display::error_string(err_code).to_string();
        display.set_primary_display(&error_string, true);
        display.set_is_in_error(true);
        self.is_error = true;
        self.expression.clear();
    }

    fn update_display(&mut self, display: &mut dyn CalcDisplay) {
        let text = self.get_display();
        self.number_string = text.clone();
        display.set_primary_display(&text, false);
    }

    fn update_number_string(&mut self) {
        self.number_string = display::format_number(self.current_val, self.precision);
    }

    fn add_current_to_expression(&mut self) {
        // Used when transitioning from recording to display mode
    }

    fn append_operand_to_expression(&mut self) {
        // Add current value/input to expression if not already there
        if self.is_recording {
            let s = self.input.to_string();
            if !s.is_empty() && s != "0" {
                // Only append if expression doesn't already end with this number
                self.expression.push_str(&s);
            }
        } else if !self.number_string.is_empty() {
            // Append formatted number
        }
    }

    fn update_expression_last_op(&mut self, cmd: u32) {
        // Replace the last operator in the expression string
        let op_str = op_to_string(cmd);
        if let Some(last_space) = self.expression.rfind(' ') {
            if last_space > 0 {
                if let Some(second_last) = self.expression[..last_space].rfind(' ') {
                    self.expression.truncate(second_last);
                    self.expression.push_str(&format!(" {} ", op_str));
                }
            }
        }
    }

    fn wrap_expression_with_unary(&mut self, func_name: &str) {
        // Wrap the last operand in the expression with a unary function
        let val_str = display::format_number(self.current_val, self.precision);
        // Remove the last operand if it matches
        let trimmed = self.expression.trim_end();
        if trimmed.ends_with(&val_str) {
            let new_len = trimmed.len() - val_str.len();
            self.expression.truncate(new_len);
        }
        self.expression
            .push_str(&format!("{}({})", func_name, val_str));
    }
}

/// Simple pseudo-random number generator.
fn rand_f64() -> f64 {
    use std::time::SystemTime;
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (seed as f64 % 1000000.0) / 1000000.0
}

/// Convert a binary operator command to its display string.
/// Ported from OpCodeToBinaryString in scicomm.cpp.
fn op_to_string(cmd: u32) -> &'static str {
    match cmd {
        IDC_ADD => "+",
        IDC_SUB => "\u{2212}",
        IDC_MUL => "\u{00D7}",
        IDC_DIV => "\u{00F7}",
        IDC_MOD => "mod",
        IDC_PWR => "^",
        IDC_ROOT => "yroot",
        IDC_LOGBASEY => "logbase",
        IDC_AND => "AND",
        IDC_OR => "OR",
        IDC_XOR => "XOR",
        IDC_NAND => "NAND",
        IDC_NOR => "NOR",
        IDC_LSHF => "Lsh",
        IDC_RSHF => "Rsh",
        IDC_RSHFL => "Rsh",
        _ => "?",
    }
}

/// Convert a unary operator command to its display string.
/// Ported from OpCodeToUnaryString in scicomm.cpp.
fn unary_op_to_string(cmd: u32, is_inv: bool) -> String {
    match cmd {
        IDC_SQRT => "sqrt".to_string(),
        IDC_SQR => "sqr".to_string(),
        IDC_CUB => "cube".to_string(),
        IDC_CUBEROOT => "cuberoot".to_string(),
        IDC_FAC => "fact".to_string(),
        IDC_REC => "1/".to_string(),
        IDC_LOG => "log".to_string(),
        IDC_LN => {
            if is_inv {
                "powe".to_string()
            } else {
                "ln".to_string()
            }
        }
        IDC_POW10 => "10^".to_string(),
        IDC_POW2 => "2^".to_string(),
        IDC_ABS => "abs".to_string(),
        IDC_FLOOR => "floor".to_string(),
        IDC_CEIL => "ceil".to_string(),
        IDC_SIN => {
            if is_inv {
                "asin".to_string()
            } else {
                "sin".to_string()
            }
        }
        IDC_COS => {
            if is_inv {
                "acos".to_string()
            } else {
                "cos".to_string()
            }
        }
        IDC_TAN => {
            if is_inv {
                "atan".to_string()
            } else {
                "tan".to_string()
            }
        }
        IDC_SINH => {
            if is_inv {
                "asinh".to_string()
            } else {
                "sinh".to_string()
            }
        }
        IDC_COSH => {
            if is_inv {
                "acosh".to_string()
            } else {
                "cosh".to_string()
            }
        }
        IDC_TANH => {
            if is_inv {
                "atanh".to_string()
            } else {
                "tanh".to_string()
            }
        }
        IDC_CHOP => {
            if is_inv {
                "frac".to_string()
            } else {
                "int".to_string()
            }
        }
        IDC_COM => "not".to_string(),
        IDC_PERCENT => "%".to_string(),
        _ => "?".to_string(),
    }
}
