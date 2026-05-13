use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::{DefaultTerminal, Frame};
use ratatui::{prelude::*, widgets::*};
use std::process::Command;
fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let diff = get_git_diff();
    let diff_chunks = get_diff_chunks(&diff);
    loop {
        terminal.draw(|frame| render_diff(frame, &diff_chunks))?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn get_diff_chunks(diff: &str) -> Vec<Line> {
    diff.split("\n")
        .map(|str| {
            if str.starts_with("diff --git") {
                Line::from(str).style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else if str.starts_with("+") {
                Line::from(str).style(Style::default().fg(Color::Green))
            } else if str.starts_with("-") {
                Line::from(str).style(Style::default().fg(Color::Red))
            } else if str.starts_with("@@") {
                Line::from(str).style(Style::default().fg(Color::Cyan))
            } else {
                Line::from(str)
            }
        })
        .collect()
}

fn render_diff(frame: &mut Frame, diff: &[Line]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // фиксированно 3 строки
            Constraint::Min(0),    // остальное
        ])
        .split(frame.area());

    let header = Paragraph::new("GitTaTUI").block(Block::default().borders(Borders::ALL));
    let body = Paragraph::new(diff.to_vec()).block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, chunks[0]);
    frame.render_widget(body, chunks[1]);
}

// hello
fn get_git_diff() -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "git diff"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("git diff")
            .output()
            .expect("failed to execute process")
    };

    String::from_utf8_lossy(&output.stdout).into_owned()
}
