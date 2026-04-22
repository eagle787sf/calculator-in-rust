# COSMIC Calculator — Microsoft Calculator Ported to Rust

A faithful **Rust port** of the [Microsoft Calculator](https://github.com/microsoft/calculator) C++ engine, built with the [libcosmic](https://github.com/pop-os/libcosmic) toolkit for the [COSMIC desktop environment](https://github.com/pop-os/cosmic-epoch).

Works natively on **Ubuntu Linux** (22.04 / 24.04 / 24.10+) and **Pop!_OS COSMIC**.

---

## Download AppImage (Easiest — No Build Required)

**[Download CosmicCalculator-x86_64.AppImage (7.6 MB)](https://github.com/eagle787sf/calculator-in-rust/raw/claude/cosmic-rust-calculator-mvpma/dist/CosmicCalculator-x86_64.AppImage)**

```bash
# Or from your terminal:
wget https://github.com/eagle787sf/calculator-in-rust/raw/claude/cosmic-rust-calculator-mvpma/dist/CosmicCalculator-x86_64.AppImage
chmod +x CosmicCalculator-x86_64.AppImage
./CosmicCalculator-x86_64.AppImage
```

No Rust, no cargo, no build tools needed. Works on any modern Linux distribution with a Wayland compositor.

---

## What Was Ported

The entire C++ CalcManager engine from Microsoft's open-source Windows Calculator was converted to idiomatic Rust:

| Original C++ File | Rust Port | What It Does |
|---|---|---|
| `Command.h` / `CCommand.h` | `engine/command.rs` | All 100+ command IDs (digits, operators, memory, modes) |
| `scicomm.cpp` ProcessCommandWorker | `engine/calc_engine.rs` | Complete calculator state machine with operator precedence, parenthesis stack, memory, and expression tracking |
| `scioper.cpp` DoOperation | `engine/operations.rs` | Binary operations: +, −, ×, ÷, mod, power, root, log base y |
| `scifunc.cpp` SciCalcFunctions | `engine/operations.rs` | Unary operations: sin/cos/tan, sinh/cosh/tanh, log/ln, √/x²/x³, factorial, reciprocal, percent, abs/floor/ceil |
| `CalcInput.cpp` | `engine/calc_input.rs` | Digit-by-digit input recording with decimal point, exponent, sign toggle, backspace |
| `ICalcDisplay.h` | `engine/display.rs` | Display callback trait, error codes, number formatting |
| `CalculatorStandardOperators.xaml` | `app.rs` | UI button layout matching the original Windows Calculator |

## Features

- **Standard mode** — full operator precedence, matching Windows Calculator
- **Scientific mode** — sin, cos, tan (+ inverses), log, ln, 10ˣ, x², √x, n!, |x|, π, e, x^y
- **Mode switcher** in the header bar (Standard ↔ Scientific)
- **Angle modes** — DEG / RAD / GRAD (cycle with button)
- **INV toggle** — for arcsin, arccos, arctan, eˣ (auto-resets after use)
- Parentheses with automatic closing on equals
- Memory operations: MC, MR, M+, M−, MS
- Context-aware percentage (`200 + 10% = 220`)
- Implicit multiplication: `(3)(4) = 12`
- Full keyboard input support
- Native COSMIC desktop integration with automatic light/dark theming
- 18 unit tests covering the ported engine

## Button Layout

### Standard Mode

```
+-------+-------+-------+-------+
|   %   |  CE   |   C   |   ⌫   |
+-------+-------+-------+-------+
|  1/x  |  x²   |  √x   |   ÷   |
+-------+-------+-------+-------+
|   7   |   8   |   9   |   ×   |
+-------+-------+-------+-------+
|   4   |   5   |   6   |   −   |
+-------+-------+-------+-------+
|   1   |   2   |   3   |   +   |
+-------+-------+-------+-------+
|   ±   |   0   |   .   |   =   |
+-------+-------+-------+-------+
```

<img width="1024" height="780" alt="Screenshot From 2026-04-21 21-19-20" src="https://github.com/user-attachments/assets/b0374f0f-42e6-4d56-8a6a-6b3435a45401" />


### Scientific Mode

```
+-------+-------+-------+-------+-------+
|  DEG  |  F-E  |   (   |   )   |   ⌫   |
+-------+-------+-------+-------+-------+
|  x²   |  x^y  |  sin  |  cos  |  tan  |
+-------+-------+-------+-------+-------+
|  √x   | 10^x  |  log  |  ln   |  INV  |
+-------+-------+-------+-------+-------+
|  n!   |   ±   |   π   |   e   |   ÷   |
+-------+-------+-------+-------+-------+
|   7   |   8   |   9   |   ×   |   %   |
+-------+-------+-------+-------+-------+
|   4   |   5   |   6   |   −   |  1/x  |
+-------+-------+-------+-------+-------+
|   1   |   2   |   3   |   +   |  |x|  |
+-------+-------+-------+-------+-------+
|  CE   |   0   |   .   |   =   |   C   |
+-------+-------+-------+-------+-------+
```
<img width="1024" height="780" alt="image" src="https://github.com/user-attachments/assets/d66d3b3f-713d-4de0-9e71-6d9b90b0531c" />

---

## Build From Source (Ubuntu / Pop!_OS)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && source "$HOME/.cargo/env"

# Install system deps
sudo apt update && sudo apt install -y build-essential pkg-config cmake git \
    libwayland-dev libxkbcommon-dev libinput-dev libfontconfig1-dev \
    libfreetype6-dev libexpat1-dev

# Clone, build, run
git clone https://github.com/eagle787sf/calculator-in-rust.git
cd calculator-in-rust
git checkout claude/cosmic-rust-calculator-mvpma
chmod +x setup.sh && ./setup.sh
cargo run --release
```

### System-wide install

```bash
cargo install just   # if not already installed
sudo just install
cosmic-calculator     # launch from terminal or app menu
```

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `0`–`9` | Number input |
| `+` `-` `*` `/` | Arithmetic operators |
| `.` or `,` | Decimal point |
| `(` `)` | Parentheses |
| `^` | Power |
| `%` | Percent |
| `!` | Factorial |
| `r` / `R` | Reciprocal (1/x) |
| `q` / `Q` | Square root |
| `Enter` | Equals |
| `Backspace` | Delete last digit |
| `Escape` | Clear all (C) |
| `Delete` | Clear entry (CE) |

## Project Structure

```
src/
├── main.rs                  Entry point, COSMIC app settings
├── app.rs                   COSMIC Application UI (Standard + Scientific modes)
├── i18n.rs                  Internationalization (Fluent)
└── engine/
    ├── mod.rs               Engine module exports
    ├── command.rs           Command enum (from Command.h / CCommand.h)
    ├── calc_engine.rs       State machine (from scicomm.cpp)
    ├── operations.rs        Binary + unary ops (from scioper.cpp / scifunc.cpp)
    ├── calc_input.rs        Digit input handling (from CalcInput.cpp)
    └── display.rs           Display trait + formatting (from ICalcDisplay.h)
```

## Tests

```bash
cargo test    # 18 tests, all passing
```

## Inspired By

- [Microsoft Calculator](https://github.com/microsoft/calculator) — Original C++ engine ported to Rust
- [COSMIC Epoch](https://github.com/pop-os/cosmic-epoch) — Desktop environment and libcosmic toolkit
- [cosmic-utils/calculator](https://github.com/cosmic-utils/calculator) — Community COSMIC calculator

## License

- Rust port and COSMIC UI: GPL-3.0
- Original Microsoft Calculator engine: [MIT License](./LICENSE)
