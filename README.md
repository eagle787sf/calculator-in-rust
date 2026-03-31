# COSMIC Calculator

A calculator application built with **Rust** and the [libcosmic](https://github.com/pop-os/libcosmic) toolkit for the [COSMIC desktop environment](https://github.com/pop-os/cosmic-epoch).

Works natively on **Ubuntu Linux** and **Pop!_OS COSMIC**.

## Features

- Standard arithmetic operations: addition, subtraction, multiplication, division
- Parentheses for grouping expressions
- Power/exponent operator (`^`)
- Square root function (`√`)
- Percentage (modulo) operator (`%`)
- Sign negation (`+/-`)
- Full keyboard input support
- Built-in expression evaluator using the shunting-yard algorithm (no external dependencies like `qalc`)
- Native COSMIC desktop integration with automatic theming (light/dark)
- Internationalization (i18n) ready with Fluent

## Screenshot Layout

```
+---------------------------------+
|            Calculator           |
+---------------------------------+
|                              0  |
+---------------------------------+
|  (    )    √    x^n             |
|  C    %    ÷     ⌫             |
|  7    8    9     ×              |
|  4    5    6     −              |
|  1    2    3     +              |
|  ±    0    .     =              |
+---------------------------------+

<img width="385" height="516" alt="image" src="https://github.com/user-attachments/assets/028c1c00-47ac-4831-97df-34fe97a42b3a" />

```

---

## Installation Guide

### Ubuntu (22.04 / 24.04 / 24.10+)

#### Step 1: Install Rust

If you don't have Rust installed yet:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Verify with:

```bash
rustc --version   # should be 1.75 or higher
cargo --version
```

#### Step 2: Install system dependencies

```bash
sudo apt update
sudo apt install -y build-essential pkg-config cmake git \
    libwayland-dev libxkbcommon-dev libinput-dev \
    libfontconfig1-dev libfreetype6-dev libexpat1-dev
```

#### Step 3: Clone the repository

```bash
git clone https://github.com/eagle787sf/calculator-in-rust.git
cd calculator-in-rust
git checkout claude/cosmic-rust-calculator-mvpma
```

#### Step 4: Run the setup script

This clones the required `cosmic-text` dependency at the correct version:

```bash
chmod +x setup.sh
./setup.sh
```

#### Step 5: Build and run

```bash
# Build the release binary
cargo build --release

# Run the calculator
cargo run --release
```

#### Step 6: Install system-wide (optional)

Install `just` first (if not already installed):

```bash
cargo install just
```

Then install the calculator:

```bash
sudo just install
```

This installs:
- The binary to `/usr/bin/cosmic-calculator`
- The desktop entry to `/usr/share/applications/`
- The app icon to `/usr/share/icons/hicolor/scalable/apps/`

After installing, you can launch it from your application menu or by running:

```bash
cosmic-calculator
```

To uninstall:

```bash
sudo just uninstall
```

---

### Pop!_OS (COSMIC Edition)

Pop!_OS with the COSMIC desktop is the primary target for this calculator. The steps are the same as Ubuntu with one difference: COSMIC system dependencies are likely already installed.

#### Step 1: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

#### Step 2: Install build dependencies

```bash
sudo apt update
sudo apt install -y build-essential pkg-config cmake git \
    libwayland-dev libxkbcommon-dev libinput-dev \
    libfontconfig1-dev libfreetype6-dev libexpat1-dev
```

#### Step 3: Clone, setup, and build

```bash
git clone https://github.com/eagle787sf/calculator-in-rust.git
cd calculator-in-rust
git checkout claude/cosmic-rust-calculator-mvpma
./setup.sh
cargo build --release
```

#### Step 4: Install and launch

```bash
cargo install just
sudo just install
```

The calculator will now appear in the **COSMIC App Launcher** as "Calculator" with its own icon. It automatically matches your COSMIC desktop theme (light or dark).

You can also launch it from the terminal:

```bash
cosmic-calculator
```

---

## Quick Start (TL;DR)

```bash
# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh && source "$HOME/.cargo/env"

# Install system deps (Ubuntu / Pop!_OS)
sudo apt install -y build-essential pkg-config cmake git libwayland-dev \
    libxkbcommon-dev libinput-dev libfontconfig1-dev libfreetype6-dev libexpat1-dev

# Clone, build, and run
git clone https://github.com/eagle787sf/calculator-in-rust.git
cd calculator-in-rust
git checkout claude/cosmic-rust-calculator-mvpma
./setup.sh
cargo run --release
```

---

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| `0`-`9` | Number input |
| `+` `-` `*` `/` | Operators |
| `.` or `,` | Decimal point |
| `(` `)` | Parentheses |
| `^` | Power |
| `%` | Modulo |
| `Enter` | Evaluate expression |
| `Backspace` | Delete last character |
| `Escape` / `Delete` | Clear all |

## Architecture

| File | Description |
|------|-------------|
| `src/main.rs` | Entry point — initializes i18n and COSMIC app settings |
| `src/app.rs` | COSMIC `Application` trait implementation with UI grid layout and event handling |
| `src/calculator.rs` | Math expression evaluator using the shunting-yard algorithm (tokenizer → parser → RPN evaluator) |
| `src/i18n.rs` | Internationalization module using Fluent |
| `i18n/en/cosmic_calculator.ftl` | English translations |
| `justfile` | Build, install, and uninstall recipes |
| `res/*.desktop` | Desktop entry for COSMIC/GNOME app launchers |
| `res/icons/` | SVG application icon |
| `setup.sh` | Vendor setup script for cosmic-text dependency |

## Running Tests

The math expression evaluator includes unit tests:

```bash
cargo test
```

```
running 8 tests
test calculator::tests::test_basic_operations ... ok
test calculator::tests::test_decimal ... ok
test calculator::tests::test_division_by_zero ... ok
test calculator::tests::test_format_result ... ok
test calculator::tests::test_power ... ok
test calculator::tests::test_precedence ... ok
test calculator::tests::test_sqrt ... ok
test calculator::tests::test_unary_minus ... ok

test result: ok. 8 passed; 0 failed
```

## Inspired By

- [Microsoft Calculator](https://github.com/microsoft/calculator) — UI layout and feature set reference
- [COSMIC Epoch](https://github.com/pop-os/cosmic-epoch) — Desktop environment and toolkit
- [cosmic-utils/calculator](https://github.com/cosmic-utils/calculator) — Community COSMIC calculator

## License

GPL-3.0
