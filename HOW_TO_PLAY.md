# How to Play

## Quick Start

1. **First Time Setup**
   ```bash
   cargo run --release -- seed data/vocab.txt
   ```

2. **Launch the App**
   ```bash
   cargo run --release
   ```

## Main Menu

Use **arrow keys** or **j/k** to navigate, **Enter** to select:

- **Continue Learning** - Practice words by group (saves your progress)
- **Review Marks** - Study words you've marked
- **Revise Weak** - Focus on words you got wrong
- **Exit** - Quit the app

## Practice Mode

### The Flow

1. See a word on screen
2. Try to recall its definition
3. Press **s** to show the definition
4. Grade yourself: **y** (correct) or **n** (wrong)
5. Press **Enter** to move to the next word

### Controls

| Key | Action |
|-----|--------|
| **s** | Show definition |
| **y** | Mark as correct (plays correct sound) |
| **n** | Mark as wrong (plays wrong sound) |
| **m** | Toggle bookmark (for later review) |
| **Enter** | Next word (after grading) |
| **q** or **Esc** | Return to menu |

### What You See

- **Word counter** - Your position in the session
- **Group & ID** - Which vocabulary group you're in
- **Stats** - How long since you last saw this word, and your accuracy
- **Star (*)** - Shows if word is bookmarked

## Test Mode

Test mode challenges you to type the word from its definition!

### The Flow

1. See a definition on screen
2. Press **i** to enter insert mode
3. Type the word
4. Press **Enter** to submit
5. See if you got it right (green = correct, red = wrong)
6. Press **Enter** again to move to the next word

### Controls

| Key | Action |
|-----|--------|
| **i** | Enter insert mode (start typing) |
| **Esc** | Exit insert mode |
| **Backspace** | Delete last character |
| **Enter** | Submit answer / Next word |
| **m** | Toggle bookmark |
| **q** or **Esc** | Return to menu (when not in insert mode) |

### Features

- Case-insensitive matching
- Word is revealed after you submit
- Same bookmarking and stats tracking as Practice mode

## Tips

- Be honest when grading yourself - it helps the app track weak words
- Use **m** to bookmark tricky words for focused review later
- Your progress auto-saves, so you can quit anytime
- Words you mark wrong will appear in "Revise Weak" mode
- Try Test mode for a more challenging practice session!
