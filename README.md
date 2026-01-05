# Chordvery

A TUI chord discovery tool with MIDI input, binary tree progression suggestions, and piano keyboard visualization.

## Features

- **MIDI Input**: Connect to any MIDI device to capture live chord playing
- **Chord Detection**: Recognizes major, minor, diminished, augmented, 7th variants, sus chords, and more
- **Progression Suggestions**: Binary tree visualization showing expected and surprising chord progressions
- **Two Modes**:
  - **Discovery Mode**: Persistent history trail of played chords
  - **Jam Mode**: Fading history for live improvisation
- **Piano Visualization**: Dynamic keyboard display with pressed keys and root highlighting

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
