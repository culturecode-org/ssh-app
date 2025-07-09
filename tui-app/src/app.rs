use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Layout, Constraint, Direction, Alignment}, 
    widgets::{Block, Paragraph},
    DefaultTerminal, Frame,
    style::Style
};
use crate::brand;

use crate::components::welcome::*;

#[derive(Debug, Default)]
pub struct App {
    input_buffer: String,
    running: bool,
    show_link: bool,
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
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
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

        let paragraph1 = welcome_paragraph(self.show_link);
        frame.render_widget(paragraph1, inner1);

        let block2 = Block::default();
        let inner2 = block2.inner(content_layout[1]);
        frame.render_widget(block2, content_layout[1]);

        let paragraph2 = welcome_paragraph_end();
        frame.render_widget(paragraph2, inner2);
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

    fn on_key_event(&mut self, key: KeyEvent) {
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
            }
        }
    }
}
