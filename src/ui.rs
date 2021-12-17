use tui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Paragraph, Wrap},
    Frame,
};

use crate::{app::App, database::Database};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let db = Database::new();
    let total_tasks = db.total_tasks();

    if total_tasks == 0 {
        // print::help_message();
        return;
    }

    let mut default_board = db.get_default_board();
    let other_boards = db.get_other_boards();

    //TODO:
    // let tasks = db.get_tasks();

    let total_checked = db.total_checked();

    default_board.extend(other_boards);

    let size = f.size();
    let mut buffer = Vec::new();
    let mut i = 1;

    let boards = db.get_boards();

    for board in boards {
        let header = Spans::from(vec![
            Span::raw("ㅤ"),
            Span::styled(
                format!("{}:", board.name),
                Style::default().add_modifier(Modifier::UNDERLINED),
            ),
            Span::styled(
                format!(" [{}/{}]", total_checked, total_tasks),
                Style::default().fg(Color::DarkGray),
            ),
        ]);

        buffer.push(header);

        let tasks: Vec<_> = board
            .tasks
            .iter()
            .map(|task| {
                //weird work around for incrimenting count
                i += 1;
                if task.note {
                    Spans::from(vec![
                        Span::raw("ㅤㅤ"),
                        Span::styled(format!("{}.", i - 1), Style::default().fg(Color::DarkGray)),
                        Span::styled("*", Style::default().fg(Color::Magenta)),
                        Span::raw(task.content.clone()),
                        Span::styled(" 75d", Style::default().fg(Color::DarkGray)),
                    ])
                } else {
                    if task.checked {
                        Spans::from(vec![
                            Span::raw("ㅤㅤ"),
                            Span::styled(
                                format!("{}.", i - 1),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::styled("  [√] ", Style::default().fg(Color::Magenta)),
                            Span::raw(task.content.clone()),
                            Span::styled(" 75d", Style::default().fg(Color::DarkGray)),
                        ])
                    } else {
                        Spans::from(vec![
                            Span::raw("ㅤㅤ"),
                            Span::styled(
                                format!("{}.", i - 1),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::styled("  [ ] ", Style::default().fg(Color::Magenta)),
                            Span::raw(task.content.clone()),
                            Span::styled(" 75d", Style::default().fg(Color::DarkGray)),
                        ])
                    }
                }
            })
            .collect();

        buffer.extend(tasks);
    }

    let footer = vec![
        Spans::from(vec![Span::styled(
            "0% of all tasks completed",
            Style::default().fg(Color::DarkGray),
        )]),
        Spans::from(vec![
            Span::styled("3", Style::default().fg(Color::Green)),
            Span::styled(" done · ", Style::default().fg(Color::DarkGray)),
            Span::styled("10", Style::default().fg(Color::Magenta)),
            Span::styled(" pending · ", Style::default().fg(Color::DarkGray)),
            Span::styled("1", Style::default().fg(Color::Blue)),
            Span::styled(" notes", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    buffer.extend(footer);

    let buffer = Paragraph::new(buffer)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(buffer, size);
}
