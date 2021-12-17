use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
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

    let total_checked = db.total_checked();

    let mut prev_board = String::new();
    // let mut print = |task: Task, i: usize| {
    //     if prev_board.is_empty() {
    //         prev_board = task.board;
    //         print::header(total_checked, total_tasks, &prev_board)
    //     } else if prev_board != task.board {
    //         prev_board = task.board;
    //         print::header(total_checked, total_tasks, &prev_board)
    //     }

    //     if task.note {
    //         print::note(i, &task.content, total_tasks);
    //     } else {
    //         print::task(i, task.checked, &task.content, 0, total_tasks);
    //     }
    // };
    default_board.extend(other_boards);
    let mut i = 1;

    let mut out = Vec::new();
    for task in default_board {
        if prev_board.is_empty() {
            prev_board = task.board.clone();
            out.push(Spans::from(vec![
                Span::styled(
                    format!("{}.", task.board.clone()),
                    Style::default().add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(" [1/3]", Style::default().fg(Color::DarkGray)),
            ]));
        } else if prev_board != task.board {
            prev_board = task.board.clone();
            out.push(Spans::from(vec![
                Span::styled(
                    format!("{}.", task.board.clone()),
                    Style::default().add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(" [1/3]", Style::default().fg(Color::DarkGray)),
            ]));
        };
        if task.note {
            out.push(Spans::from(vec![
                Span::styled(format!("{}.", i), Style::default().fg(Color::DarkGray)),
                Span::styled("*", Style::default().fg(Color::Magenta)),
                Span::raw(task.content.clone()),
            ]));
        } else {
            if task.checked {
                out.push(Spans::from(vec![
                    Span::styled(format!("{}.", i), Style::default().fg(Color::DarkGray)),
                    Span::styled("  [√] ", Style::default().fg(Color::Magenta)),
                    Span::raw(task.content.clone()),
                    // Span::styled(" 75d", Style::default().fg(Color::DarkGray)),
                ]));
            } else {
                out.push(Spans::from(vec![
                    Span::styled(format!("{}.", i), Style::default().fg(Color::DarkGray)),
                    Span::styled("  [ ] ", Style::default().fg(Color::Magenta)),
                    Span::raw(task.content.clone()),
                ]));
            }
        }

        i += 1;
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

    let task = Paragraph::new(out)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    let footer = Paragraph::new(footer)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    let size = f.size();
    f.render_widget(task, size);
    // f.render_widget(task, Rect::new(3, 1, size.width - 3, size.height - 1));
    // f.render_widget(footer, Rect::new(1, 3, size.width - 1, size.height - 3));
}
