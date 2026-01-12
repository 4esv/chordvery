use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use midir::{MidiInput as MidirInput, MidiInputConnection};

pub struct MidiInput {
    _connection: Option<MidiInputConnection<()>>,
    held_notes: Arc<Mutex<HashSet<u8>>>,
}

impl MidiInput {
    pub fn new() -> Self {
        Self {
            _connection: None,
            held_notes: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub fn list_ports() -> Result<Vec<String>> {
        let midi_in = MidirInput::new("chordvery-list")?;
        let ports = midi_in.ports();

        let names: Vec<String> = ports
            .iter()
            .filter_map(|p| midi_in.port_name(p).ok())
            .collect();

        Ok(names)
    }

    pub fn connect(port_index: usize) -> Result<Self> {
        let midi_in = MidirInput::new("chordvery")?;
        let ports = midi_in.ports();

        if port_index >= ports.len() {
            return Err(anyhow!("Port index {} out of range", port_index));
        }

        let port = &ports[port_index];
        let port_name = midi_in.port_name(port)?;

        let held_notes = Arc::new(Mutex::new(HashSet::new()));
        let held_notes_clone = Arc::clone(&held_notes);

        let connection = midi_in.connect(
            port,
            "chordvery-input",
            move |_timestamp, message, _| {
                if message.len() >= 3 {
                    let status = message[0] & 0xF0;
                    let note = message[1];
                    let velocity = message[2];

                    let mut notes = held_notes_clone.lock().unwrap();

                    match status {
                        0x90 if velocity > 0 => {
                            notes.insert(note);
                        }
                        0x80 | 0x90 => {
                            notes.remove(&note);
                        }
                        _ => {}
                    }
                }
            },
            (),
        )?;

        eprintln!("Connected to MIDI port: {}", port_name);

        Ok(Self {
            _connection: Some(connection),
            held_notes,
        })
    }

    pub fn connect_first() -> Result<Self> {
        let ports = Self::list_ports()?;

        if ports.is_empty() {
            return Err(anyhow!("No MIDI ports available"));
        }

        Self::connect(0)
    }

    pub fn held_notes(&self) -> HashSet<u8> {
        self.held_notes.lock().unwrap().clone()
    }

    pub fn disconnect(&mut self) {
        self._connection = None;
    }
}

impl Default for MidiInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let midi = MidiInput::new();
        assert!(midi.held_notes().is_empty());
    }

    #[test]
    fn test_held_notes_empty() {
        let midi = MidiInput::new();
        let notes = midi.held_notes();
        assert!(notes.is_empty());
    }

    #[test]
    fn test_note_tracking_simulation() {
        let held_notes = Arc::new(Mutex::new(HashSet::new()));

        {
            let mut notes = held_notes.lock().unwrap();
            notes.insert(60);
            notes.insert(64);
            notes.insert(67);
        }

        {
            let notes = held_notes.lock().unwrap();
            assert!(notes.contains(&60));
            assert!(notes.contains(&64));
            assert!(notes.contains(&67));
            assert_eq!(notes.len(), 3);
        }

        {
            let mut notes = held_notes.lock().unwrap();
            notes.remove(&64);
        }

        {
            let notes = held_notes.lock().unwrap();
            assert!(notes.contains(&60));
            assert!(!notes.contains(&64));
            assert!(notes.contains(&67));
            assert_eq!(notes.len(), 2);
        }
    }
}
