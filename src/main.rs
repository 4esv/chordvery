use std::io;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use chordvery::midi::MidiInput;
use chordvery::ui::App;

#[derive(Parser)]
#[command(name = "chordvery")]
#[command(about = "TUI chord finder with MIDI input and progression suggestions")]
#[command(version)]
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
        list_ports()?;
        return Ok(());
    }

    let mut app = App::new();

    match cli.port {
        Some(port) => {
            if let Err(e) = app.connect_midi_port(port) {
                eprintln!("Warning: Could not connect to MIDI port {}: {}", port, e);
                eprintln!("Continuing without MIDI input...");
            }
        }
        None => {
            if let Err(e) = app.connect_midi() {
                eprintln!("Warning: Could not connect to MIDI: {}", e);
                eprintln!("Continuing without MIDI input...");
            }
        }
    }

    run_app(app)?;

    Ok(())
}

fn list_ports() -> Result<()> {
    let ports = MidiInput::list_ports()?;

    if ports.is_empty() {
        println!("No MIDI input ports available.");
    } else {
        println!("Available MIDI input ports:");
        for (i, name) in ports.iter().enumerate() {
            println!("  {}: {}", i, name);
        }
    }

    Ok(())
}

fn run_app(mut app: App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| app.render(f))?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if app.show_help {
                        app.show_help = false;
                    } else {
                        app.handle_key(key.code);
                    }
                }
            }
        }

        app.tick();

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
