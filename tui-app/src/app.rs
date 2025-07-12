use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Layout, Constraint, Direction, Alignment}, 
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
    style::Style
};
use tokio::sync::mpsc;
use crate::brand;
use crate::components::welcome::*;
use crate::components::discord;

#[derive(Debug)]
enum LinkStatus {
    None,
    Fetching,
    Success(String),
    Error(String)
}

impl Default for LinkStatus {
    fn default() -> LinkStatus {
        LinkStatus::None
    }
}

#[derive(Debug)]
enum AppMessage {
    LinkFetched(String),
    LinkError(String),
}

#[derive(Debug, Default)]
pub struct App {
    pub input_buffer: String,
    running: bool,
    show_link: bool,
    link_status: LinkStatus,
    message_receiver: Option<mpsc::UnboundedReceiver<AppMessage>>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
            
            // Check for messages from async tasks
            if let Some(ref mut receiver) = self.message_receiver {
                while let Ok(message) = receiver.try_recv() {
                    match message {
                        AppMessage::LinkFetched(link) => {
                            self.link_status = LinkStatus::Success(link);
                        }
                        AppMessage::LinkError(error) => {
                            self.link_status = LinkStatus::Error(error);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let screen_area = frame.area();
        let background = Paragraph::new("")
            .style(Style::default().bg(brand::BrandColors::Dark.color()));
        frame.render_widget(background, screen_area);
        
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Logo height + spacing
                Constraint::Fill(1),     // Main block area
            ])
            .split(frame.area());
            
        let logo = Paragraph::new(brand::culturecode_logo_long())
            .alignment(Alignment::Center);
        frame.render_widget(logo, layout[0]);
        
        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(12),  // First paragraph block height
                Constraint::Length(25),  // Second paragraph block height
            ])
            .split(layout[1]);
            
        let block1 = Block::default();
        let inner1 = block1.inner(content_layout[0]);
        frame.render_widget(block1, content_layout[0]);
        
        let paragraph1 = self.get_welcome_paragraph();
        frame.render_widget(paragraph1, inner1);
        
        let block2 = Block::default();
        let inner2 = block2.inner(content_layout[1]);
        frame.render_widget(block2, content_layout[1]);
        
        let paragraph2 = welcome_paragraph_end();
        frame.render_widget(paragraph2, inner2);
    }

    fn get_welcome_paragraph(&self) -> Paragraph<'_> {
        match &self.link_status {
            LinkStatus::None => welcome_paragraph(self.show_link, None),
            LinkStatus::Fetching => welcome_paragraph(self.show_link, Some("Fetching link...")),
            LinkStatus::Success(link) => welcome_paragraph(self.show_link, Some(link)),
            LinkStatus::Error(error) => welcome_paragraph(self.show_link, Some(error)),
        }
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    pub fn on_key_event(&mut self, key: KeyEvent) {
        if let KeyCode::Char(c) = key.code {
            self.input_buffer.push(c);
            if self.input_buffer.len() > 3 {
                self.input_buffer.remove(0);
            }
            if self.input_buffer == "200" {
                self.running = false;
            }
            if c == 'd' || c == 'D' {
                if !self.show_link {
                    self.show_link = true;
                    self.start_link_fetch();
                }
            }
        }
    }

    pub fn process_async_messages(&mut self) {
        if let Some(ref mut receiver) = self.message_receiver {
            while let Ok(message) = receiver.try_recv() {
                match message {
                    AppMessage::LinkFetched(link) => {
                        self.link_status = LinkStatus::Success(link);
                    }
                    AppMessage::LinkError(error) => {
                        self.link_status = LinkStatus::Error(error);
                    }
                }
            }
        }
    }

    fn start_link_fetch(&mut self) {
        self.link_status = LinkStatus::Fetching;
        
        // channel for async
        let (sender, receiver) = mpsc::unbounded_channel();
        self.message_receiver = Some(receiver);
        
        // spawn async task
        tokio::spawn(async move {
            match discord::get_invite_link().await {
                Ok(link) => {
                    let _ = sender.send(AppMessage::LinkFetched(link));
                }
                Err(error) => {
                    let _ = sender.send(AppMessage::LinkError(format!("Error: {}", error)));
                }
            }
        });
    }
}
