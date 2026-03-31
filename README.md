# COSMIC Calculator

A calculator application built with Rust and the [libcosmic](https://github.com/pop-os/libcosmic) toolkit for the [COSMIC desktop environment](https://github.com/pop-os/cosmic-epoch).

Works on **Ubuntu Linux** and **Pop!_OS COSMIC**.

## Features

- Standard arithmetic operations: addition, subtraction, multiplication, division
- Parentheses for grouping expressions
- Power/exponent operator
- Square root function
- Percentage (modulo) operator
- Sign negation (+/-)
- Full keyboard input support
- Built-in expression evaluator (no external dependencies like `qalc`)
- Native COSMIC desktop integration with theming support
- Internationalization (i18n) ready

## Screenshot Layout

```
+---------------------------------+
|                Calculator       |
+---------------------------------+
|                              0  |
+---------------------------------+
|  (   )   sqrt   x^n            |
|  C   %    /     <-              |
|  7   8    9     x               |
|  4   5    6     -               |
|  1   2    3     +               |
| +/- 0    .      =              |
+---------------------------------+
```

## Building

### Prerequisites

- Rust toolchain (1.75+): https://rustup.rs/
- System dependencies for libcosmic/iced:

**Ubuntu / Pop!_OS:**
```bash
sudo apt install build-essential pkg-config libwayland-dev libxkbcommon-dev \
    libinput-dev libfontconfig1-dev libfreetype6-dev
```

### Setup

Run the setup script to vendor the required `cosmic-text` dependency:

```bash
./setup.sh
```

### Build & Run

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run
cargo run --release

# Using just (if installed)
just run
```

### Install

```bash
just install
```

This installs the binary, desktop entry, and icon to `/usr/local/`.

### Uninstall

```bash
just uninstall
```

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| 0-9 | Number input |
| + - * / | Operators |
| . or , | Decimal point |
| ( ) | Parentheses |
| ^ | Power |
| % | Modulo |
| Enter | Evaluate |
| Backspace | Delete last character |
| Escape / Delete | Clear all |

## Architecture

- **`src/main.rs`** - Entry point, initializes i18n and COSMIC app settings
- **`src/app.rs`** - COSMIC `Application` trait implementation with UI layout and event handling
- **`src/calculator.rs`** - Math expression evaluator using the shunting-yard algorithm (tokenizer, parser, RPN evaluator)
- **`src/i18n.rs`** - Internationalization module using Fluent

## License

GPL-3.0
