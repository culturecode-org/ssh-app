use ratatui::{
    text::{Line, Text, Span},
    style::{Color, Style, Modifier},
};

pub enum BrandColors {
    Lavender,
    Peach,
    Mint,
    Coral,
    Light,
    Dark,
    Gray,
    DarkGray,
}

impl BrandColors {
    pub fn color(&self) -> Color {
        match self {
            BrandColors::Lavender => Color::Rgb(195, 187, 227),
            BrandColors::Peach => Color::Rgb(250, 209, 163),
            BrandColors::Mint => Color::Rgb(204, 235, 177),
            BrandColors::Coral => Color::Rgb(240, 119, 119),
            BrandColors::Light => Color::Rgb(244, 241, 230),
            BrandColors::Dark => Color::Rgb(37, 35, 34),
            BrandColors::Gray => Color::Rgb(225, 225, 225),
            BrandColors::DarkGray => Color::Rgb(124, 124, 124)
        }
    }
}

pub fn _culturecode_logo_short() -> Text<'static> {
    let lines = vec![
        Line::from(vec![
            Span::styled("culture",
                Style::default()
                    .fg(BrandColors::Light.color())
                    .add_modifier(Modifier::BOLD)),
            Span::styled("code",
                Style::default()
                    .fg(BrandColors::Dark.color())
                    .bg(BrandColors::Light.color())),
        ]),
        Line::from(vec![
            Span::raw("       "), // spacing to center the color bar
            Span::styled("▀",
                Style::default()
                    .fg(BrandColors::Lavender.color())),
            Span::styled("▀",
                Style::default()
                    .fg(BrandColors::Peach.color())),
            Span::styled("▀",
                Style::default()
                    .fg(BrandColors::Mint.color())),
            Span::styled("▀",
                Style::default()
                    .fg(BrandColors::Coral.color())),
        ]),
    ];
    Text::from(lines)
}

pub fn culturecode_logo_long() -> Text<'static> {
    let lines = vec![
        Line::raw(""),
        Line::from(vec![
            Span::styled("culture",
                Style::default()
                    .fg(BrandColors::Light.color())
                    // .bg(BrandColors::Dark.color())
                    .add_modifier(Modifier::BOLD)),
            Span::styled("code",
                Style::default()
                    .fg(BrandColors::Dark.color())
                    .bg(BrandColors::Light.color()))
        ]),
        Line::from(vec![
            Span::raw("       "), // spacing to center the color bar
            Span::styled("█",
                Style::default()
                    .fg(BrandColors::Lavender.color())),
            Span::styled("█",
                Style::default()
                    .fg(BrandColors::Peach.color())),
            Span::styled("█",
                Style::default()
                    .fg(BrandColors::Mint.color())),
            Span::styled("█",
                Style::default()
                    .fg(BrandColors::Coral.color())),
        ]),
    ];
    Text::from(lines)
}

pub fn culturecode_logo_tail() -> Line<'static> {
    Line::from(vec![
        Span::raw("       "),
        Span::styled("█",
            Style::default()
                .fg(BrandColors::Lavender.color())),
        Span::styled("█",
            Style::default()
                .fg(BrandColors::Peach.color())),
        Span::styled("█",
            Style::default()
                .fg(BrandColors::Mint.color())),
        Span::styled("█",
            Style::default()
                .fg(BrandColors::Coral.color())),
    ])
}
