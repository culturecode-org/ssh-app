use tui_app::App as TuiApp;
use ratatui::{backend::CrosstermBackend, Terminal, TerminalOptions, Viewport};
use ratatui::layout::Rect;
use super::terminal::TerminalHandle;
use std::sync::Arc;
use tokio::sync::Mutex;

type SshTerminal = Terminal<CrosstermBackend<TerminalHandle>>;

pub type SharedApp = Arc<Mutex<App>>;

#[derive(Debug)]
pub struct App {
    pub content: String,
    tui_app: Option<TuiApp>,
    terminal: Option<SshTerminal>,
    mode: AppMode,
}

#[derive(Debug)]
enum AppMode {
    Mock,
    Tui,
}

impl App {
    pub fn start() -> Self {
        Self { 
            content: String::new(),
            tui_app: None,
            terminal: None,
            mode: AppMode::Mock,
        }
    }
    
    pub fn start_tui(terminal_handle: TerminalHandle) -> Self {
        let backend = CrosstermBackend::new(terminal_handle);
        let options = TerminalOptions {
            viewport: Viewport::Fixed(Rect::default()),
        };
        let terminal = Terminal::with_options(backend, options).ok();
        
        Self {
            content: String::new(),
            tui_app: Some(TuiApp::new()),
            terminal,
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
        match route {
            Some("hello") => self.content = "Shell started! Hello World!\r\n".to_string(),
            Some("discord") => self.content = "Here is the discord link: discord.gg/12345\r\n".to_string(),
            _ => self.content = "Here is the discord link: discord.gg/12345\r\n".to_string(),
        }
    }
    
    fn serve_tui(&mut self) {
        if let (Some(terminal), Some(tui_app)) = (&mut self.terminal, &mut self.tui_app) {
            let _ = terminal.draw(|frame| tui_app.render(frame));
        }
    }
    
    pub fn handle_input(&mut self, data: &[u8]) -> bool {
        match self.mode {
            AppMode::Mock => matches!(data, b"q" | b"\x03" | b"\x04"),
            AppMode::Tui => {
                if let Some(tui_app) = &mut self.tui_app {
                    if let Some(key_event) = ssh_data_to_key_event(data) {
                        tui_app.on_key_event(key_event);
                        return tui_app.input_buffer == "200";
                    }
                }
                false
            }
        }
    }
    
    pub fn resize(&mut self, width: u16, height: u16) {
        if let Some(terminal) = &mut self.terminal {
            let rect = Rect { x: 0, y: 0, width, height };
            let _ = terminal.resize(rect);
        }
    }
}

fn ssh_data_to_key_event(data: &[u8]) -> Option<crossterm::event::KeyEvent> {
    use crossterm::event::{KeyEvent, KeyCode, KeyModifiers};
    match data {
        b"q" => Some(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE)),
        b"d" => Some(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE)),
        b"D" => Some(KeyEvent::new(KeyCode::Char('D'), KeyModifiers::SHIFT)),
        b"200" => Some(KeyEvent::new(KeyCode::Char('0'), KeyModifiers::NONE)), // Handle sequence
        _ if data.len() == 1 && data[0].is_ascii() => {
            Some(KeyEvent::new(KeyCode::Char(data[0] as char), KeyModifiers::NONE))
        }
        _ => None,
    }
}
