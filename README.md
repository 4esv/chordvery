# Chordvery

A TUI chord discovery tool with MIDI input, binary tree progression suggestions, and piano keyboard visualization.

## Features

Chordvery connects to any MIDI device to capture live chord playing, and it recognizes major, minor, diminished, and augmented chords, along with 7th variants, sus chords, and more. A binary tree visualization suggests chord progressions, showing both the expected paths and the surprising ones. The tool runs in two modes: Discovery mode keeps a persistent history trail of played chords, and Jam mode fades that history instead, for live improvisation. Additionally, the piano keyboard visualization updates as keys are pressed and highlights the root.

## Installation

```sh
cargo install --path .
```

Or build from source:

```sh
cargo build --release
./target/release/chordvery
```

## Usage

```sh
# Run with first available MIDI device
chordvery

# List available MIDI ports
chordvery --list

# Connect to a specific MIDI port
chordvery --port 1
```

## Keyboard Shortcuts

| Key     | Action                          |
|---------|---------------------------------|
| `Tab`   | Toggle Discovery/Jam mode       |
| `e`     | Toggle extended chords (7ths)   |
| `c`     | Clear chord history             |
| `?`     | Show help overlay               |
| `q`/Esc | Quit                            |

## Building

```sh
# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy lints
cargo clippy
```

## Requirements

- Rust 1.70+
- MIDI device (optional - app runs without one for testing)

## License

MIT
