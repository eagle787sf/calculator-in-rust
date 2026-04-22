# COSMIC Calculator

A faithful **Rust port** of the [Microsoft Calculator](https://github.com/microsoft/calculator) engine, built with the [libcosmic](https://github.com/pop-os/libcosmic) toolkit for the [COSMIC desktop environment](https://github.com/pop-os/cosmic-epoch).

Works natively on **Ubuntu Linux** (22.04 / 24.04 / 24.10+) and **Pop!_OS COSMIC**.

## What Was Ported

The entire C++ CalcManager engine from Microsoft's open-source Windows Calculator was converted to idiomatic Rust:

| Original C++ File | Rust Port | What It Does |
|---|---|---|
| `Command.h` / `CCommand.h` | `engine/command.rs` | All 100+ command IDs (digits, operators, memory, modes) |
| `scicomm.cpp` ProcessCommandWorker | `engine/calc_engine.rs` | The complete calculator state machine with operator precedence, parenthesis stack, memory, and expression tracking |
| `scioper.cpp` DoOperation | `engine/operations.rs` | Binary operations: +, −, ×, ÷, mod, power, root, log base y |
| `scifunc.cpp` SciCalcFunctions | `engine/operations.rs` | Unary operations: sin/cos/tan, sinh/cosh/tanh, log/ln, √/x²/x³, factorial, reciprocal, percent, abs/floor/ceil |
| `CalcInput.cpp` | `engine/calc_input.rs` | Digit-by-digit input recording with decimal point, exponent, sign toggle, and backspace |
| `ICalcDisplay.h` | `engine/display.rs` | Display callback trait, error codes, number formatting |
| `CalculatorStandardOperators.xaml` | `app.rs` | UI button layout matching the original Windows Calculator |

## Features

- Standard calculator mode with full operator precedence (same as Windows Calculator)
- Parentheses with automatic closing on equals
- Memory operations: MC, MR, M+, M−, MS
- Percentage that works like Windows Calculator (context-aware: `200 + 10% = 220`)
- Scientific functions: 1/x, x², √x
- Full keyboard input support
- Implicit multiplication: `(3)(4) = 12` and `5(3) = 15`
- Native COSMIC desktop integration with automatic light/dark theming
- 18 unit tests covering the ported engine

## Button Layout

Matches the original `CalculatorStandardOperators.xaml` from Windows Calculator:

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

---

## Install on Ubuntu

Tested on Ubuntu 22.04 LTS, 24.04 LTS, and 24.10+.

### Easiest: Download the AppImage (no build required)

A pre-built AppImage is available in the [`dist/` folder](https://github.com/eagle787sf/calculator-in-rust/tree/claude/cosmic-rust-calculator-mvpma/dist). AppImages run on any modern Linux distribution with no installation.

```bash
# Download (7.6 MB)
wget https://github.com/eagle787sf/calculator-in-rust/raw/claude/cosmic-rust-calculator-mvpma/dist/CosmicCalculator-x86_64.AppImage

# Make it executable
chmod +x CosmicCalculator-x86_64.AppImage

# Run it
./CosmicCalculator-x86_64.AppImage
```

That's it — no Rust, no cargo, no build tools needed. The AppImage bundles everything except the standard Linux libraries (`libc`, `libm`, `libxkbcommon`) which are already on every Ubuntu/Pop!_OS system.

> **Optional:** To have the AppImage appear in your app launcher, you can install [AppImageLauncher](https://github.com/TheAssassin/AppImageLauncher) or manually move it to `~/.local/bin/` and create a `.desktop` file.

### Build from source

If you'd rather build it yourself:

### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Verify:

```bash
rustc --version   # needs 1.75+
cargo --version
```

### Step 2: Install system dependencies

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    cmake \
    git \
    just \
    libwayland-dev \
    libxkbcommon-dev \
    libinput-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libexpat1-dev
```

> **Note:** If `just` is not available via apt on your Ubuntu version, install it with `cargo install just`.

### Step 3: Clone and build

```bash
git clone https://github.com/eagle787sf/calculator-in-rust.git
cd calculator-in-rust
git checkout claude/cosmic-rust-calculator-mvpma
```

### Step 4: Run setup and build

```bash
chmod +x setup.sh
./setup.sh
cargo build --release
```

### Step 5: Run the calculator

```bash
cargo run --release
```

### Step 6: Install system-wide (optional)

```bash
sudo just install
```

This installs:

| What | Where |
|------|-------|
| Binary | `/usr/bin/cosmic-calculator` |
| Desktop entry | `/usr/share/applications/dev.eagle787sf.CosmicCalculator.desktop` |
| App icon | `/usr/share/icons/hicolor/scalable/apps/dev.eagle787sf.CosmicCalculator.svg` |

After installing, launch from your application menu or terminal:

```bash
cosmic-calculator
```

To uninstall:

```bash
sudo just uninstall
```

---

## Install on Pop!_OS COSMIC

Pop!_OS with the COSMIC desktop is the primary target. Most system libraries are already installed.

### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Step 2: Install build dependencies

```bash
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    cmake \
    git \
    just \
    libwayland-dev \
    libxkbcommon-dev \
    libinput-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libexpat1-dev
```

### Step 3: Clone, build, and install

```bash
git clone https://github.com/eagle787sf/calculator-in-rust.git
cd calculator-in-rust
git checkout claude/cosmic-rust-calculator-mvpma
./setup.sh
cargo build --release
sudo just install
```

The calculator appears in the **COSMIC App Launcher** as "Calculator" with its own icon. It automatically matches your COSMIC desktop theme.

Launch from terminal:

```bash
cosmic-calculator
```

---

## Quick Start (copy-paste)

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
├── app.rs                   COSMIC Application UI (ported from XAML layout)
├── i18n.rs                  Internationalization (Fluent)
└── engine/
    ├── mod.rs               Engine module exports
    ├── command.rs           Command enum (from Command.h / CCommand.h)
    ├── calc_engine.rs       State machine (from scicomm.cpp)
    ├── operations.rs        Binary + unary ops (from scioper.cpp / scifunc.cpp)
    ├── calc_input.rs        Digit input handling (from CalcInput.cpp)
    └── display.rs           Display trait + formatting (from ICalcDisplay.h)
```

## Running Tests

```bash
cargo test
```

```
running 18 tests
test engine::calc_input::tests::test_basic_input ... ok
test engine::calc_input::tests::test_decimal_input ... ok
test engine::calc_input::tests::test_sign_toggle ... ok
test engine::calc_input::tests::test_backspace ... ok
test engine::calc_input::tests::test_leading_zero ... ok
test engine::operations::tests::test_addition ... ok
test engine::operations::tests::test_subtraction ... ok
test engine::operations::tests::test_multiplication ... ok
test engine::operations::tests::test_division ... ok
test engine::operations::tests::test_division_by_zero ... ok
test engine::operations::tests::test_power ... ok
test engine::operations::tests::test_sqrt ... ok
test engine::operations::tests::test_factorial ... ok
test engine::operations::tests::test_percent_mul ... ok
test engine::operations::tests::test_percent_add ... ok
test engine::operations::tests::test_reciprocal ... ok
test engine::operations::tests::test_square ... ok
test engine::operations::tests::test_negate_complement ... ok

test result: ok. 18 passed; 0 failed
```

## Inspired By

- [Microsoft Calculator](https://github.com/microsoft/calculator) — Original C++ engine that was ported to Rust
- [COSMIC Epoch](https://github.com/pop-os/cosmic-epoch) — Desktop environment and libcosmic toolkit
- [cosmic-utils/calculator](https://github.com/cosmic-utils/calculator) — Community COSMIC calculator

## License

- Rust port and COSMIC UI: GPL-3.0
- Original Microsoft Calculator engine: MIT License
