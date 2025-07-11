use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Layout, Constraint, Direction, Alignment}, 
    widgets::{Block, Paragraph},
    backend::Backend,
    Terminal, Frame,
    style::Style
};
use crate::brand;
use crate::components::welcome::*;
use crate::components::discord;

#[derive(Debug)]
pub enum LinkStatus {
    None,
    Fetching,
    Success(String),
    Error(String),
}

impl Default for LinkStatus {
    fn default() -> Self {
        LinkStatus::None
    }
}

#[derive(Default, Debug)]
pub struct App {
    pub input_buffer: String,
    pub running: bool,
    show_link: bool,
    link_status: LinkStatus,
    link_receiver: Option<tokio::sync::oneshot::Receiver<Result<String, String>>>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run<B: Backend>(mut self, mut terminal: Terminal<B>) -> Result<()> {
        self.running = true;

        while self.running {
            self.poll_link_fetch();
            terminal.draw(|frame| {
                log::warn!("—— Redrawing frame");
                self.render(frame);
            })?;
            self.handle_crossterm_events()?;

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Ok(())
    }

    pub fn poll_link_fetch(&mut self) {
        log::warn!("cheeeeeckingg");
        if let Some(mut receiver) = self.link_receiver.take() {
            match receiver.try_recv() {
                Ok(result) => {
                    self.link_receiver = None;
                    match result {
                        Ok(link) => {
                            log::warn!("Received success: {}", link);
                            self.link_status = LinkStatus::Success(link);
                            self.show_link = true;
                        }
                        Err(e) => {
                            log::warn!("Received error: {}", e);
                            self.link_status = LinkStatus::Error(e);
                        }
                    }
                }
                Err(tokio::sync::oneshot::error::TryRecvError::Empty) => {
                }
                Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                    log::warn!("Channel closed");
                    self.link_status = LinkStatus::Error("Channel closed".to_string());
                    self.link_receiver = None;
                }
            }
        }
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

        // Pass the link status to the welcome paragraph
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
        // Use poll with a short timeout to avoid blocking
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
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
                }

                if matches!(self.link_status, LinkStatus::None) {
                    self.start_link_fetch();
                }
            }
        }
    }

    fn start_link_fetch(&mut self) {
        self.link_status = LinkStatus::Fetching;
        self.show_link = true;

        let tx = tokio::sync::oneshot::channel();
        let sender = tx.0;
        let receiver = tx.1;

        tokio::spawn(async move {
            let result = discord::get_invite_link().await;
            let _ = sender.send(result);
        });

        self.link_receiver = Some(receiver);
    }
}
