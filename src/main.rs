use ratatui::{DefaultTerminal, Frame};
use std::process::Command;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}


fn render(frame: &mut Frame) {
    let output = gitDiff();    

    frame.render_widget(output, frame.area());
}


fn gitDiff() -> String {
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
