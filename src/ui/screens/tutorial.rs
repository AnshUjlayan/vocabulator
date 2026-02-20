// Tutorial screen module
// Handles rendering and event processing for the tutorial screen

use ratatui::Frame;
use ratatui::prelude::Rect;
use ratatui::style::Stylize;
use ratatui::widgets::Padding;
use crate::ui::app::App;

/// Render the tutorial screen
///
/// Displays the tutorial interface with:
/// - Current step instruction text in a distinct area
/// - Progress indicator (Step X of Y)
/// - Highlight for relevant UI elements based on HighlightTarget
/// - Hint messages when validation fails
/// - Sample practice content when in practice steps (steps 4-11)
/// - Borders to separate tutorial content from application content
/// - Consistent styling with existing UI
///
/// **Validates: Requirements 3.2, 3.3, 11.1, 11.2, 11.3, 11.4, 11.5**
pub fn render(frame: &mut Frame, app: &App) {
    use crate::core::tutorial::get_current_step;

    // Check if tutorial state exists
    let tutorial_state = match &app.tutorial_state {
        Some(state) => state,
        None => return,
    };

    let area = frame.size();
    let current_step = get_current_step(tutorial_state);

    // Check if we're in exit confirmation mode
    if tutorial_state.exit_requested {
        // Check if this is a congratulations dialog
        let is_congrats = tutorial_state.completed_actions.contains(&"SHOW_CONGRATS".to_string());
        if is_congrats {
            render_congratulations(frame, area);
        } else {
            render_exit_confirmation(frame, area);
        }
        return;
    }

    // Determine if we're in a practice step (steps 5-8, 10-12)
    let is_practice_step = (current_step.id >= 5 && current_step.id <= 8) || (current_step.id >= 10 && current_step.id <= 12);

    if is_practice_step {
        // Render practice screen with tutorial overlay
        render_practice_with_tutorial(frame, app, tutorial_state, current_step);
    } else {
        // Render tutorial-only screen (steps 0-3, 11)
        render_tutorial_only(frame, app, tutorial_state, current_step, area);
    }
}

/// Render exit confirmation dialog
fn render_exit_confirmation(frame: &mut Frame, area: Rect) {
    use ratatui::{
        layout::Alignment,
        style::{Color, Style},
        widgets::{Block, Borders, Clear, Paragraph},
    };

    // Create a centered popup
    let popup_area = centered_rect(60, 30, area);

    // Clear the area
    frame.render_widget(Clear, popup_area);

    // Create the confirmation dialog
    let block = Block::default()
        .title("Exit Tutorial?")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    let text = vec![
        "",
        "Are you sure you want to exit the tutorial?",
        "",
        "Your progress will not be saved, but you can",
        "restart the tutorial anytime from the main menu.",
        "",
        "",
        "Press 'y' to exit and start learning",
        "Press 'n' or Escape to continue tutorial",
    ];

    let paragraph = Paragraph::new(text.join("\n"))
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

    frame.render_widget(paragraph, popup_area);
}

/// Render congratulations dialog
fn render_congratulations(frame: &mut Frame, area: Rect) {
    use ratatui::{
        layout::Alignment,
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, Clear, Paragraph},
    };

    // Create a centered popup
    let popup_area = centered_rect(70, 40, area);

    // Clear the area
    frame.render_widget(Clear, popup_area);

    // Create the congratulations dialog
    let block = Block::default()
        .title("🎉 Congratulations! 🎉")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black).fg(Color::Green));

    let text = vec![
        "",
        "You've completed the tutorial!",
        "",
        "You now know how to:",
        "• Navigate menus with arrow keys or j/k",
        "• Practice vocabulary words",
        "• Show definitions with 's'",
        "• Grade yourself with 'y' or 'n'",
        "• Bookmark words with 'm'",
        "• Move to the next word with Enter",
        "• Exit practice with 'q' or Escape",
        "",
        "There's also a Test mode where you type the word!",
        "Your progress auto-saves, so practice anytime!",
        "",
        "",
        "Press any key to start practicing!",
    ];

    let paragraph = Paragraph::new(text.join("\n"))
        .block(block)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));

    frame.render_widget(paragraph, popup_area);
}

/// Render tutorial-only screen (for non-practice steps)
fn render_tutorial_only(
    frame: &mut Frame,
    app: &App,
    tutorial_state: &crate::core::tutorial::TutorialState,
    current_step: &crate::core::tutorial::TutorialStep,
    area: Rect,
) {
    use ratatui::{
        layout::{Alignment, Constraint, Direction, Layout},
        style::{Color, Style},
        widgets::{Block, Borders, Padding, Paragraph},
    };

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),  // Progress indicator
            Constraint::Length(8),  // Instruction
            Constraint::Min(10),    // Content area (menu or message)
            Constraint::Length(3),  // Hint/Error message
        ])
        .split(area);

    // ───────── PROGRESS INDICATOR ─────────
    let progress_text = format!(
        "Tutorial - Step {} of {}",
        tutorial_state.current_step + 1,
        tutorial_state.total_steps
    );
    let progress = Paragraph::new(progress_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        );
    frame.render_widget(progress, layout[0]);

    // ───────── INSTRUCTION ─────────
    let instruction = Paragraph::new(current_step.instruction)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title("Instructions")
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1))
                .style(Style::default().fg(Color::Yellow)),
        );
    frame.render_widget(instruction, layout[1]);

    // ───────── CONTENT AREA ─────────
    // For steps 1-3, show the menu
    if current_step.id >= 1 && current_step.id <= 3 {
        render_menu_preview(frame, app, layout[2], current_step.highlight.as_ref());
    } else if current_step.id == 4 {
        // For step 4, show countdown timer
        let elapsed = tutorial_state.step_entered_at
            .map(|t| t.elapsed().as_secs())
            .unwrap_or(0);
        let remaining = 10_u64.saturating_sub(elapsed);
        
        let message = format!(
            "Get ready to practice!\n\nYou'll see a vocabulary word.\nTry to recall its definition before revealing it.\n\nAuto-advancing in {} second{}...\n\n(or press any key to continue)",
            remaining,
            if remaining == 1 { "" } else { "s" }
        );

        let content = Paragraph::new(message)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(1)),
            );
        frame.render_widget(content, layout[2]);
    } else {
        // For steps 0 and 13, show a welcome/completion message
        let message = if current_step.id == 0 {
            "Welcome to Vocabulator!\n\nThis interactive tutorial will guide you through\nall the features of the application.\n\nYou'll learn by doing - the tutorial will\nwait for you to perform each action correctly."
        } else {
            "Congratulations!\n\nYou've completed the tutorial and learned:\n\n• How to navigate menus\n• How to practice vocabulary words\n• How to grade yourself\n• How to bookmark words\n• How to exit and return to the menu\n\nThere's also a Test mode where you type the word!\nYour progress auto-saves, so feel free to\nquit anytime with 'q' or Escape."
        };

        let content = Paragraph::new(message)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(1)),
            );
        frame.render_widget(content, layout[2]);
    }

    // ───────── HINT/ERROR MESSAGE ─────────
    if let Some(error) = &app.error {
        let hint = Paragraph::new(error.as_str())
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::horizontal(1)),
            );
        frame.render_widget(hint, layout[3]);
    }
}

/// Render practice screen with tutorial overlay
fn render_practice_with_tutorial(
    frame: &mut Frame,
    app: &App,
    tutorial_state: &crate::core::tutorial::TutorialState,
    current_step: &crate::core::tutorial::TutorialStep,
) {
    use ratatui::{
        layout::{Alignment, Constraint, Direction, Layout},
        style::{Color, Style},
        widgets::{Block, Borders, Padding, Paragraph},
    };
    use crate::core::utils;

    // Get the session from tutorial state
    let session = match &tutorial_state.sample_session {
        Some(s) => s,
        None => return,
    };

    let word = session.current();
    let area = frame.size();

    // Main layout with tutorial overlay at top
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(6),  // Tutorial overlay
            Constraint::Length(3),  // Header
            Constraint::Length(5),  // Word
            Constraint::Length(5),  // Definition
            Constraint::Length(4),  // Stats
            Constraint::Length(5),  // Actions
        ])
        .split(area);

    // ───────── TUTORIAL OVERLAY ─────────
    let tutorial_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Progress
            Constraint::Length(3),  // Instruction
        ])
        .split(main_layout[0]);

    // Progress indicator
    let progress_text = format!(
        "Tutorial - Step {} of {}",
        tutorial_state.current_step + 1,
        tutorial_state.total_steps
    );
    let progress = Paragraph::new(progress_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        );
    frame.render_widget(progress, tutorial_layout[0]);

    // Instruction
    let instruction_text = if let Some(error) = &app.error {
        format!("❌ {}", error)
    } else {
        format!("📖 {}", current_step.instruction)
    };

    let instruction_style = if app.error.is_some() {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let instruction = Paragraph::new(instruction_text)
        .alignment(Alignment::Center)
        .style(instruction_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .padding(Padding::horizontal(1)),
        );
    frame.render_widget(instruction, tutorial_layout[1]);

    // ───────── PRACTICE SCREEN CONTENT (same as practice.rs) ─────────

    // Header
    let header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_layout[1]);

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

    // Word
    let word_style = match session.graded {
        Some(true) => Style::default().fg(Color::Green),
        Some(false) => Style::default().fg(Color::Red),
        None => Style::default(),
    };

    let word_block = Block::default()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));

    let inner = word_block.inner(main_layout[2]);
    frame.render_widget(word_block, main_layout[2]);

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

    // Definition
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

    frame.render_widget(definition, main_layout[3]);

    // Stats
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

    frame.render_widget(stats, main_layout[4]);

    // Actions with highlighting
    render_actions_with_highlight(
        frame,
        main_layout[5],
        current_step.highlight.as_ref(),
    );
}

/// Render action buttons with optional highlighting
fn render_actions_with_highlight(
    frame: &mut Frame,
    area: Rect,
    highlight: Option<&crate::core::tutorial::HighlightTarget>,
) {
    use ratatui::{
        layout::{Constraint, Direction, Layout},
        widgets::{Block, Borders},
    };
    use crate::core::tutorial::HighlightTarget;

    let actions_block = Block::default()
        .title("Actions")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1));

    let inner_actions = actions_block.inner(area);
    frame.render_widget(actions_block, area);

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

    // Determine which button to highlight
    let highlight_key = if let Some(HighlightTarget::KeyHint(key)) = highlight {
        Some(*key)
    } else {
        None
    };

    render_button_with_highlight(frame, buttons[0], "Show", "s", highlight_key);
    render_button_with_highlight(frame, buttons[1], "Correct", "y", highlight_key);
    render_button_with_highlight(frame, buttons[2], "Wrong", "n", highlight_key);
    render_button_with_highlight(frame, buttons[3], "Mark", "m", highlight_key);
    render_button_with_highlight(frame, buttons[4], "Next", "⏎", highlight_key);
}

/// Render a single button with optional highlighting
fn render_button_with_highlight(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    key: &str,
    highlight_key: Option<&str>,
) {
    use ratatui::{
        layout::Alignment,
        style::{Color, Style},
        text::{Line, Span},
        widgets::{Block, Borders, Paragraph},
    };

    // Check if this button should be highlighted
    let should_highlight = highlight_key.map_or(false, |h| {
        // Exact match or special case for Enter key
        h == key || (key == "⏎" && (h == "Enter" || h.to_lowercase() == "enter"))
    });

    let key_style = if should_highlight {
        Style::default().fg(Color::Green).bold()
    } else {
        Style::default().fg(Color::Yellow)
    };

    let border_style = if should_highlight {
        Style::default().fg(Color::Green)
    } else {
        Style::default()
    };

    let content = Line::from(vec![
        Span::styled(label, Style::default().bold()),
        Span::raw("\n"),
        Span::styled(format!("[{}]", key), key_style),
    ]);

    let button = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).style(border_style));

    frame.render_widget(button, area);
}

/// Render menu preview for tutorial steps 1-3
fn render_menu_preview(
    frame: &mut Frame,
    app: &App,
    area: Rect,
    highlight: Option<&crate::core::tutorial::HighlightTarget>,
) {
    use ratatui::{
        style::{Color, Style},
        text::Line,
        widgets::{Block, Borders, List, ListItem, Padding},
    };
    use crate::core::tutorial::HighlightTarget;

    // Determine which menu item to highlight
    let highlight_index = if let Some(HighlightTarget::MenuOption(idx)) = highlight {
        Some(*idx)
    } else {
        None
    };

    let menu_block = Block::default()
        .title("Main Menu")
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .style(Style::default().fg(Color::White));

    let inner = menu_block.inner(area);
    frame.render_widget(menu_block, area);

    // Create menu items
    let items: Vec<ListItem> = app
        .menu_items
        .iter()
        .enumerate()
        .map(|(i, action)| {
            let is_selected = i == app.selected;
            let is_highlighted = highlight_index == Some(i);

            let prefix = if is_selected { "> " } else { "  " };
            let text = format!("{}{}", prefix, action.label());

            let style = if is_highlighted {
                Style::default().fg(Color::Green).bold()
            } else if is_selected {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(text)).style(style)
        })
        .collect();

    let list = List::new(items);
    frame.render_widget(list, inner);
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    use ratatui::layout::{Constraint, Direction, Layout};

    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

use crossterm::event::{KeyCode, KeyEvent};
use crate::core::tutorial::{validate_and_advance, ValidationResult, mark_tutorial_completed};
use crate::ui::app::Screen;

/// Handle keyboard events for the tutorial screen
///
/// This is the main event handler for the tutorial screen. It integrates with the
/// tutorial engine's validate_and_advance function and handles the tutorial flow.
///
/// Responsibilities:
/// - Check for exit request (q/Escape) and show confirmation prompt
/// - Pass key events to tutorial engine for validation
/// - Update tutorial state based on validation result
/// - Display hint messages for invalid actions
/// - Transition to Menu screen on completion
/// - Handle exit confirmation (confirm/cancel)
///
/// **Validates: Requirements 3.4, 3.5, 4.1, 4.2, 10.1, 10.2, 10.3**
pub fn handle_event(app: &mut App, key: KeyEvent) {
    // Check if tutorial state exists
    if app.tutorial_state.is_none() {
        // No tutorial state, return to menu
        app.current_screen = Screen::Menu;
        return;
    }

    // Check if exit confirmation is pending
    let exit_requested = app.tutorial_state.as_ref().unwrap().exit_requested;
    let is_congrats = app.tutorial_state.as_ref().unwrap().completed_actions.contains(&"SHOW_CONGRATS".to_string());
    
    if exit_requested {
        if is_congrats {
            // This is the congratulations dialog - any key returns to menu
            app.tutorial_state = None;
            app.current_screen = Screen::Menu;
            return;
        }
        
        // Handle exit confirmation
        match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => {
                // Confirm exit - return to menu without marking tutorial as completed
                app.tutorial_state = None;
                app.current_screen = Screen::Menu;
                return;
            }
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                // Cancel exit - resume tutorial
                app.tutorial_state.as_mut().unwrap().exit_requested = false;
                return;
            }
            _ => {
                // Ignore other keys during confirmation
                return;
            }
        }
    }

    // Check for exit request (q or Escape)
    if matches!(key.code, KeyCode::Char('q') | KeyCode::Esc) {
        // Show confirmation prompt
        app.tutorial_state.as_mut().unwrap().exit_requested = true;
        return;
    }

    // Special handling for step 4 (auto-advance on any key)
    let current_step = app.tutorial_state.as_ref().unwrap().current_step;
    if current_step == 4 {
        // Any key press advances from step 4 to step 5
        app.tutorial_state.as_mut().unwrap().current_step = 5;
        app.tutorial_state.as_mut().unwrap().step_entered_at = Some(std::time::Instant::now());
        app.error = None;
        return;
    }

    // Handle menu navigation keys during tutorial steps 1-3
    if current_step >= 1 && current_step <= 3 {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                // Move menu selection down
                if app.selected < app.menu_items.len() - 1 {
                    app.selected += 1;
                } else {
                    app.selected = 0; // Wrap around
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                // Move menu selection up
                if app.selected > 0 {
                    app.selected -= 1;
                } else {
                    app.selected = app.menu_items.len() - 1; // Wrap around
                }
            }
            _ => {}
        }
    }

    // Handle practice-related keys during tutorial (steps 5-8, 10-12)
    if (current_step >= 5 && current_step <= 8) || (current_step >= 10 && current_step <= 12) {
        // Handle keys that modify the sample session
        match key.code {
            KeyCode::Char('m') => {
                // Toggle bookmark on current word in sample session
                if let Some(ref mut tutorial_state) = app.tutorial_state {
                    if let Some(ref mut session) = tutorial_state.sample_session {
                        if session.index < session.words.len() {
                            session.words[session.index].marked = !session.words[session.index].marked;
                            crate::audio::play_mark_sound();
                        }
                    }
                }
            }
            KeyCode::Char('s') => {
                // Show definition in sample session
                if let Some(ref mut tutorial_state) = app.tutorial_state {
                    if let Some(ref mut session) = tutorial_state.sample_session {
                        session.show_definition = true;
                    }
                }
            }
            KeyCode::Char('y') => {
                // Grade the word as correct in sample session
                if let Some(ref mut tutorial_state) = app.tutorial_state {
                    if let Some(ref mut session) = tutorial_state.sample_session {
                        session.graded = Some(true);
                        crate::audio::play_correct_sound();
                    }
                }
            }
            KeyCode::Char('n') => {
                // Grade the word as incorrect in sample session
                if let Some(ref mut tutorial_state) = app.tutorial_state {
                    if let Some(ref mut session) = tutorial_state.sample_session {
                        session.graded = Some(false);
                        crate::audio::play_wrong_sound();
                    }
                }
            }
            KeyCode::Enter => {
                // Advance to next word in sample session
                if let Some(ref mut tutorial_state) = app.tutorial_state {
                    if let Some(ref mut session) = tutorial_state.sample_session {
                        if session.index < session.words.len() - 1 {
                            session.index += 1;
                            // Reset state for new word
                            session.show_definition = false;
                            session.graded = None;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Pass key event to tutorial engine for validation
    // We need to extract tutorial_state temporarily to avoid borrow conflicts
    let mut tutorial_state = app.tutorial_state.take().unwrap();
    let result = validate_and_advance(&mut tutorial_state, app, key);
    
    match result {
        ValidationResult::Valid => {
            // Step completed successfully, state already advanced
            // Clear any previous error messages
            app.error = None;
            // Restore tutorial state
            app.tutorial_state = Some(tutorial_state);
        }
        ValidationResult::Invalid(hint) => {
            // Invalid action, display hint message
            app.error = Some(hint);
            // Restore tutorial state
            app.tutorial_state = Some(tutorial_state);
        }
        ValidationResult::Complete => {
            // Tutorial completed! Show congratulations and mark as completed
            if let Err(e) = mark_tutorial_completed(&app.conn) {
                // Log error but don't block completion
                eprintln!("Failed to mark tutorial as completed: {}", e);
            }
            
            // Set a flag to show congratulations dialog
            // We'll use the exit_requested field temporarily to show the congrats dialog
            app.tutorial_state = Some(tutorial_state);
            app.tutorial_state.as_mut().unwrap().exit_requested = true;
            // Store a special marker in completed_actions to indicate this is a completion dialog
            app.tutorial_state.as_mut().unwrap().completed_actions.push("SHOW_CONGRATS".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::app::App;
    use crate::core::tutorial::init_tutorial;
    use crossterm::event::{KeyEvent, KeyModifiers};

    #[test]
    fn test_handle_event_no_tutorial_state_returns_to_menu() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        app.tutorial_state = None;

        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.current_screen, Screen::Menu);
    }

    #[test]
    fn test_handle_event_q_key_requests_exit() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        app.tutorial_state = Some(init_tutorial());

        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        handle_event(&mut app, key);

        assert!(app.tutorial_state.as_ref().unwrap().exit_requested);
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_handle_event_escape_key_requests_exit() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        app.tutorial_state = Some(init_tutorial());

        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert!(app.tutorial_state.as_ref().unwrap().exit_requested);
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_handle_event_confirm_exit_with_y() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        state.exit_requested = true;
        app.tutorial_state = Some(state);

        let key = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::empty());
        handle_event(&mut app, key);

        assert!(app.tutorial_state.is_none());
        assert_eq!(app.current_screen, Screen::Menu);
    }

    #[test]
    fn test_handle_event_confirm_exit_with_uppercase_y() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        state.exit_requested = true;
        app.tutorial_state = Some(state);

        let key = KeyEvent::new(KeyCode::Char('Y'), KeyModifiers::empty());
        handle_event(&mut app, key);

        assert!(app.tutorial_state.is_none());
        assert_eq!(app.current_screen, Screen::Menu);
    }

    #[test]
    fn test_handle_event_cancel_exit_with_n() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        state.exit_requested = true;
        app.tutorial_state = Some(state);

        let key = KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty());
        handle_event(&mut app, key);

        assert!(app.tutorial_state.is_some());
        assert!(!app.tutorial_state.as_ref().unwrap().exit_requested);
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_handle_event_cancel_exit_with_uppercase_n() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        state.exit_requested = true;
        app.tutorial_state = Some(state);

        let key = KeyEvent::new(KeyCode::Char('N'), KeyModifiers::empty());
        handle_event(&mut app, key);

        assert!(app.tutorial_state.is_some());
        assert!(!app.tutorial_state.as_ref().unwrap().exit_requested);
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_handle_event_cancel_exit_with_escape() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        state.exit_requested = true;
        app.tutorial_state = Some(state);

        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert!(app.tutorial_state.is_some());
        assert!(!app.tutorial_state.as_ref().unwrap().exit_requested);
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_handle_event_ignore_other_keys_during_exit_confirmation() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        state.exit_requested = true;
        let initial_step = state.current_step;
        app.tutorial_state = Some(state);

        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        handle_event(&mut app, key);

        // Should remain in exit confirmation state
        assert!(app.tutorial_state.is_some());
        assert!(app.tutorial_state.as_ref().unwrap().exit_requested);
        assert_eq!(app.tutorial_state.as_ref().unwrap().current_step, initial_step);
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_handle_event_valid_action_advances_step() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        app.tutorial_state = Some(init_tutorial());

        // Step 0 expects Enter key
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.tutorial_state.as_ref().unwrap().current_step, 1);
        assert!(app.error.is_none());
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_handle_event_invalid_action_shows_hint() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        app.tutorial_state = Some(init_tutorial());

        // Step 0 expects Enter, press 'x' instead
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        handle_event(&mut app, key);

        assert_eq!(app.tutorial_state.as_ref().unwrap().current_step, 0);
        assert!(app.error.is_some());
        assert_eq!(app.current_screen, Screen::Tutorial);
    }

    #[test]
    fn test_handle_event_completion_marks_tutorial_and_returns_to_menu() {
        use crate::core::tutorial::is_tutorial_completed;
        use crate::db::schema::INIT_SCHEMA;

        let mut app = App::new_test();
        // Initialize database schema
        app.conn.execute_batch(INIT_SCHEMA).unwrap();
        
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        // Set to last step
        state.current_step = state.total_steps - 1;
        app.tutorial_state = Some(state);

        // Complete the last step (expects Enter)
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        // Should show congratulations dialog (tutorial state still exists with exit_requested=true)
        assert_eq!(app.current_screen, Screen::Tutorial);
        assert!(app.tutorial_state.is_some());
        assert!(app.tutorial_state.as_ref().unwrap().exit_requested);
        assert!(app.tutorial_state.as_ref().unwrap().completed_actions.contains(&"SHOW_CONGRATS".to_string()));

        // Press any key to dismiss congratulations and go to menu
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        // Now should return to menu
        assert_eq!(app.current_screen, Screen::Menu);
        assert!(app.tutorial_state.is_none());

        // Should mark tutorial as completed
        assert!(is_tutorial_completed(&app.conn).unwrap());
    }

    #[test]
    fn test_handle_event_exit_without_completion_does_not_mark_completed() {
        use crate::core::tutorial::is_tutorial_completed;
        use crate::db::schema::INIT_SCHEMA;

        let mut app = App::new_test();
        // Initialize database schema
        app.conn.execute_batch(INIT_SCHEMA).unwrap();
        
        app.current_screen = Screen::Tutorial;
        app.tutorial_state = Some(init_tutorial());

        // Request exit
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        handle_event(&mut app, key);

        // Confirm exit
        let key = KeyEvent::new(KeyCode::Char('y'), KeyModifiers::empty());
        handle_event(&mut app, key);

        // Should return to menu
        assert_eq!(app.current_screen, Screen::Menu);
        assert!(app.tutorial_state.is_none());

        // Should NOT mark tutorial as completed
        assert!(!is_tutorial_completed(&app.conn).unwrap());
    }

    #[test]
    fn test_handle_event_m_key_toggles_bookmark_during_practice_steps() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        // Set to step 7 (bookmark step)
        state.current_step = 7;
        app.tutorial_state = Some(state);

        // Verify word is not marked initially
        assert!(!app.tutorial_state.as_ref().unwrap().sample_session.as_ref().unwrap().words[0].marked);

        // Press 'm' to toggle bookmark
        let key = KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty());
        handle_event(&mut app, key);

        // Verify word is now marked
        assert!(app.tutorial_state.as_ref().unwrap().sample_session.as_ref().unwrap().words[0].marked);

        // Press 'm' again to toggle bookmark off
        let key = KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty());
        handle_event(&mut app, key);

        // Verify word is no longer marked
        assert!(!app.tutorial_state.as_ref().unwrap().sample_session.as_ref().unwrap().words[0].marked);
    }

    #[test]
    fn test_handle_event_enter_advances_word_index_during_practice_steps() {
        let mut app = App::new_test();
        app.current_screen = Screen::Tutorial;
        let mut state = init_tutorial();
        // Set to step 10 (next word step)
        state.current_step = 10;
        app.tutorial_state = Some(state);

        // Verify we're at word index 0
        assert_eq!(app.tutorial_state.as_ref().unwrap().sample_session.as_ref().unwrap().index, 0);

        // Press Enter to advance to next word
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        handle_event(&mut app, key);

        // Verify we're now at word index 1
        assert_eq!(app.tutorial_state.as_ref().unwrap().sample_session.as_ref().unwrap().index, 1);
    }
}
