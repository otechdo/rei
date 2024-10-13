#![allow(clippy::too_many_lines)]
#![allow(clippy::multiple_crate_versions)]

use crate::PageIndex::{Page0, Page1, Page2, Page3, Page4, Page5, Page6, Page7, Page8};
use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Padding};
use ratatui::{CompletedFrame, Terminal};
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::io::Stdout;
use std::process::{Command, Stdio};
use tui_textarea::TextArea;

const COMMIT_TEMPLATE :&str = "# %title%

%description%

### Steps to reproduce

%steps%

### Expected behavior

%expected_description%

## Resolution

Explains the state of the system before and after the changes

### Before

%system_before%

### After

%system_after%

### Expectation

%expectation%

### Samples

%samples%

## Security

The security section discusses any security-related impacts that may arise from the newly added authentication system

### Vulnerabilities

%vulnerabilities%

### Quality

%qualities%

### Conformity

%conformity%

### Risk

%risk%

## Tests

### Added

%tests_added%

### Updated

%tests_updated%

### Deleted

%test_deleted%

### Platforms

%tested_platforms%

## Requirements

### BREAKING CHANGES

%breaking_changes%

### Dependencies

%dependencies%

### Rollback

%rollbacks%

## Database

### Up

%db_up%

### Down

%db_down%

### Changes

%db_changes%

### Why

%why_db_changes%

## Communication

### Authors

%authors%

### Testers

%testers%

### Comments

%comments%

### Notes

%notes%

## Ideas

### News headline

%news_headline%

### Workflow

%workflow%

### Examples

%workflows_samples%

### Technical considerations

%technical_considerations%

## Next

### Description

%next_description%

### Motivation

%next_motivation%

### Why implement it

%next_reasons%

### Related Links

%next_links%

";

#[derive(Serialize, Default)]
pub struct Commit {
    pub title: String,
    pub description: String,
    pub steps: String,
    pub expected_description: String,
    pub system_before: String,
    pub system_after: String,
    pub expectation: String,
    pub samples: String,
    pub vulnerabilities: String,
    pub qualities: String,
    pub conforms: String,
    pub risks: String,
    pub tests_added: String,
    pub tests_updated: String,
    pub tests_deleted: String,
    pub platforms: String,
    pub breaking_changes: String,
    pub dependencies: String,
    pub rollbacks: String,
    pub up_migrations: String,
    pub down_migrations: String,
    pub changes: String,
    pub migration_why: String,
    pub authors: String,
    pub testers: String,
    pub comments: String,
    pub notes: String,
    pub packages: String,
    pub news_headline: String,
    pub workflow: String,
    pub workflows_samples: String,
    pub technical_considerations: String,
    pub next_features: String,
    pub next_motivation: String,
    pub next_reasons: String,
    pub next_resources: String,
}

enum PageIndex {
    Page0,
    Page1,
    Page2,
    Page3,
    Page4,
    Page5,
    Page6,
    Page7,
    Page8,
}

impl Display for PageIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Page0 => {
                write!(f, " /dev/null <== page 1/9 ==> Resolution ")
            }
            Page1 => {
                write!(f, " Problematics <== page 2/9 ==> Security ")
            }
            Page2 => {
                write!(f, " Resolution <== page 3/9 ==> Tests ")
            }
            Page3 => {
                write!(f, " Security <== page 4/9 ==> Requirements ")
            }
            Page4 => {
                write!(f, " Tests <== page 5/9 ==> Database ")
            }
            Page5 => {
                write!(f, " Requirements <== page 6/9 ==> Communication ")
            }
            Page6 => {
                write!(f, " Database <== page 7/9 ==> Ideas ")
            }
            Page7 => {
                write!(f, " Communication <== page 8/9 ==> Next ")
            }
            Page8 => {
                write!(f, " Ideas <== page 9/9 ==> /dev/null ")
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

fn get_lines(pages: &mut [Page], page_index: PageIndex, area: usize) -> String {
    let x = pages.get_mut(page_index as usize).unwrap();
    let y = x.areas.get_mut(area).unwrap();
    y.lines().join("\n")
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

fn update_commit(page: &mut [Page]) -> Commit {
    Commit {
        title: get_lines(page, Page0, 0),
        description: get_lines(page, Page0, 1),
        steps: get_lines(page, Page0, 2),
        expected_description: get_lines(page, Page0, 3),
        system_before: get_lines(page, Page1, 0),
        system_after: get_lines(page, Page1, 1),
        expectation: get_lines(page, Page1, 2),
        samples: get_lines(page, Page1, 3),
        vulnerabilities: get_lines(page, Page2, 0),
        qualities: get_lines(page, Page2, 1),
        conforms: get_lines(page, Page2, 2),
        risks: get_lines(page, Page2, 3),
        tests_added: get_lines(page, Page3, 0),
        tests_updated: get_lines(page, Page3, 1),
        tests_deleted: get_lines(page, Page3, 2),
        platforms: get_lines(page, Page3, 3),
        breaking_changes: get_lines(page, Page4, 0),
        dependencies: get_lines(page, Page4, 1),
        packages: get_lines(page, Page4, 2),
        news_headline: get_lines(page, Page7, 0),
        workflow: get_lines(page, Page7, 1),
        workflows_samples: get_lines(page, Page7, 2),
        technical_considerations: get_lines(page, Page7, 3),
        next_features: get_lines(page, Page8, 0),
        next_motivation: get_lines(page, Page8, 1),
        next_reasons: get_lines(page, Page8, 2),
        next_resources: get_lines(page, Page8, 3),
        rollbacks: get_lines(page, Page4, 3),
        up_migrations: get_lines(page, Page5, 0),
        down_migrations: get_lines(page, Page5, 1),
        changes: get_lines(page, Page5, 2),
        migration_why: get_lines(page, Page5, 3),
        authors: get_lines(page, Page6, 0),
        testers: get_lines(page, Page6, 1),
        comments: get_lines(page, Page6, 2),
        notes: get_lines(page, Page6, 3),
    }
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
            current_page: Page0,
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
            current_page: Page1,
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
            current_page: Page2,
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
            current_page: Page3,
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
            current_page: Page4,
            titles: [
                "Breaking changes",
                "New needed dependencies",
                "New packages needed",
                "Rollback",
            ],
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
            current_page: Page5,
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
            current_page: Page6,
            titles: ["Authors", "Testers", "Comments", "Notes"],
            describe: [
                "Authors name's",
                "Indicate the project testers",
                "Indicate comments and feedback",
                "Indicate important remarks and observations",
            ],
        },
        Page {
            main_title: "Ideas",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: Page7,
            titles: [
                "News headline",
                "Workflow samples",
                "Examples",
                "Technical considerations",
            ],
            describe: [
                "News headline and brief description",
                "Workflow steps and user interactions",
                "Code examples or visual mockups",
                "Technical considerations",
            ],
        },
        Page {
            main_title: "Next",
            areas: [
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
                TextArea::default(),
            ],
            current_page: Page8,
            titles: [
                "To Implement",
                "Motivation",
                "Implementation Plan",
                "Related links",
            ],
            describe: [
                "Description of the feature",
                "Reasons for implementing this feature",
                "Technical approach and steps involved",
                "Links to relevant resources",
            ],
        },
    ];
    let mut page: usize = 0;
    let mut witch: usize = 0;
    let mut commit_message: Commit = update_commit(&mut pages);
    loop {
        match pages[page].current_page {
            Page0 | Page1 | Page2 | Page3 | Page4 | Page5 | Page6 | Page7 | Page8 => {
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
            } else if key.code == KeyCode::F(6) {
                let message = COMMIT_TEMPLATE
                    .replace("%title%", &commit_message.title)
                    .replace("%description%", &commit_message.description)
                    .replace("%steps%", &commit_message.steps)
                    .replace(
                        "%expected_description%",
                        &commit_message.expected_description,
                    )
                    .replace("%system_before%", &commit_message.system_before)
                    .replace("%system_after%", &commit_message.system_after)
                    .replace("%system_before%", &commit_message.system_before)
                    .replace("%expectation%", &commit_message.expectation)
                    .replace("%samples%", &commit_message.samples)
                    .replace("%vulnerabilities%", &commit_message.vulnerabilities)
                    .replace("%qualities%", &commit_message.qualities)
                    .replace("%conformity%", &commit_message.conforms)
                    .replace("%risk%", &commit_message.risks)
                    .replace("%tests_added%", &commit_message.tests_added)
                    .replace("%tests_updated%", &commit_message.tests_updated)
                    .replace("%test_deleted%", &commit_message.tests_deleted)
                    .replace("%tested_platforms%", &commit_message.platforms)
                    .replace("%breaking_changes%", &commit_message.breaking_changes)
                    .replace("%dependencies%", &commit_message.dependencies)
                    .replace("%rollbacks%", &commit_message.rollbacks)
                    .replace("%db_up%", &commit_message.up_migrations)
                    .replace("%db_down%", &commit_message.down_migrations)
                    .replace("%db_changes%", &commit_message.changes)
                    .replace("%why_db_changes%", &commit_message.migration_why)
                    .replace("%authors%", &commit_message.authors)
                    .replace("%testers%", &commit_message.testers)
                    .replace("%comments%", &commit_message.comments)
                    .replace("%notes%", &commit_message.notes)
                    .replace("%news_headline%", &commit_message.news_headline)
                    .replace("%workflow%", &commit_message.workflow)
                    .replace("%workflows_samples%", &commit_message.workflows_samples)
                    .replace(
                        "%technical_considerations%",
                        &commit_message.technical_considerations,
                    )
                    .replace("%next_motivation%", &commit_message.next_motivation)
                    .replace("%next_description%", &commit_message.next_features)
                    .replace("%next_reasons%", &commit_message.next_reasons)
                    .replace("%next_links%", &commit_message.next_resources);

                assert!(Command::new("git")
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .arg("commit")
                    .arg("-m")
                    .arg(message)
                    .current_dir(".")
                    .spawn()
                    .expect("git not founded")
                    .wait()
                    .is_ok());
                commit_message = Commit::default();
                let _ = commit(rei, app);
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
                commit_message = update_commit(&mut pages);
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
