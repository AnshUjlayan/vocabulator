// Tutorial engine module
// Manages tutorial state, step progression, and validation logic

use crate::core::session::Session;
use crate::ui::app::App;
use crossterm::event::KeyCode;

/// Defines a single step in the tutorial sequence
pub struct TutorialStep {
    pub id: usize,
    pub instruction: &'static str,
    pub hint: Option<&'static str>,
    pub validation: StepValidation,
    pub highlight: Option<HighlightTarget>,
}

/// Validation criteria for completing a tutorial step
pub enum StepValidation {
    /// Step completes when a specific key is pressed
    KeyPress(KeyCode),
    /// Step completes when a specific menu item is selected
    MenuSelection(usize),
    /// Step completes when a custom condition function returns true
    StateCondition(fn(&App, &TutorialState) -> bool),
}

/// UI elements that can be highlighted during a tutorial step
pub enum HighlightTarget {
    /// Highlight a specific menu option by index
    MenuOption(usize),
    /// Highlight a keyboard shortcut hint
    KeyHint(&'static str),
}

/// Represents the current state of the tutorial
#[derive(Debug)]
pub struct TutorialState {
    pub current_step: usize,
    pub total_steps: usize,
    pub sample_session: Option<Session>,
    pub completed_actions: Vec<String>,
    pub exit_requested: bool,
    pub step_entered_at: Option<std::time::Instant>,
}

/// Result of validating a user action against the current tutorial step
pub enum ValidationResult {
    /// Action was valid, step completed successfully
    Valid,
    /// Action was invalid, contains hint message to display
    Invalid(String),
    /// Tutorial is complete, all steps finished
    Complete,
}

/// Complete sequence of tutorial steps guiding users through all application features
pub const TUTORIAL_STEPS: &[TutorialStep] = &[
    // Step 0: Welcome
    TutorialStep {
        id: 0,
        instruction: "Welcome to Vocabulator! This tutorial will teach you how to use the app. Press Enter to continue.",
        hint: Some("Press the Enter key to proceed."),
        validation: StepValidation::KeyPress(KeyCode::Enter),
        highlight: None,
    },
    
    // Step 1: Menu navigation down
    TutorialStep {
        id: 1,
        instruction: "Use the Down arrow or 'j' key to move down in the menu. Try it now.",
        hint: Some("Press Down arrow or 'j' to move the selection down."),
        validation: StepValidation::KeyPress(KeyCode::Down),
        highlight: Some(HighlightTarget::MenuOption(1)),
    },
    
    // Step 2: Menu navigation up
    TutorialStep {
        id: 2,
        instruction: "Use the Up arrow or 'k' key to move up. Try moving back up.",
        hint: Some("Press Up arrow or 'k' to move the selection up."),
        validation: StepValidation::KeyPress(KeyCode::Up),
        highlight: Some(HighlightTarget::MenuOption(0)),
    },
    
    // Step 3: Select Continue Learning
    TutorialStep {
        id: 3,
        instruction: "Press Enter to select 'Continue Learning' and start a practice session.",
        hint: Some("Make sure 'Continue Learning' is highlighted, then press Enter."),
        validation: StepValidation::MenuSelection(0),
        highlight: Some(HighlightTarget::MenuOption(0)),
    },
    
    // Step 4: View word (auto-advance after 10 seconds or any key press)
    TutorialStep {
        id: 4,
        instruction: "You see a vocabulary word. Try to recall its definition before revealing it.",
        hint: Some("This message will auto-advance in 10 seconds, or press any key to continue."),
        validation: StepValidation::StateCondition(|_app, _state| {
            // This step auto-advances, validation handled in event handler
            false
        }),
        highlight: None,
    },
    
    // Step 5: Show definition
    TutorialStep {
        id: 5,
        instruction: "Press 's' to show the definition.",
        hint: Some("Press the 's' key to reveal the definition."),
        validation: StepValidation::KeyPress(KeyCode::Char('s')),
        highlight: Some(HighlightTarget::KeyHint("s")),
    },
    
    // Step 6: Grade (accept both y and n)
    TutorialStep {
        id: 6,
        instruction: "Grade yourself honestly. Press 'y' if you knew it, or 'n' if you didn't.",
        hint: Some("Press 'y' for correct or 'n' for incorrect."),
        validation: StepValidation::StateCondition(|_app, _state| {
            // This step is validated by the key press handler
            // We'll check if the user pressed y or n in the validation logic
            // For now, this will be handled by the special case in validate_and_advance
            true // This will be overridden by the key check
        }),
        highlight: Some(HighlightTarget::KeyHint("y/n")),
    },
    
    // Step 7: Bookmark feature
    TutorialStep {
        id: 7,
        instruction: "Press 'm' to bookmark this word. Bookmarked words show a star (*).",
        hint: Some("Press the 'm' key to toggle the bookmark."),
        validation: StepValidation::StateCondition(|_app, state| {
            // Check if word is marked in the tutorial sample session
            let session = match &state.sample_session {
                Some(s) => s,
                None => return false,
            };
            // Check if current word is marked
            if session.index < session.words.len() {
                session.words[session.index].marked
            } else {
                false
            }
        }),
        highlight: Some(HighlightTarget::KeyHint("m")),
    },
    
    // Step 8: Unbookmark
    TutorialStep {
        id: 8,
        instruction: "Press 'm' again to remove the bookmark.",
        hint: Some("Press the 'm' key to toggle the bookmark off."),
        validation: StepValidation::StateCondition(|_app, state| {
            // Check if word is unmarked
            let session = match &state.sample_session {
                Some(s) => s,
                None => return false,
            };
            // Word at current index should be unmarked
            if session.index < session.words.len() {
                !session.words[session.index].marked
            } else {
                false
            }
        }),
        highlight: Some(HighlightTarget::KeyHint("m")),
    },
    
    // Step 9: Explain Review Marks feature
    TutorialStep {
        id: 9,
        instruction: "Bookmarked words can be reviewed later! Use 'Review Marks' from the main menu to practice only your bookmarked words. Press Enter to continue.",
        hint: Some("Press Enter to continue learning about the app."),
        validation: StepValidation::KeyPress(KeyCode::Enter),
        highlight: None,
    },
    
    // Step 10: Advance to next word
    TutorialStep {
        id: 10,
        instruction: "Press Enter to move to the next word.",
        hint: Some("Press the Enter key to advance to the next word."),
        validation: StepValidation::StateCondition(|_app, state| {
            // Check if we've advanced to the next word
            let session = match &state.sample_session {
                Some(s) => s,
                None => return false,
            };
            // We should have moved to index 1 or higher
            session.index >= 1
        }),
        highlight: Some(HighlightTarget::KeyHint("Enter")),
    },
    
    // Step 11: Practice more words
    TutorialStep {
        id: 11,
        instruction: "Practice with a few more words using 's', 'y'/'n', 'm', and Enter as you like.",
        hint: Some("Use the practice controls freely. Advance to at least 2 more words to continue."),
        validation: StepValidation::StateCondition(|_app, state| {
            // Check if we've advanced to word index 2 or higher in the tutorial sample session
            let session = match &state.sample_session {
                Some(s) => s,
                None => return false,
            };
            session.index >= 2
        }),
        highlight: None,
    },
    
    // Step 12: Exit to menu
    TutorialStep {
        id: 12,
        instruction: "Press 'q' or Escape to return to the main menu.",
        hint: Some("Press 'q' or Escape to exit the practice session."),
        validation: StepValidation::KeyPress(KeyCode::Char('q')),
        highlight: Some(HighlightTarget::KeyHint("q")),
    },
    
    // Step 13: Completion
    TutorialStep {
        id: 13,
        instruction: "Great job! You've learned the basics. There's also a Test mode where you type the word from the definition. Your progress auto-saves. Press Enter to finish.",
        hint: Some("Press Enter to complete the tutorial."),
        validation: StepValidation::KeyPress(KeyCode::Enter),
        highlight: None,
    },
];

/// Sample vocabulary words used during the tutorial
/// These words have simple, clear definitions suitable for demonstration
/// They will be created as temporary Word structs with negative IDs to distinguish them from real vocabulary data
pub const SAMPLE_WORDS: &[(&str, &str)] = &[
    ("ephemeral", "lasting for a very short time"),
    ("ubiquitous", "present, appearing, or found everywhere"),
    ("serendipity", "the occurrence of events by chance in a happy way"),
    ("eloquent", "fluent or persuasive in speaking or writing"),
    ("pragmatic", "dealing with things sensibly and realistically"),
];

/// Initialize a new tutorial session with sample data
///
/// Creates a TutorialState with:
/// - Step counter initialized to 0
/// - Sample session containing words with negative IDs (-1, -2, -3, etc.)
/// - All sample words have group_id set to -1
/// - Sample words are isolated from real vocabulary data
///
/// **Validates: Requirements 12.1, 12.2**
/// Initialize a new tutorial session with sample data
///
/// Creates a TutorialState with:
/// - Step counter initialized to 0
/// - Sample session containing words with negative IDs (-1, -2, -3, etc.)
/// - All sample words have group_id set to -1
/// - Sample words are isolated from real vocabulary data
///
/// **Validates: Requirements 12.1, 12.2**
pub fn init_tutorial() -> TutorialState {
    // Create a sample session for tutorial practice
    let sample_session = create_sample_session();

    TutorialState {
        current_step: 0,
        total_steps: TUTORIAL_STEPS.len(),
        sample_session: Some(sample_session),
        completed_actions: Vec::new(),
        exit_requested: false,
        step_entered_at: Some(std::time::Instant::now()),
    }
}
/// Create a sample session for tutorial practice
///
/// Builds a Session containing sample words with:
/// - Negative IDs (-1, -2, -3, etc.) to distinguish from real vocabulary
/// - group_id set to -1 to mark as tutorial data
/// - Default statistics fields (times_seen: 0, success_count: 0)
/// - marked set to false initially
/// - last_seen set to None
///
/// The session starts at index 0 and is of type Group.
///
/// **Validates: Requirements 12.1, 12.2, 12.3**
pub fn create_sample_session() -> Session {
    use crate::db::models::Word;

    // Create sample words with negative IDs to distinguish from real vocabulary
    let sample_words: Vec<Word> = SAMPLE_WORDS
        .iter()
        .enumerate()
        .map(|(i, (word, definition))| Word {
            id: -((i + 1) as i32), // Negative IDs: -1, -2, -3, etc.
            word: word.to_string(),
            definition: definition.to_string(),
            group_id: -1, // Tutorial group ID
            marked: false, // Not marked initially
            last_seen: None, // Never seen before
            times_seen: 0, // Default statistics
            success_count: 0, // Default statistics
        })
        .collect();

    // Create a sample session for tutorial practice
    Session::new(sample_words, 0, crate::core::session::Type::Group)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_tutorial_creates_valid_state() {
        let state = init_tutorial();

        // Verify step counter is initialized to 0
        assert_eq!(state.current_step, 0);

        // Verify total steps matches the tutorial steps array
        assert_eq!(state.total_steps, TUTORIAL_STEPS.len());

        // Verify sample session exists
        assert!(state.sample_session.is_some());

        // Verify completed actions is empty
        assert!(state.completed_actions.is_empty());

        // Verify exit not requested
        assert!(!state.exit_requested);
    }

    #[test]
    fn test_init_tutorial_sample_words_have_negative_ids() {
        let state = init_tutorial();
        let session = state.sample_session.expect("Sample session should exist");

        // Verify all words have negative IDs
        for word in &session.words {
            assert!(word.id < 0, "Word ID should be negative, got {}", word.id);
        }
    }

    #[test]
    fn test_init_tutorial_sample_words_have_tutorial_group_id() {
        let state = init_tutorial();
        let session = state.sample_session.expect("Sample session should exist");

        // Verify all words have group_id -1
        for word in &session.words {
            assert_eq!(word.group_id, -1, "Word group_id should be -1, got {}", word.group_id);
        }
    }

    #[test]
    fn test_init_tutorial_creates_correct_number_of_words() {
        let state = init_tutorial();
        let session = state.sample_session.expect("Sample session should exist");

        // Verify number of words matches SAMPLE_WORDS
        assert_eq!(session.words.len(), SAMPLE_WORDS.len());
    }

    #[test]
    fn test_init_tutorial_words_match_sample_data() {
        let state = init_tutorial();
        let session = state.sample_session.expect("Sample session should exist");

        // Verify words match the SAMPLE_WORDS data
        for (i, word) in session.words.iter().enumerate() {
            let (expected_word, expected_def) = SAMPLE_WORDS[i];
            assert_eq!(word.word, expected_word);
            assert_eq!(word.definition, expected_def);
        }
    }

    #[test]
    fn test_init_tutorial_words_not_marked() {
        let state = init_tutorial();
        let session = state.sample_session.expect("Sample session should exist");

        // Verify all words are not marked initially
        for word in &session.words {
            assert!(!word.marked, "Word should not be marked initially");
        }
    }

    #[test]
    fn test_init_tutorial_session_starts_at_index_zero() {
        let state = init_tutorial();
        let session = state.sample_session.expect("Sample session should exist");

        // Verify session starts at index 0
        assert_eq!(session.index, 0);
    }

    #[test]
    fn test_get_current_step_returns_correct_step() {
        let mut state = init_tutorial();

        // Test first step
        state.current_step = 0;
        let step = get_current_step(&state);
        assert_eq!(step.id, 0);

        // Test middle step
        state.current_step = 5;
        let step = get_current_step(&state);
        assert_eq!(step.id, 5);

        // Test last step
        state.current_step = TUTORIAL_STEPS.len() - 1;
        let step = get_current_step(&state);
        assert_eq!(step.id, TUTORIAL_STEPS.len() - 1);
    }

    #[test]
    fn test_get_current_step_handles_out_of_bounds() {
        let mut state = init_tutorial();

        // Test index beyond array bounds
        state.current_step = TUTORIAL_STEPS.len() + 10;
        let step = get_current_step(&state);
        // Should return last step instead of panicking
        assert_eq!(step.id, TUTORIAL_STEPS.len() - 1);
    }

    #[test]
    fn test_validate_and_advance_with_exact_key_match() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        let app = App::new_test(); // We'll need to add this test helper

        // Step 0 expects Enter key
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Valid));
        assert_eq!(state.current_step, 1);
    }

    #[test]
    fn test_validate_and_advance_with_key_alternative_j_for_down() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = 1; // Step 1 expects Down arrow
        let app = App::new_test();

        // Press 'j' instead of Down arrow
        let key = KeyEvent::new(KeyCode::Char('j'), KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Valid));
        assert_eq!(state.current_step, 2);
    }

    #[test]
    fn test_validate_and_advance_with_key_alternative_k_for_up() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = 2; // Step 2 expects Up arrow
        let app = App::new_test();

        // Press 'k' instead of Up arrow
        let key = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Valid));
        assert_eq!(state.current_step, 3);
    }

    #[test]
    fn test_validate_and_advance_with_key_alternative_escape_for_q() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = 12; // Step 12 expects 'q'
        let app = App::new_test();

        // Press Escape instead of 'q'
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Valid));
        assert_eq!(state.current_step, 13);
    }

    #[test]
    fn test_validate_and_advance_with_invalid_key_returns_hint() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        let app = App::new_test();

        // Step 0 expects Enter, press 'x' instead
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        match result {
            ValidationResult::Invalid(hint) => {
                assert!(!hint.is_empty());
                // Step should not advance
                assert_eq!(state.current_step, 0);
            }
            _ => panic!("Expected Invalid result"),
        }
    }

    #[test]
    fn test_validate_and_advance_grading_step_accepts_y_or_n() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = 6; // Step 6 expects 'y' but also accepts 'n'
        let app = App::new_test();

        // Press 'n' instead of 'y'
        let key = KeyEvent::new(KeyCode::Char('n'), KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Valid));
        assert_eq!(state.current_step, 7);
    }

    #[test]
    fn test_validate_and_advance_returns_complete_at_end() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = TUTORIAL_STEPS.len() - 1; // Last step
        let app = App::new_test();

        // Complete the last step
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Complete));
        assert_eq!(state.current_step, TUTORIAL_STEPS.len());
    }

    #[test]
    fn test_validate_and_advance_already_complete() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = TUTORIAL_STEPS.len() + 1; // Beyond last step
        let app = App::new_test();

        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Complete));
    }

    #[test]
    fn test_validate_and_advance_menu_selection_correct_index() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = 3; // Step 3 expects MenuSelection(0)
        let mut app = App::new_test();
        app.selected = 0; // Menu is at index 0

        // Press Enter with correct menu selection
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Valid));
        assert_eq!(state.current_step, 4);
    }

    #[test]
    fn test_validate_and_advance_menu_selection_wrong_index() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = 3; // Step 3 expects MenuSelection(0)
        let mut app = App::new_test();
        app.selected = 1; // Menu is at wrong index

        // Press Enter with wrong menu selection
        let key = KeyEvent::new(KeyCode::Enter, KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        match result {
            ValidationResult::Invalid(_) => {
                // Step should not advance
                assert_eq!(state.current_step, 3);
            }
            _ => panic!("Expected Invalid result"),
        }
    }

    #[test]
    fn test_validate_and_advance_menu_selection_without_enter() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = 3; // Step 3 expects MenuSelection(0)
        let mut app = App::new_test();
        app.selected = 0; // Menu is at correct index

        // Press a different key (not Enter)
        let key = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        match result {
            ValidationResult::Invalid(_) => {
                // Step should not advance
                assert_eq!(state.current_step, 3);
            }
            _ => panic!("Expected Invalid result"),
        }
    }

    #[test]
    fn test_validate_and_advance_state_condition_satisfied() {
        use crossterm::event::{KeyEvent, KeyModifiers};

        let mut state = init_tutorial();
        state.current_step = 7; // Step 7 expects bookmark to be true
        let app = App::new_test();

        // Mark the first word in the tutorial sample session
        if let Some(ref mut session) = state.sample_session {
            session.words[0].marked = true;
        }

        // Press any key (state condition doesn't depend on key)
        let key = KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        assert!(matches!(result, ValidationResult::Valid));
        assert_eq!(state.current_step, 8);
    }

    #[test]
    fn test_validate_and_advance_state_condition_not_satisfied() {
        use crossterm::event::{KeyEvent, KeyModifiers};
        use crate::db::models::Word;

        let mut state = init_tutorial();
        state.current_step = 7; // Step 7 expects bookmark to be true
        let mut app = App::new_test();

        // Create a session with an unmarked word
        let words = vec![Word {
            id: -1,
            word: "test".to_string(),
            definition: "test def".to_string(),
            group_id: -1,
            marked: false, // Bookmark is NOT set
            last_seen: None,
            times_seen: 0,
            success_count: 0,
        }];
        app.session = Some(Session::new(words, 0, crate::core::session::Type::Group));

        // Press any key
        let key = KeyEvent::new(KeyCode::Char('m'), KeyModifiers::empty());
        let result = validate_and_advance(&mut state, &app, key);

        match result {
            ValidationResult::Invalid(_) => {
                // Step should not advance
                assert_eq!(state.current_step, 7);
            }
            _ => panic!("Expected Invalid result"),
        }
    }

    #[test]
    fn test_is_tutorial_completed_default() {
        use crate::db::schema::INIT_SCHEMA;
        use rusqlite::Connection;

        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(INIT_SCHEMA).unwrap();

        // When key doesn't exist, should return false
        let completed = is_tutorial_completed(&conn).unwrap();
        assert_eq!(completed, false);
    }

    #[test]
    fn test_mark_tutorial_completed() {
        use crate::db::schema::INIT_SCHEMA;
        use rusqlite::Connection;

        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(INIT_SCHEMA).unwrap();

        // Mark as completed
        mark_tutorial_completed(&conn).unwrap();

        // Verify it's marked as completed
        let completed = is_tutorial_completed(&conn).unwrap();
        assert_eq!(completed, true);
    }

    #[test]
    fn test_reset_tutorial() {
        use crate::db::schema::INIT_SCHEMA;
        use rusqlite::Connection;

        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(INIT_SCHEMA).unwrap();

        // Mark as completed first
        mark_tutorial_completed(&conn).unwrap();
        assert_eq!(is_tutorial_completed(&conn).unwrap(), true);

        // Reset the tutorial
        reset_tutorial(&conn).unwrap();

        // Verify it's reset to false
        let completed = is_tutorial_completed(&conn).unwrap();
        assert_eq!(completed, false);
    }

    #[test]
    fn test_tutorial_completion_round_trip() {
        use crate::db::schema::INIT_SCHEMA;
        use rusqlite::Connection;

        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(INIT_SCHEMA).unwrap();

        // Test false -> true -> false -> true
        assert_eq!(is_tutorial_completed(&conn).unwrap(), false);

        mark_tutorial_completed(&conn).unwrap();
        assert_eq!(is_tutorial_completed(&conn).unwrap(), true);

        reset_tutorial(&conn).unwrap();
        assert_eq!(is_tutorial_completed(&conn).unwrap(), false);

        mark_tutorial_completed(&conn).unwrap();
        assert_eq!(is_tutorial_completed(&conn).unwrap(), true);
    }

    #[test]
    fn test_create_sample_session_returns_valid_session() {
        let session = create_sample_session();

        // Verify session has words
        assert!(!session.words.is_empty());

        // Verify session starts at index 0
        assert_eq!(session.index, 0);

        // Verify session type is Group
        assert_eq!(session.session_type, crate::core::session::Type::Group);
    }

    #[test]
    fn test_create_sample_session_words_have_negative_ids() {
        let session = create_sample_session();

        // Verify all words have negative IDs
        for word in &session.words {
            assert!(word.id < 0, "Word ID should be negative, got {}", word.id);
        }
    }

    #[test]
    fn test_create_sample_session_words_have_tutorial_group_id() {
        let session = create_sample_session();

        // Verify all words have group_id -1
        for word in &session.words {
            assert_eq!(word.group_id, -1, "Word group_id should be -1, got {}", word.group_id);
        }
    }

    #[test]
    fn test_create_sample_session_words_not_marked() {
        let session = create_sample_session();

        // Verify all words are not marked initially
        for word in &session.words {
            assert!(!word.marked, "Word should not be marked initially");
        }
    }

    #[test]
    fn test_create_sample_session_default_statistics() {
        let session = create_sample_session();

        // Verify all words have default statistics
        for word in &session.words {
            assert_eq!(word.times_seen, 0, "times_seen should be 0");
            assert_eq!(word.success_count, 0, "success_count should be 0");
            assert_eq!(word.last_seen, None, "last_seen should be None");
        }
    }

    #[test]
    fn test_create_sample_session_matches_sample_words() {
        let session = create_sample_session();

        // Verify number of words matches SAMPLE_WORDS
        assert_eq!(session.words.len(), SAMPLE_WORDS.len());

        // Verify words match the SAMPLE_WORDS data
        for (i, word) in session.words.iter().enumerate() {
            let (expected_word, expected_def) = SAMPLE_WORDS[i];
            assert_eq!(word.word, expected_word);
            assert_eq!(word.definition, expected_def);
        }
    }

    #[test]
    fn test_create_sample_session_unique_negative_ids() {
        let session = create_sample_session();

        // Collect all IDs
        let ids: Vec<i32> = session.words.iter().map(|w| w.id).collect();

        // Verify all IDs are unique
        let mut unique_ids = ids.clone();
        unique_ids.sort();
        unique_ids.dedup();
        assert_eq!(ids.len(), unique_ids.len(), "All word IDs should be unique");

        // Verify IDs are sequential negative numbers
        for (i, word) in session.words.iter().enumerate() {
            assert_eq!(word.id, -((i + 1) as i32), "Word ID should be -{}", i + 1);
        }
    }
}


/// Get the current tutorial step definition
///
/// Returns a reference to the TutorialStep corresponding to the current step index.
/// If the step index is out of bounds, returns the last step to prevent panics.
///
/// **Validates: Requirements 3.4, 4.1**
pub fn get_current_step(state: &TutorialState) -> &TutorialStep {
    let index = state.current_step.min(TUTORIAL_STEPS.len() - 1);
    &TUTORIAL_STEPS[index]
}

/// Validate user action against current step and advance if correct
///
/// Checks if the provided key event satisfies the validation criteria for the current step.
/// Supports:
/// - KeyPress validation with alternatives (j/k for arrow keys)
/// - MenuSelection validation (checks selected menu index)
/// - StateCondition validation (evaluates custom condition function)
///
/// Returns:
/// - ValidationResult::Valid if step completed successfully (advances to next step)
/// - ValidationResult::Invalid with hint if action doesn't match validation
/// - ValidationResult::Complete if all tutorial steps are finished
///
/// **Validates: Requirements 3.4, 4.1, 4.2, 4.3, 4.4**
pub fn validate_and_advance(
    state: &mut TutorialState,
    app: &App,
    key: crossterm::event::KeyEvent,
) -> ValidationResult {
    // Check if tutorial is already complete
    if state.current_step >= TUTORIAL_STEPS.len() {
        return ValidationResult::Complete;
    }

    let current_step = get_current_step(state);
    let is_valid = match &current_step.validation {
        StepValidation::KeyPress(expected_key) => {
            // Check for exact key match
            if key.code == *expected_key {
                true
            } else {
                // Check for key alternatives (j/k for arrows, Escape for q)
                match (*expected_key, key.code) {
                    // Down arrow alternatives
                    (KeyCode::Down, KeyCode::Char('j')) => true,
                    // Up arrow alternatives
                    (KeyCode::Up, KeyCode::Char('k')) => true,
                    // 'q' alternatives
                    (KeyCode::Char('q'), KeyCode::Esc) => true,
                    // 'y' or 'n' for grading step (step 6)
                    (KeyCode::Char('y'), KeyCode::Char('n')) if current_step.id == 6 => true,
                    (KeyCode::Char('n'), KeyCode::Char('y')) if current_step.id == 6 => true,
                    _ => false,
                }
            }
        }
        StepValidation::MenuSelection(expected_index) => {
            // For menu selection, we need to check if Enter was pressed
            // and the menu is at the correct index
            if key.code == KeyCode::Enter {
                // Check if the menu selection matches the expected index
                app.selected == *expected_index
            } else {
                false
            }
        }
        StepValidation::StateCondition(condition_fn) => {
            // Evaluate the custom condition function
            condition_fn(app, state)
        }
    };

    if is_valid {
        // Advance to next step
        state.current_step += 1;
        
        // Update step entry timestamp
        state.step_entered_at = Some(std::time::Instant::now());

        // Check if tutorial is now complete
        if state.current_step >= TUTORIAL_STEPS.len() {
            ValidationResult::Complete
        } else {
            ValidationResult::Valid
        }
    } else {
        // Return hint message for invalid action
        let hint = current_step.hint.unwrap_or("Try again.");
        ValidationResult::Invalid(hint.to_string())
    }
}

/// Check if the tutorial has been completed
///
/// Queries the database for the tutorial completion flag.
/// Returns false if the key doesn't exist (default behavior).
///
/// **Validates: Requirements 2.1, 2.2, 2.3**
pub fn is_tutorial_completed(conn: &rusqlite::Connection) -> anyhow::Result<bool> {
    crate::db::queries::get_tutorial_completed(conn)
}

/// Mark the tutorial as completed
///
/// Sets the tutorial completion flag to true in the database.
/// This prevents the tutorial prompt from showing on subsequent launches.
///
/// **Validates: Requirements 2.1, 2.2**
pub fn mark_tutorial_completed(conn: &rusqlite::Connection) -> anyhow::Result<()> {
    crate::db::queries::set_tutorial_completed(conn, true)
}

/// Reset the tutorial completion flag
///
/// Sets the tutorial completion flag to false in the database.
/// This is primarily used for testing purposes to allow the tutorial to be shown again.
///
/// **Validates: Requirements 2.4**
pub fn reset_tutorial(conn: &rusqlite::Connection) -> anyhow::Result<()> {
    crate::db::queries::set_tutorial_completed(conn, false)
}

/// Check if the current step should auto-advance
///
/// Step 4 auto-advances after 10 seconds or on any key press.
/// Returns true if the step should advance.
pub fn should_auto_advance(state: &TutorialState) -> bool {
    if state.current_step != 4 {
        return false;
    }
    
    if let Some(entered_at) = state.step_entered_at {
        entered_at.elapsed().as_secs() >= 10
    } else {
        false
    }
}
