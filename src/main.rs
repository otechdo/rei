#![allow(clippy::too_many_lines)]
#![allow(clippy::multiple_crate_versions)]

use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders};
use ratatui::{CompletedFrame, Terminal};
use std::io::Stdout;
use tui_textarea::TextArea;
enum PageIndex {
    Page0,
    Page1,
    Page2,
    Page3,
    Page4,
    Page5,
    Page6,
}

struct Page {
    main_title: &'static str,
    areas: [TextArea<'static>; 4],
    current_page: PageIndex,
    titles: [&'static str; 4],
}

#[derive(Copy, Clone)]
struct App {
    current_page: usize,
    max_pages: usize,
}
impl App {
    fn new(max_pages: usize) -> Self {
        Self {
            current_page: 0,
            max_pages,
        }
    }
    fn render_commit<'a>(
        self,
        rei: &'a mut Terminal<CrosstermBackend<Stdout>>,
        areas: &mut Vec<TextArea>,
        titles: &mut Vec<&'static str>,
        main_title: &'static str,
    ) -> std::io::Result<CompletedFrame<'a>> {
        for (i, area) in areas.iter_mut().enumerate() {
            area.set_block(
                Block::default()
                    .borders(Borders::all())
                    .style(Style::default())
                    .title(format!(" {} ", titles[i])),
            );
        }
        rei.draw(|f| {
            let parent_block = Block::default()
                .title(format!(" {main_title} "))
                .title_alignment(Alignment::Center)
                .borders(Borders::all());

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(f.area());

            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

            let right_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);
            f.render_widget(parent_block, f.area());
            if self.current_page == 0 {
                f.render_widget(&areas[0], left_chunks[0]);
                f.render_widget(&areas[2], left_chunks[1]);

                f.render_widget(&areas[1], right_chunks[0]);
                f.render_widget(&areas[3], right_chunks[1]);
            } else {
                let x = self.current_page; // 1
                let a = x * 2 + 2; // 1 *2 +2 = 4
                let b = a + 2; // 4+2 =6

                let c = x * 2 + 3; // 1 *2 +3 = 5
                let d = a + 3; // 4+3 =7;
                f.render_widget(&areas[a], left_chunks[0]);
                f.render_widget(&areas[b], left_chunks[1]);

                f.render_widget(&areas[c], right_chunks[0]);
                f.render_widget(&areas[d], right_chunks[1]);
            }
        })
    }
    fn next_page(&mut self) -> &mut Self {
        if self.current_page == self.max_pages {
            self
        } else {
            self.current_page += 1;
            self
        }
    }

    fn prev_page(&mut self) -> &mut Self {
        if self.current_page.gt(&0) {
            self.current_page -= 1;
        }
        self
    }
}
fn inactivate(textarea: &mut TextArea<'_>, title: &str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::all())
            .style(Style::default().fg(Color::DarkGray))
            .title(format!(" {title} ")),
    );
}
fn update(witch: usize, textarea: &mut [TextArea], titles: &[&str]) {
    assert_eq!(textarea.len(), titles.len());
    for x in 0..textarea.len() {
        if x.ne(&witch) {
            inactivate(&mut textarea[x], titles[x]);
        } else {
            activate(&mut textarea[x], titles[witch]);
        }
    }
}
fn activate(textarea: &mut TextArea<'_>, title: &str) {
    textarea.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));
    textarea.set_cursor_style(Style::default().add_modifier(Modifier::REVERSED));
    let mut lines: Vec<usize> = Vec::new();
    textarea.lines().iter().for_each(|line| {
        lines.push(line.chars().count());
    });
    let block = Block::default()
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    let mut red = false;
    let mut blue = false;
    let mut cyan = false;
    let mut green = false;
    for line in &lines {
        if line.gt(&72) {
            red = true;
            blue = false;
            cyan = false;
            green = false;
        } else if line.le(&35) && line.ge(&20) {
            blue = true;
        } else if line.le(&50) && line.ge(&35) {
            cyan = true;
        } else if line.lt(&72) && line.ge(&50) {
            green = true;
        }
    }
    if red {
        textarea.set_block(
            block
                .borders(Borders::all())
                .style(Style::default().fg(Color::Red))
                .border_style(Style::default().fg(Color::Red))
                .title(format!(
                    " {title} ( a line is superior to the max lines length )"
                )),
        );
    } else if blue {
        textarea.set_block(
            block
                .borders(Borders::all())
                .style(Style::default().fg(Color::Blue))
                .border_style(Style::default().fg(Color::Blue))
                .title(format!(" {title} ")),
        );
    } else if cyan {
        textarea.set_block(
            block
                .borders(Borders::all())
                .style(Style::default().fg(Color::Cyan))
                .border_style(Style::default().fg(Color::Cyan))
                .title(format!(" {title} ")),
        );
    } else if green {
        textarea.set_block(
            block
                .borders(Borders::all())
                .style(Style::default().fg(Color::Green))
                .border_style(Style::default().fg(Color::Green))
                .title(format!(" {title} ")),
        );
    } else {
        textarea.set_block(
            block
                .borders(Borders::all())
                .style(Style::default().fg(Color::White))
                .border_style(Style::default().fg(Color::White))
                .title(format!(" {title} ")),
        );
    }
}
fn commit(rei: &mut Terminal<CrosstermBackend<Stdout>>) -> std::io::Result<()> {
    let app = App::new(6);
    let pages = [
        Page {
            main_title: "Problematic",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: PageIndex::Page0,
            titles: ["Title", "Description", "Steps to reproduce", "Expectation"],
        },
        Page {
            main_title: "Resolution",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: PageIndex::Page1,
            titles: ["Before", "After", "Samples", "Results"],
        },
        Page {
            main_title: "Security",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: PageIndex::Page2,
            titles: ["Vulnerability", "Quality", "Conformity", "Risk"],
        },
        Page {
            main_title: "Tests",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: PageIndex::Page3,
            titles: ["Added", "Updated", "Removed", "Platforms"],
        },
        Page {
            main_title: "Requirements",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: PageIndex::Page4,
            titles: ["Database", "Dependencies", "Packages", "Rollback"],
        },
        Page {
            main_title: "Timeline",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: PageIndex::Page5,
            titles: ["Phase", "Results", "Status", "Updates"],
        },
        Page {
            main_title: "Communication",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: PageIndex::Page6,
            titles: ["Authors", "Testers", "Comments", "Notes"],
        },
    ];
    let mut witch: usize = 0;

    loop {
        match pages[witch].current_page {
            PageIndex::Page0
            | PageIndex::Page1
            | PageIndex::Page2
            | PageIndex::Page3
            | PageIndex::Page4
            | PageIndex::Page5
            | PageIndex::Page6 => {
                assert!(app
                    .render_commit(
                        rei,
                        &mut pages[witch].areas.to_vec(),
                        &mut pages[witch].titles.to_vec(),
                        pages[witch].main_title
                    )
                    .is_ok());
            }
        }
        if let Ok(Event::Key(key)) = event::read() {
            if key.code == KeyCode::Esc {
                break;
            } else if key.code == KeyCode::PageUp {
                if witch.lt(&(pages.len() - 1)) {
                    witch += 1;
                }
            } else if key.code == KeyCode::PageDown {
                if witch.gt(&0) {
                    witch -= 1;
                }
            }
        }
    }
    Ok(())
}

fn dojo(rei: &mut Terminal<CrosstermBackend<Stdout>>) {
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.code == KeyCode::Esc {
                break;
            }
            if key.code == KeyCode::F(2) && commit(rei).is_ok() {
                break;
            }
        }
    }
}
fn main() {
    let mut rei = ratatui::init();
    dojo(&mut rei);
    ratatui::restore();
}
