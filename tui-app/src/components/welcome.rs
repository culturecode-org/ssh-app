use ratatui::{
    text::{Line, Span, Text},
    widgets::Paragraph,
    style::{Style, Modifier},
    layout::Alignment,
};
use crate::brand::*;

pub fn welcome_paragraph<'a>(show_link: bool, link_content: Option<&'a str>) -> Paragraph<'a> {
    let mut lines = vec![
        Line::from(
            Span::styled("────────────────────────────────────────────────────",
                Style::default()
                    .fg(BrandColors::DarkGray.color())
                    .add_modifier(Modifier::BOLD))
        ),
        Line::from(vec![
            Span::styled("Congratulations",
                Style::default()
                    .fg(BrandColors::Light.color())
                    .add_modifier(Modifier::BOLD)),
            Span::styled(", your ",
                Style::default()
                    .fg(BrandColors::Gray.color())
            ),
            Span::styled("PublicKey",
                Style::default()
                    .fg(BrandColors::Lavender.color())
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("has been ",
                Style::default()
                    .fg(BrandColors::Gray.color())
            ),
            Span::styled("accepted",
                Style::default()
                    .fg(BrandColors::Peach.color())
                    .add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(
            Span::styled("╭─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─╮",
                Style::default()
                    .fg(BrandColors::DarkGray.color()))
        ),
    ];

    if show_link {
        match link_content {
            Some(content) => {
                if content.starts_with("https://") {
                    // Valid link
                    lines.push(Line::from(vec![
                        Span::styled("|          ",
                            Style::default()
                                .fg(BrandColors::DarkGray.color())),
                        Span::styled(content,
                            Style::default()
                                .fg(BrandColors::Mint.color())
                                .add_modifier(Modifier::UNDERLINED | Modifier::BOLD)),
                        Span::styled("          |",
                            Style::default()
                                .fg(BrandColors::DarkGray.color()),
                        ),
                    ]));
                } else if content.starts_with("Fetching") {
                    // Fetching state
                    lines.push(Line::from(vec![
                        Span::styled("|         ",
                            Style::default()
                                .fg(BrandColors::DarkGray.color())),
                        Span::styled(content,
                            Style::default()
                                .fg(BrandColors::Mint.color())
                                .add_modifier(Modifier::SLOW_BLINK | Modifier::ITALIC)),
                        Span::styled("         |",
                            Style::default()
                                .fg(BrandColors::DarkGray.color()),
                        ),
                    ]));
                } else {
                    // Error state
                    lines.push(Line::from(vec![
                        Span::styled("|     ",
                            Style::default()
                                .fg(BrandColors::DarkGray.color())),
                        Span::styled(content,
                            Style::default()
                                .fg(BrandColors::Coral.color())
                                .add_modifier(Modifier::BOLD)),
                        Span::styled("     |",
                            Style::default()
                                .fg(BrandColors::DarkGray.color()),
                        ),
                    ]));
                }
            }
            None => {
                // No link content, show loading message
                lines.push(Line::from(vec![
                    Span::styled("|         ",
                        Style::default()
                            .fg(BrandColors::DarkGray.color())),
                    Span::styled("Loading...",
                        Style::default()
                            .fg(BrandColors::Mint.color())
                            .add_modifier(Modifier::SLOW_BLINK | Modifier::ITALIC)),
                    Span::styled("         |",
                        Style::default()
                            .fg(BrandColors::DarkGray.color()),
                    ),
                ]));
            }
        }
    } else {
        lines.push(Line::from(vec![
            Span::styled("|         ",
                Style::default()
                    .fg(BrandColors::DarkGray.color())),
            Span::styled("press ",
                Style::default()
                    .fg(BrandColors::Mint.color())
                    .add_modifier(Modifier::SLOW_BLINK | Modifier::ITALIC)),
            Span::styled("'D' ",
                Style::default()
                    .fg(BrandColors::Mint.color())
                    .add_modifier(Modifier::SLOW_BLINK | Modifier::BOLD)),
            Span::styled("to reveal your link",
                Style::default()
                    .fg(BrandColors::Mint.color())
                    .add_modifier(Modifier::SLOW_BLINK | Modifier::ITALIC)),
            Span::styled("         |",
                Style::default()
                    .fg(BrandColors::DarkGray.color()),
            ),
        ]));
    }

    lines.push(Line::from(
        Line::from(
            Span::styled("╰─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─╯",
                Style::default()
                    .fg(BrandColors::DarkGray.color()))
        ),
    ));

    if show_link && link_content.is_some() && !link_content.unwrap().starts_with("Error") {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("This is the beginning — ",
                Style::default()
                    .fg(BrandColors::Gray.color())
                    .add_modifier(Modifier::ITALIC)),
            Span::styled("welcome",
                Style::default()
                    .fg(BrandColors::Lavender.color())
                    .add_modifier(Modifier::BOLD)
            ),
            Span::styled("to",
                Style::default()
                    .fg(BrandColors::Peach.color())
                    .add_modifier(Modifier::BOLD)
            ),
            Span::styled("the",
                Style::default()
                    .fg(BrandColors::Mint.color())
                    .add_modifier(Modifier::BOLD)
            ),
            Span::styled("culture",
                Style::default()
                    .fg(BrandColors::Coral.color())
                    .add_modifier(Modifier::BOLD)
            ),
        ]));
    }

    if show_link && link_content.is_some() && !link_content.unwrap().starts_with("Error") {
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("to exit type: ",
                Style::default()
                    .fg(BrandColors::DarkGray.color())),
            Span::styled("'200'",
                Style::default()
                    .fg(BrandColors::Light.color())
                    .add_modifier(Modifier::BOLD))
        ]));
    }

    Paragraph::new(Text::from(lines)).alignment(Alignment::Center)
}

pub fn welcome_paragraph_end() -> Paragraph<'static> {
    let mut lines = vec![];
    lines.push(Line::from(
        Span::styled("────────────────────────────────────────────────────",
            Style::default()
                .fg(BrandColors::DarkGray.color())
                .add_modifier(Modifier::BOLD))
    ));
    lines.extend(std::iter::repeat_with(culturecode_logo_tail)
        .take(33)
        .collect::<Vec<_>>());

    Paragraph::new(Text::from(lines)).alignment(Alignment::Center)
}
