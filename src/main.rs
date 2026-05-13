use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
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

struct App {
    diff_lines: Vec<Line<'static>>,
    scroll_offset: u16,
}

impl App {
    fn new() -> Self {
        let diff = get_git_diff();

        Self {
            diff_lines: get_diff_chunks(&diff),
            scroll_offset: 0,
        }
    }
    fn scroll_down(&mut self) {
        if (self.diff_lines.len() as u16) > self.scroll_offset {
            self.scroll_offset += 1;
        }
    }

    fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|frame| render_diff(frame, &app))?;
        // if crossterm::event::read()?.is_key_press() {
        //     break Ok(());
        // }
        match crossterm::event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Char('j') | KeyCode::Down => app.scroll_down(),
                KeyCode::Char('k') | KeyCode::Up => app.scroll_up(),
                KeyCode::Char('q') | KeyCode::Esc => break Ok(()),
                _ => {}
            },
            _ => {}
        }
    }
}

fn style_line(str: &str) -> Style {
    if str.starts_with("diff --git") {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else if str.starts_with("+++ ") || str.starts_with("--- ") {
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD)
    } else if str.starts_with("+") {
        Style::default().fg(Color::Green)
    } else if str.starts_with("-") {
        Style::default().fg(Color::Red)
    } else if str.starts_with("@@") {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default()
    }
}

fn get_diff_chunks(diff: &str) -> Vec<Line<'static>> {
    diff.split("\n")
        .map(|str| Line::from(str.to_string()).style(style_line(str)))
        .collect()
}

fn render_diff(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let diff_lines = &app.diff_lines;

    let body_view_port_height = chunks[1].height.saturating_sub(2);
    let max_offset = (diff_lines.len() as u16).saturating_sub(body_view_port_height);
    let effective_offset = app.scroll_offset.min(max_offset);

    let header = Paragraph::new("GitTaTUI").block(Block::default().borders(Borders::ALL));
    let body = Paragraph::new(diff_lines.to_vec())
        .block(Block::default().borders(Borders::ALL))
        .scroll((effective_offset, 0));

    let status_bar = Paragraph::new("Down: [j/<ArrowDown>] | Up: k/<ArrowUp> | Exit: q/<Esc>")
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, chunks[0]);
    frame.render_widget(body, chunks[1]);
    frame.render_widget(status_bar, chunks[2]);
}

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
