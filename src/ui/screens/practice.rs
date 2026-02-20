use crate::audio;
use crate::core::{actions, utils};
use crate::ui::app::{App, Screen};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
};

pub fn handle_event(app: &mut App, key: KeyEvent) {
    let session = match &mut app.session {
        Some(s) => s,
        None => return,
    };

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            app.session = None;
            app.current_screen = Screen::Menu;
        }
        KeyCode::Char('s') => {
            session.show_definition = true;
        }
        KeyCode::Char('y') => {
            if session.show_definition {
                session.graded = Some(true);
                audio::play_correct_sound();
            }
        }
        KeyCode::Char('n') => {
            if session.show_definition {
                session.graded = Some(false);
                audio::play_wrong_sound();
            }
        }
        KeyCode::Char('m') => {
            let word = session.current_mut();
            word.marked = !word.marked;
            audio::play_mark_sound();
        }
        KeyCode::Enter => {
            if session.show_definition && session.graded.is_some() {
                if let Err(e) = actions::handle_enter(app) {
                    app.error = Some(e.to_string());
                    app.current_screen = Screen::Menu;
                }
            }
        }
        _ => {}
    }
}

pub fn render(frame: &mut Frame, app: &App) {
    let session = match &app.session {
        Some(s) => s,
        None => return,
    };

    let word = session.current();
    let area = frame.size();

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(5), // Word
            Constraint::Length(5), // Definition
            Constraint::Length(4), // Stats
            Constraint::Length(5), // Actions
        ])
        .split(area);

    // ───────── HEADER ─────────
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(layout[0]);

    let left_header = Paragraph::new(format!(
        "{} WORD [{}/{}]",
        if word.marked { "*" } else { " " },
        session.index + 1,
        session.words.len()
    ))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1)),
    );

    let right_header = Paragraph::new(format!("Group {} | Id {}", word.group_id, word.id))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        );

    frame.render_widget(left_header, header_chunks[0]);
    frame.render_widget(right_header, header_chunks[1]);

    // ───────── WORD ─────────
    let word_style = match session.graded {
        Some(true) => Style::default().fg(Color::Green),
        Some(false) => Style::default().fg(Color::Red),
        None => Style::default(),
    };

    let word_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));

    let inner = word_block.inner(layout[1]);
    frame.render_widget(word_block, layout[1]);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(1),
            Constraint::Percentage(40),
        ])
        .split(inner);

    let word_para = Paragraph::new(word.word.clone())
        .style(word_style)
        .alignment(Alignment::Center)
        .bold();

    frame.render_widget(word_para, vertical[1]);

    // ───────── DEFINITION ─────────
    let def_text = if session.show_definition {
        word.definition.clone()
    } else {
        "(hidden)".into()
    };

    let definition = Paragraph::new(def_text).alignment(Alignment::Center).block(
        Block::default()
            .title("Definition")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1)),
    );

    frame.render_widget(definition, layout[2]);

    // ───────── STATS ─────────
    let stats = Paragraph::new(format!(
        "Last Seen: {}\nAccuracy: {}/{}",
        utils::relative_time(word.last_seen),
        word.success_count,
        word.times_seen
    ))
    .block(
        Block::default()
            .title("Stats")
            .borders(Borders::ALL)
            .padding(Padding::horizontal(1)),
    );

    frame.render_widget(stats, layout[3]);

    // ───────── ACTION BUTTONS ─────────
    let actions_block = Block::default()
        .title("Actions")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));

    let inner_actions = actions_block.inner(layout[4]);
    frame.render_widget(actions_block, layout[4]);

    let buttons = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(inner_actions);

    render_button(frame, buttons[0], "Show", "s");
    render_button(frame, buttons[1], "Correct", "y");
    render_button(frame, buttons[2], "Wrong", "n");
    render_button(frame, buttons[3], "Mark", "m");
    render_button(frame, buttons[4], "Next", "⏎");
}

fn render_button(frame: &mut Frame, area: Rect, label: &str, key: &str) {
    let content = Line::from(vec![
        Span::styled(label, Style::default().bold()),
        Span::raw("\n"),
        Span::styled(format!("[{}]", key), Style::default().fg(Color::Yellow)),
    ]);

    let button = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(button, area);
}
