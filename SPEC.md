# Chordvery V0 Specification

A TUI chord discovery tool with MIDI input, binary tree progression suggestions, and a piano keyboard visualization.

---

## V0 Success Criteria

V0 is complete when ALL of the following work end-to-end:

1. **MIDI Input**: App connects to a MIDI device, receives note on/off events, displays pressed keys on piano
2. **Chord Detection**: Detects and names the chord being played (at minimum: major, minor, dim, aug, 7th variants)
3. **Binary Tree**: Shows 2 levels of suggested next chords based on the current chord
4. **Two Modes**: Discovery mode (history trail) and Jam mode (fading history) with Tab to switch
5. **All tests pass**: Unit tests for chord detection, progression logic, and UI rendering

---

## Tech Stack

```toml
[package]
name = "chordvery"
version = "0.1.0"
edition = "2021"
description = "TUI chord finder with MIDI input and progression suggestions"
license = "MIT"

[dependencies]
# Terminal UI (same as pianito)
ratatui = "0.29"
crossterm = "0.28"

# MIDI
midir = "0.10"

# CLI
clap = { version = "4", features = ["derive"] }

# Utilities
anyhow = "1.0"
thiserror = "2"

[dev-dependencies]
```

---

## File Structure

```
chordvery/
├── Cargo.toml
├── README.md                    # Basic usage instructions
├── src/
│   ├── main.rs                  # CLI entry, event loop
│   ├── lib.rs                   # Library exports
│   │
│   ├── midi/
│   │   ├── mod.rs
│   │   └── input.rs             # MIDI device connection, note events
│   │
│   ├── theory/
│   │   ├── mod.rs
│   │   ├── note.rs              # Note representation (MIDI number ↔ name)
│   │   ├── chord.rs             # Chord detection from notes
│   │   ├── quality.rs           # Chord quality (Major, Minor, Dim, etc.)
│   │   └── progression.rs       # Binary tree logic, next chord suggestions
│   │
│   └── ui/
│       ├── mod.rs
│       ├── app.rs               # App state machine
│       ├── theme.rs             # Colors, styles (from pianito)
│       └── components/
│           ├── mod.rs
│           ├── piano.rs         # Piano widget (adapted from pianito)
│           ├── tree.rs          # Binary tree visualization
│           └── history.rs       # Chord history with fade
```

---

## Module Specifications

### 1. `midi/input.rs` - MIDI Input Handler

**Struct**: `MidiInput`

```rust
pub struct MidiInput {
    connection: Option<MidiInputConnection<()>>,
    held_notes: Arc<Mutex<HashSet<u8>>>,  // MIDI note numbers currently held
}

impl MidiInput {
    pub fn new() -> Self;
    pub fn list_ports() -> Result<Vec<String>>;
    pub fn connect(port_index: usize) -> Result<Self>;
    pub fn connect_first() -> Result<Self>;  // Connect to first available
    pub fn held_notes(&self) -> HashSet<u8>;
    pub fn disconnect(&mut self);
}
```

**Behavior**:
- On note-on (velocity > 0): add to held_notes
- On note-off (or velocity = 0): remove from held_notes
- Thread-safe via Arc<Mutex>

**Tests**:
- [ ] `test_note_tracking` - mock MIDI events, verify held_notes state

---

### 2. `theory/note.rs` - Note Representation

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Note {
    pub midi: u8,           // MIDI note number (0-127)
}

impl Note {
    pub fn new(midi: u8) -> Self;
    pub fn name(&self) -> &'static str;           // "C", "C#", "D", etc.
    pub fn octave(&self) -> i8;                   // -1 to 9
    pub fn display(&self) -> String;             // "C4", "F#3", etc.
    pub fn pitch_class(&self) -> u8;             // 0-11 (C=0, C#=1, ...)
    pub fn from_name(name: &str) -> Option<Self>; // "C4" -> Note
}
```

**Tests**:
- [ ] `test_midi_to_name` - MIDI 60 → "C", 61 → "C#", etc.
- [ ] `test_octave` - MIDI 60 → octave 4, MIDI 21 → octave 0
- [ ] `test_from_name` - "C4" → MIDI 60

---

### 3. `theory/quality.rs` - Chord Qualities

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Quality {
    Major,          // R 3 5
    Minor,          // R b3 5
    Diminished,     // R b3 b5
    Augmented,      // R 3 #5
    Major7,         // R 3 5 7
    Minor7,         // R b3 5 b7
    Dominant7,      // R 3 5 b7
    Diminished7,    // R b3 b5 bb7
    HalfDim7,       // R b3 b5 b7
    MinorMajor7,    // R b3 5 7
    Augmented7,     // R 3 #5 b7
    Sus2,           // R 2 5
    Sus4,           // R 4 5
    Add9,           // R 3 5 9  (extended, optional for V0)
    Unknown,        // Unrecognized
}

impl Quality {
    pub fn symbol(&self) -> &'static str;  // "", "m", "dim", "+", "maj7", "m7", "7", etc.
    pub fn intervals(&self) -> &'static [u8];  // Semitones from root
}
```

**Tests**:
- [ ] `test_quality_intervals` - verify interval patterns
- [ ] `test_quality_symbol` - verify display symbols

---

### 4. `theory/chord.rs` - Chord Detection

```rust
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chord {
    pub root: Note,
    pub quality: Quality,
    pub bass: Option<Note>,  // For inversions (e.g., C/E)
}

impl Chord {
    pub fn detect(notes: &HashSet<u8>) -> Option<Self>;
    pub fn name(&self) -> String;  // "C", "Am", "G7", "Dm/F"
    pub fn roman_numeral(&self, key: Note) -> String;  // "I", "ii", "V7", etc.
}
```

**Detection Algorithm**:
1. If < 3 notes, return None
2. Normalize to pitch classes (0-11)
3. For each note as potential root:
   - Calculate intervals from root
   - Match against known quality patterns
4. Prefer root position, then common inversions
5. Return best match or Unknown

**Tests**:
- [ ] `test_detect_major` - {C, E, G} → C Major
- [ ] `test_detect_minor` - {A, C, E} → A Minor
- [ ] `test_detect_seventh` - {G, B, D, F} → G7
- [ ] `test_detect_inversion` - {E, G, C} → C/E
- [ ] `test_detect_empty` - {} → None
- [ ] `test_detect_two_notes` - {C, G} → None

---

### 5. `theory/progression.rs` - Binary Tree Suggestions

```rust
#[derive(Clone, Debug)]
pub struct ProgressionNode {
    pub chord: Chord,
    pub left: Option<Box<ProgressionNode>>,   // "Expected" next chord
    pub right: Option<Box<ProgressionNode>>,  // "Surprising" alternative
}

pub struct ProgressionTree {
    extended_mode: bool,  // Include 9ths, 11ths, 13ths
}

impl ProgressionTree {
    pub fn new() -> Self;
    pub fn set_extended(&mut self, extended: bool);
    pub fn suggest(&self, current: &Chord, key: Option<Note>) -> ProgressionNode;
}
```

**Suggestion Rules** (V0 - simple heuristics):

| Current | Left (Expected) | Right (Surprise) |
|---------|-----------------|------------------|
| I       | IV              | vi               |
| ii      | V               | IV               |
| iii     | vi              | IV               |
| IV      | V               | I                |
| V       | I               | vi               |
| vi      | ii              | IV               |
| vii°    | I               | iii              |

Additional rules:
- Circle of fifths: any chord → chord a 5th down
- Relative minor/major swaps
- Tritone substitution (extended mode only)

**Tests**:
- [ ] `test_suggest_from_I` - C major → (F, Am)
- [ ] `test_suggest_from_V` - G major → (C, Am)
- [ ] `test_two_levels` - verify tree has depth 2

---

### 6. `ui/components/piano.rs` - Piano Widget

Adapted from pianito.

**Changes from pianito**:
- Remove `deviations` (not tuning)
- Add `pressed: HashSet<u8>` for MIDI input
- Add `root_highlight: Option<u8>` to mark chord root
- Dynamic range based on pressed notes

```rust
pub struct Piano {
    start_midi: u8,
    num_keys: usize,
    pressed: HashSet<u8>,       // Currently pressed keys
    root: Option<u8>,           // Chord root (highlighted differently)
}

impl Piano {
    pub fn new(start_midi: u8, num_keys: usize) -> Self;
    pub fn dynamic(pressed: &HashSet<u8>) -> Self;  // Auto-range
    pub fn pressed(mut self, keys: HashSet<u8>) -> Self;
    pub fn root(mut self, midi: Option<u8>) -> Self;
}

impl Widget for Piano { ... }
```

**Tests** (from pianito, adapted):
- [ ] `test_render_octave_off`
- [ ] `test_render_octave_on`
- [ ] `test_dynamic_range` - expands based on notes

---

### 7. `ui/components/tree.rs` - Binary Tree Widget

```rust
pub struct ChordTree {
    root: Option<ProgressionNode>,
    depth: usize,  // How many levels to show (default 2)
}

impl ChordTree {
    pub fn new() -> Self;
    pub fn root(mut self, node: ProgressionNode) -> Self;
    pub fn depth(mut self, d: usize) -> Self;
}

impl Widget for ChordTree { ... }
```

**Rendering** (left-to-right tree):
```
         ┌─ Dm
    C ──┤
         └─ Am ──┬─ F
                 └─ Dm
```

Uses box-drawing chars: `─`, `┬`, `├`, `└`, `│`

**Tests**:
- [ ] `test_render_single_node`
- [ ] `test_render_two_levels`

---

### 8. `ui/components/history.rs` - Chord History

```rust
#[derive(Clone)]
pub struct ChordEntry {
    pub chord: Chord,
    pub age: u8,  // 0 = current, 1 = previous, etc.
}

pub struct ChordHistory {
    entries: Vec<ChordEntry>,
    max_entries: usize,
    fade: bool,  // Jam mode fading
}

impl ChordHistory {
    pub fn new(max: usize) -> Self;
    pub fn push(&mut self, chord: Chord);
    pub fn set_fade(&mut self, fade: bool);
    pub fn tick(&mut self);  // Age all entries
}

impl Widget for ChordHistory { ... }
```

**Rendering**:
- Discovery mode: `C → Am → F → G` (all same color)
- Jam mode: Recent chords bright, older chords dim (using ratatui styles)

**Tests**:
- [ ] `test_push_and_age`
- [ ] `test_max_entries`

---

### 9. `ui/app.rs` - App State Machine

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Discovery,
    Jam,
}

pub struct App {
    mode: Mode,
    midi: Option<MidiInput>,
    current_chord: Option<Chord>,
    history: ChordHistory,
    tree: ProgressionTree,
    should_quit: bool,
    extended_chords: bool,
}

impl App {
    pub fn new() -> Self;
    pub fn connect_midi(&mut self) -> Result<()>;
    pub fn toggle_mode(&mut self);
    pub fn toggle_extended(&mut self);
    pub fn tick(&mut self);  // Called each frame
    pub fn handle_key(&mut self, key: KeyCode);
    pub fn render(&self, frame: &mut Frame);
}
```

**Key Bindings**:
- `Tab` - Toggle mode (Discovery ↔ Jam)
- `e` - Toggle extended chords
- `?` - Help overlay
- `q` / `Esc` - Quit

**Tests**:
- [ ] `test_mode_toggle`
- [ ] `test_extended_toggle`

---

### 10. `main.rs` - Entry Point

```rust
#[derive(Parser)]
#[command(name = "chordvery")]
#[command(about = "TUI chord finder with MIDI input")]
struct Cli {
    /// MIDI port index (default: first available)
    #[arg(short, long)]
    port: Option<usize>,

    /// List available MIDI ports
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.list {
        // List ports and exit
    }

    // Init terminal, run app loop, restore terminal
}
```

---

## UI Layout

```
┌─ Chordvery ─────────────────────────────────────────────────────┐
│                                                                 │
│  ┌─ Suggestions ──────────────────┐  ┌─ History ─────────────┐  │
│  │         ┌─ Dm (ii)             │  │  C → Am → F → G       │  │
│  │    C ──┤                       │  │                       │  │
│  │         └─ Am (vi) ──┬─ F (IV) │  │                       │  │
│  │                      └─ Dm (ii)│  │                       │  │
│  └────────────────────────────────┘  └───────────────────────┘  │
│                                                                 │
│  ┌─ Piano ───────────────────────────────────────────────────┐  │
│  │ ║ █ █ ║ █ █ █ ║ █ █ ║ █ █ █ ║ █ █ ║ █ █ █ ║              │  │
│  │ ║▓█▓█▓║▓█▓█▓█▓║ █ █ ║ █ █ █ ║ █ █ ║ █ █ █ ║              │  │
│  │ ║▓║▓║▓║▓║▓║▓║▓║ ║ ║ ║ ║ ║ ║ ║ ║ ║ ║ ║ ║ ║ ║              │  │
│  │ ╚═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╩═╝              │  │
│  └───────────────────────────────────────────────────────────┘  │
│                                                                 │
│  [Tab] Mode: Discovery │ Playing: C major │ [e] Extended │ [?]  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Testing Requirements

### Unit Tests (required for V0)

| Module | Test | Description |
|--------|------|-------------|
| `theory::note` | `test_midi_to_name` | MIDI number to note name |
| `theory::note` | `test_octave` | MIDI to octave calculation |
| `theory::note` | `test_from_name` | Parse note string |
| `theory::chord` | `test_detect_major` | Major triad detection |
| `theory::chord` | `test_detect_minor` | Minor triad detection |
| `theory::chord` | `test_detect_seventh` | 7th chord detection |
| `theory::chord` | `test_detect_inversion` | Inverted chord detection |
| `theory::progression` | `test_suggest_from_I` | Suggestions from I chord |
| `theory::progression` | `test_two_levels` | Tree depth verification |
| `ui::piano` | `test_render_octave` | Piano rendering |
| `ui::piano` | `test_dynamic_range` | Auto-range calculation |
| `ui::tree` | `test_render_tree` | Tree widget rendering |
| `ui::history` | `test_push_and_age` | History aging |

### Manual Test (for PR)

1. Run `cargo run`
2. Connect MIDI keyboard
3. Play C major chord → see "C" detected, suggestions appear
4. Play Am → see chord change, history updates
5. Press Tab → mode switches, history fades (Jam) or stays (Discovery)
6. Press q → app exits cleanly

---

## Implementation Order

1. **Setup** - Create repo, Cargo.toml, file structure
2. **theory::note** - Note representation with tests
3. **theory::quality** - Chord qualities with tests
4. **theory::chord** - Chord detection with tests
5. **ui::theme** - Copy from pianito
6. **ui::components::piano** - Adapt from pianito with tests
7. **theory::progression** - Binary tree logic with tests
8. **ui::components::tree** - Tree widget with tests
9. **ui::components::history** - History widget with tests
10. **midi::input** - MIDI handling
11. **ui::app** - App state machine
12. **main.rs** - CLI and event loop
13. **Integration** - Wire everything together
14. **Polish** - Help overlay, error handling

---

## PR Readiness Checklist

- [ ] `cargo build --release` succeeds
- [ ] `cargo test` passes (all unit tests)
- [ ] `cargo clippy` has no warnings
- [ ] `cargo fmt --check` passes
- [ ] README.md has usage instructions
- [ ] Manual test passes (MIDI → detection → tree → modes)
- [ ] No TODO/FIXME comments left unaddressed

---

## Design Decisions

- **Piano range**: Dynamic auto-adjust based on notes played
- **Chord complexity**: Toggleable via `e` key (simple ↔ extended)
- **Progression styles**: All styles mixed (Pop, Jazz, Classical patterns)
