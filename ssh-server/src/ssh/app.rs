use tui_app::App as TuiApp;
use ratatui::{backend::CrosstermBackend, Terminal, TerminalOptions, Viewport};
use ratatui::layout::Rect;
use crossterm::event::KeyEvent;
use super::terminal::TerminalHandle;
use tokio::sync::mpsc;

type SshTerminal = Terminal<CrosstermBackend<TerminalHandle>>;

#[derive(Debug)]
pub enum AppMode {
    Mock,
    Tui,
}

#[derive(Debug)]
pub struct App {
    pub content: String,
    pub tui_app: Option<TuiApp>,
    pub terminal: Option<SshTerminal>,
    pub input_tx: Option<mpsc::UnboundedSender<KeyEvent>>,
    pub mode: AppMode,
}

impl App {
    pub fn start() -> Self {
        Self {
            content: String::new(),
            tui_app: None,
            terminal: None,
            input_tx: None,
            mode: AppMode::Mock,
        }
    }

    pub fn start_tui(terminal_handle: TerminalHandle) -> Self {
        let backend = CrosstermBackend::new(terminal_handle);
        let options = TerminalOptions {
            viewport: Viewport::Fullscreen
        };

        let terminal = Terminal::with_options(backend, options).ok();

        Self {
            content: String::new(),
            tui_app: Some(TuiApp::new()),
            terminal,
            input_tx: None,
            mode: AppMode::Tui,
        }
    }

    pub fn serve(&mut self, route: Option<&str>) {
        match self.mode {
            AppMode::Mock => self.serve_mock(route),
            AppMode::Tui => self.serve_tui(),
        }
    }

    fn serve_mock(&mut self, route: Option<&str>) {
        self.content = match route {
            Some("hello") => "Shell started! Hello World!\r\n".to_string(),
            Some("discord") => "Here is the discord link: discord.gg/12345\r\n".to_string(),
            _ => "Here is the discord link: discord.gg/12345\r\n".to_string(),
        };
    }

    pub fn serve_tui(&mut self) {
        if let (Some(terminal), Some(tui_app)) = (self.terminal.take(), self.tui_app.take()) {
            let (tx, mut rx) = mpsc::unbounded_channel::<KeyEvent>();
            self.input_tx = Some(tx);

            tokio::spawn(async move {
                let mut terminal = terminal;
                let mut tui_app = tui_app;

                log::info!("TUI App started");
                tui_app.running = true;

                while tui_app.running {
                    // CRITICAL: Re-enable this
                    tui_app.poll_link_fetch();

                    if let Err(e) = terminal.draw(|f| tui_app.render(f)) {
                        log::error!("Draw failed: {}", e);
                        break;
                    }

                    // Handle input
                    while let Ok(ev) = rx.try_recv() {
                        tui_app.on_key_event(ev);
                    }

                    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                }

                log::info!("TUI App exited");
            });
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        if let Some(terminal) = &mut self.terminal {
            let rect = Rect { x: 0, y: 0, width, height };
            let _ = terminal.resize(rect);
        }
    }

    pub fn handle_input(&mut self, data: &[u8]) -> bool {
        match self.mode {
            AppMode::Mock => matches!(data, b"q" | b"\x03" | b"\x04"),
            AppMode::Tui => {
                if let Some(event) = ssh_data_to_key_event(data) {
                    if let Some(tx) = &self.input_tx {
                        let _ = tx.send(event);
                    }
                }
                false
            }
        }
    }
}

fn ssh_data_to_key_event(data: &[u8]) -> Option<crossterm::event::KeyEvent> {
    use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
    match data {
        b"q" => Some(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
        b"d" => Some(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE)),
        b"D" => Some(KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT)),
        b"200" => Some(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)),
        _ if data.len() == 1 && data[0].is_ascii() => {
            Some(KeyEvent::new(KeyCode::Char(data[0] as char), KeyModifiers::NONE))
        }
        _ => None,
    }
}
