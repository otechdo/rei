#![allow(clippy::too_many_lines)]
#![allow(clippy::multiple_crate_versions)]

use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Padding};
use ratatui::{CompletedFrame, Terminal};
use std::fmt::{Display, Formatter};
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

impl Display for PageIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PageIndex::Page0 => {
                write!(f, " /dev/null <== page 1/7 ==> Resolution ")
            }
            PageIndex::Page1 => {
                write!(f, " Problematics <== page 2/7 ==> Security ")
            }
            PageIndex::Page2 => {
                write!(f, " Resolution <== page 3/7 ==> Tests ")
            }
            PageIndex::Page3 => {
                write!(f, " Security <== page 4/7 ==> Requirements ")
            }
            PageIndex::Page4 => {
                write!(f, " Tests <== page 5/7 ==> Database ")
            }
            PageIndex::Page5 => {
                write!(f, " Requirements <== page 6/7 ==> Communication ")
            }
            PageIndex::Page6 => {
                write!(f, " Database <== page 7/7 ==> /dev/null ")
            }
        }
    }
}

struct Page {
    main_title: &'static str,
    areas: [TextArea<'static>; 4],
    current_page: PageIndex,
    titles: [&'static str; 4],
    describe: [&'static str; 4],
}

#[derive(Copy, Clone)]
struct App {}
impl App {
    fn new() -> Self {
        Self {}
    }
    fn render_commit<'a>(
        self,
        rei: &'a mut Terminal<CrosstermBackend<Stdout>>,
        areas: &mut Vec<TextArea>,
        describe: &mut Vec<&'static str>,
        titles: &mut Vec<&'static str>,
        main_title: &'static str,
        index: &PageIndex,
        witch: usize,
    ) -> std::io::Result<CompletedFrame<'a>> {
        for (i, area) in areas.iter_mut().enumerate() {
            area.set_block(
                Block::default()
                    .borders(Borders::all())
                    .title_alignment(Alignment::Left)
                    .title(format!(" {} ", titles[i])),
            );
            area.set_line_number_style(Style::default().fg(Color::White));
            area.set_cursor_style(Style::underlined(Style::default().fg(Color::White)));
            if i.eq(&witch) {
                activate(area, titles[i], describe[i]);
            } else {
                inactivate(area, titles[i], describe[i]);
            }
        }
        rei.draw(|f| {
            let parent_block = Block::default()
                .title(format!(" {main_title} "))
                .title_alignment(Alignment::Center)
                .title_bottom(format!("{index}"))
                .title_alignment(Alignment::Center)
                .borders(Borders::all());

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(2)
                .horizontal_margin(4)
                .spacing(2)
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
            f.render_widget(&areas[0], left_chunks[0]);
            f.render_widget(&areas[2], left_chunks[1]);

            f.render_widget(&areas[1], right_chunks[0]);
            f.render_widget(&areas[3], right_chunks[1]);
        })
    }
}
fn inactivate(textarea: &mut TextArea<'_>, title: &str, describe: &str) {
    textarea.set_cursor_line_style(Style::default());
    textarea.set_cursor_style(Style::default());
    textarea.set_block(
        Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(Color::DarkGray))
            .title(format!(" {title} "))
            .title_bottom(format!(" {describe} ")),
    );
}
fn update(witch: usize, page: &mut Page) {
    for (i, area) in page.areas.iter_mut().enumerate() {
        if witch.eq(&i) {
            activate(area, page.main_title, page.describe[i]);
        } else {
            inactivate(area, page.main_title, page.describe[i]);
        }
    }
}
fn activate(textarea: &mut TextArea<'_>, title: &str, describe: &str) {
    let mut lines: Vec<usize> = Vec::new();
    textarea.lines().iter().for_each(|line| {
        lines.push(line.chars().count());
    });
    let block = Block::default()
        .rapid_blink()
        .borders(Borders::all())
        .border_type(BorderType::Rounded)
        .padding(Padding::new(0, 2, 0, 2))
        .title_alignment(Alignment::Center)
        .title(format!(" {title} "))
        .title_bottom(format!(" {describe} "));
    let mut red = false;
    let mut orange = false;
    let mut yellow = false;
    let mut green = false;
    for line in &lines {
        if line.gt(&72) {
            red = true;
            orange = false;
            yellow = false;
            green = false;
        } else if line.le(&35) && line.ge(&20) {
            green = true;
        } else if line.le(&50) && line.ge(&35) {
            yellow = true;
        } else if line.lt(&72) && line.ge(&50) {
            orange = true;
        }
    }
    if red {
        textarea.set_block(
            block
                .style(Style::default().fg(Color::Red))
                .border_style(Style::default().fg(Color::Red))
                .title(format!(
                    " {title} ( a line is superior to the max lines length )"
                )),
        );
    } else if yellow {
        textarea.set_block(
            block
                .style(Style::default().fg(Color::Yellow))
                .border_style(Style::default().fg(Color::Yellow)),
        );
    } else if orange {
        textarea.set_block(
            block
                .style(Style::default().fg(Color::Rgb(255, 165, 0)))
                .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
        );
    } else if green {
        textarea.set_block(
            block
                .style(Style::default().fg(Color::Green))
                .border_style(Style::default().fg(Color::Green)),
        );
    } else {
        textarea.set_block(
            block
                .style(Style::default().fg(Color::White))
                .border_style(Style::default().fg(Color::White)),
        );
    }
}
fn commit(rei: &mut Terminal<CrosstermBackend<Stdout>>, app: App) -> std::io::Result<()> {
    let mut pages = [
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
            describe: [
                "Indicate the problem title",
                "Describe the problem in detail",
                "Indicate the steps necessary to reproduce the problem",
                "Describe the expected behavior",
            ],
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
            titles: ["Before", "After", "Results", "Samples"],
            describe: [
                "Describe the state before the implementation of the resolution",
                "Describe the state after the implementation of the resolution",
                "Describe the results obtained after the implementation",
                "Give examples of the use of the resolution",
            ],
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
            describe: [
                "Describe potential security vulnerabilities",
                "Describe the quality aspects of the code and the solution",
                "Indicate the security and conformity standards met",
                "Describe potential security risks",
            ],
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
            titles: ["Added", "Updated", "Deleted", "Platforms"],
            describe: [
                "Describe the new tests added",
                "Describe the updated tests",
                "Describe the deleted tests",
                "Indicate the platforms on which the tests were carried out",
            ],
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
            titles: ["Breaking changes", "New needed dependencies", "New packages needed", "Rollback"],
            describe: [
                "Indicate if the code have a breaking changes",
                "Indicate the new needed dependencies",
                "Indicate the new needed packages name's",
                "Describe the rollback process in case of a problem",
            ],
        },
        Page {
            main_title: "Database",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: PageIndex::Page5,
            titles: ["Up", "Down", "Changes", "Why"],
            describe: [
                "What's it's created on up",
                "What's it's removed on down",
                "Describe the migrations results",
                "Describe the reason of the update",
            ],
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
            describe: [
                "Authors name's",
                "Indicate the project testers",
                "Indicate comments and feedback",
                "Indicate important remarks and observations",
            ],
        },
    ];
    let mut page: usize = 0;
    let mut witch: usize = 0;

    loop {
        match pages[page].current_page {
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
                        &mut pages[page].areas.to_vec(),
                        &mut pages[page].describe.to_vec(),
                        &mut pages[page].titles.to_vec(),
                        pages[page].main_title,
                        &pages[page].current_page,
                        witch,
                    )
                    .is_ok());
            }
        }
        if let Ok(Event::Key(key)) = event::read() {
            if key.code == KeyCode::Esc {
                break;
            } else if key.code == KeyCode::PageUp {
                witch = 0;
                if page.lt(&(pages.len() - 1)) {
                    page += 1;
                }
            } else if key.code == KeyCode::PageDown {
                witch = 0;
                if page.gt(&0) {
                    page -= 1;
                }
            } else if key.code == KeyCode::F(7) {
                if witch.lt(&3) {
                    witch += 1;
                    update(witch, &mut pages[page])
                }
            } else if key.code == KeyCode::F(5) {
                if witch.gt(&0) {
                    witch -= 1;
                }
            } else {
                pages[page].areas.get_mut(witch).expect("").input(key);
            }
        }
    }
    Ok(())
}

fn dojo(rei: &mut Terminal<CrosstermBackend<Stdout>>, app: App) {
    loop {
        if let Event::Key(key) = event::read().unwrap() {
            if key.code == KeyCode::Esc {
                break;
            }
            if key.code == KeyCode::F(2) && commit(rei, app).is_ok() {
                break;
            }
        }
    }
}
fn main() {
    let app = App::new();

    let mut rei = ratatui::init();
    dojo(&mut rei, app);
    ratatui::restore();
}
