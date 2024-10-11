use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::Terminal;
use std::io::{BufRead, Stdout};
use std::process::Command;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use tui_textarea::{Input, Key, TextArea};

fn inactivate(textarea: &mut TextArea<'_>, title:&str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::DarkGray))
            .title(format!(" {title} ")),
    );
}
fn update(witch:usize,textarea: &mut [TextArea],titles : &[&str])
{
    assert_eq!(textarea.len(),titles.len());

    for x in 0..textarea.len() {
        if x.ne(&witch) {
            inactivate(&mut textarea[x],titles[x]);
        }else{
            activate(&mut textarea[x],titles[witch]);
        }
    }
}
fn activate(textarea: &mut TextArea<'_>,title:&str) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .title(format!(" {title} ")).title_alignment(Alignment::Center),
    );
}

fn commit(rei: &mut Terminal<CrosstermBackend<Stdout>>) -> std::io::Result<()> {

    let mut textarea = [TextArea::default(), TextArea::default(),TextArea::default(),TextArea::default()];
    let titles = ["Title","summary","",""];
    let diff = String::from_utf8_lossy(Command::new("git").arg("diff").arg("-p").current_dir(".").output().expect("git not found").stdout.as_ref()).to_string();
    let status = String::from_utf8_lossy(Command::new("git").arg("status").current_dir(".").output().expect("git not found").stdout.as_ref()).to_string();
    textarea[0].set_block(Block::default().borders(Borders::ALL));

    textarea[1].set_block(Block::default().borders(Borders::ALL));

    textarea[2].set_block(Block::default().borders(Borders::ALL));

    textarea[3].set_block(Block::default().borders(Borders::ALL));

    let mut which = 0;

    update(which,&mut textarea,&titles);

    loop {
        rei.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(f.area());

            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(&[Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

            let two = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)])
                .split(chunks[1]);

            let horizontal_chunks_two = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(&[Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(two[0]);

            f.render_widget(&textarea[0], horizontal_chunks[0]);
            f.render_widget(&textarea[1], horizontal_chunks[1]);
            f.render_widget(&textarea[2], horizontal_chunks_two[0]);
            f.render_widget(&textarea[3], horizontal_chunks_two[1]);

        })?;

        match crossterm::event::read()?.into() {
            Input { key: Key::Esc, .. } => break,
            Input { key: Key::Up, ..} => {
                if !which.gt(&textarea.len()) {
                   which +=1;
                    update(which,&mut textarea,&titles);
                }
            },
            Input { key: Key::Down, ..} => {
                if !which.le(&0) {
                    which -=1;
                    update(which,&mut textarea,&titles);
                }
            },
            input => {
                textarea[which].input(input);
            }
        }
    }
    Ok(())
}

fn dojo(rei: &mut Terminal<CrosstermBackend<Stdout>>) {
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.code == KeyCode::Char('q') {
                break;
            } else if key.code == KeyCode::F(2) {
                if commit(rei).is_ok() {
                    break;
                }
            }
        }
    }
}
fn main() {
    let mut rei = ratatui::init();
    dojo(&mut rei);
    ratatui::restore();
}
