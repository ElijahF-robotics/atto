// Standard imports
use std::io::{self, Write};
use std::fs::File;
use std::env::args;
use std::fs;

// Crossterm for terminal management
use crossterm::{
    terminal, cursor, event::{self, Event, KeyCode},
    execute, queue
};
use crossterm::event::KeyModifiers;


fn main() -> io::Result<()> {
    // Get arguments
    let args: Vec<String> = args().collect();

    // Standard Output
    let mut stdout = io::stdout();

    // Buffers, one general and one for filename
    let mut buffer = vec![String::new()]; // Lines of text
    let mut save_buffer = String::new();

    // Cursor position
    let mut cursor_x = 0;
    let mut cursor_y = 0;

    // In save mode or not?
    let mut save_mode = false;

    // Scrolling variables
    let mut scroll_x = 0;
    let mut scroll_y = 0;

    // If argument supplied, it's the filename!
    if args.len() == 2 {
        save_buffer = args[1].clone();
        if let Ok(contents) = fs::read_to_string(&save_buffer) {
            buffer = contents.lines()
                .map(|s| s.to_string())
                .collect();
        } else {
            buffer = vec![String::new()];
        }
    }

    // Put our terminal in raw mode
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen)?;

    loop {
        // Draw current state
        draw(&mut stdout, &buffer, &save_buffer, cursor_x, cursor_y, scroll_x, scroll_y)?;

        // Get current width and height of terminal
        let (width, height) = terminal::size()?;

        // Read key event
        if let Event::Key(key_event) = event::read()? {
            match key_event.code {
                KeyCode::Char(c) => {
                    if save_mode {
                        save_buffer.insert(cursor_x, c);
                        cursor_x += 1;
                    } else {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                            // Ctrl-q
                            if c == 'q' {
                                execute!(
                                stdout,
                                cursor::MoveTo(0, 0),
                                terminal::Clear(terminal::ClearType::All),
                                cursor::Show // Make sure cursor is visible
                            )?;
                                terminal::disable_raw_mode()?;

                                return Ok(())
                            }

                            // Ctrl-s
                            if c == 's' {

                                cursor_x = 0;
                                cursor_y = (height-1) as usize;
                                save_mode = true;
                            }
                        } else {
                            insert(&mut buffer, cursor_x, cursor_y, c); cursor_x += 1;
                        }
                    }
                }

                KeyCode::Enter => {
                    if save_mode {
                        // Save the file with name
                        create_and_save_file(&buffer, &save_buffer)?;

                        // Currently just exit the program
                        execute!(
                                stdout,
                                cursor::MoveTo(0, 0),
                                terminal::Clear(terminal::ClearType::All),
                                cursor::Show)?; // Make sure cursor is visible

                        terminal::disable_raw_mode()?;
                        return Ok(())

                    } else {
                        if cursor_x >= buffer[cursor_y].len() {
                            // Insert blank line
                            buffer.insert(cursor_y+1, String::new());
                            cursor_y += 1;
                            cursor_x = 0;
                        } else {
                            // Get the part after the cursor
                            let remainder = buffer[cursor_y][cursor_x..].to_string();

                            // Keep only the part before the cursor
                            buffer[cursor_y].truncate(cursor_x);

                            // Insert the remainder as a new line
                            buffer.insert(cursor_y + 1, remainder);

                            cursor_y += 1;
                            cursor_x = 0;

                        }
                    }
                }

                KeyCode::Backspace => {
                    if save_mode {
                        if cursor_x > 0 {
                            save_buffer.remove(cursor_x - 1);
                            cursor_x -= 1;
                        }
                    } else {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                            // Delete entire line
                            buffer[cursor_y].truncate(0)
                        } else {
                            if cursor_x > 0 {
                                buffer[cursor_y].remove(cursor_x - 1);
                                cursor_x -= 1;
                            } else if cursor_y > 0 {
                                let current_line = buffer.remove(cursor_y);
                                cursor_y -= 1;
                                cursor_x = buffer[cursor_y].len();
                                buffer[cursor_y].push_str(&current_line); // Append current to previous
                                queue!(
                            stdout,
                            cursor::MoveTo(0, buffer.len() as u16),
                            terminal::Clear(terminal::ClearType::CurrentLine)
                        )?;
                            }
                        }
                    }
                }

                KeyCode::Up => {
                    if cursor_y > 0 {
                        cursor_y -= 1;
                        if cursor_x > buffer[cursor_y].len(){
                            cursor_x = buffer[cursor_y].len();
                        }
                    }
                }

                KeyCode::Down => {
                    if cursor_y < buffer.len() - 1{
                        cursor_y += 1;
                        if cursor_x > buffer[cursor_y].len(){
                            cursor_x = buffer[cursor_y].len();
                        }
                    }
                }

                KeyCode::Left => {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        cursor_x = 0;
                    }
                    if cursor_x > 0 {
                        cursor_x -= 1;
                    }
                }

                KeyCode::Right => {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        cursor_x = buffer[cursor_y].len();
                    } else if cursor_x < buffer[cursor_y].len() {
                        cursor_x += 1;
                    }

                    // After moving cursor, check if we need to scroll
                    let visible_cols = (width - 1) as usize;
                    if cursor_x >= scroll_x + visible_cols {
                        scroll_x = cursor_x - visible_cols + 1;
                    }
                }

                _ => {}
            }
        }
    }
}

fn draw(stdout: &mut io::Stdout, buffer: &[String], save_buffer: &String, cursor_x: usize, cursor_y: usize, scroll_x: usize, scroll_y: usize) -> io::Result<()> {
    let (width, height) = terminal::size()?;

    // Move cursor to top-left
    queue!(stdout, cursor::MoveTo(0, 0))?;

    // Draw each line of the buffer
    for (i, line) in buffer.iter().enumerate() {
        if i >= (height - 1) as usize {break;}
        queue!(stdout,
            cursor::MoveTo(0, i as u16),
            terminal::Clear(terminal::ClearType::CurrentLine)
        )?;
        let visible_string = line[scroll_x..].to_string();
        write!(stdout, "{}", visible_string)?;
    }

    // Draw status line at bottom
    queue!(stdout, cursor::MoveTo(0, height - 2))?;
    write!(stdout, "{}", "*".repeat(width as usize))?; // Line of asterisks

    // Draw message on status line (like filename prompt)
    queue!(stdout, cursor::MoveTo(0, height - 1), terminal::Clear(terminal::ClearType::CurrentLine))?;
    write!(stdout, "{}", save_buffer)?;

    // Position cursor where user expects it
    queue!(stdout, cursor::MoveTo(cursor_x as u16, cursor_y as u16))?;

    // Actually write everything to screen
    stdout.flush()?;

    Ok(())
}
fn insert(buffer: &mut Vec<String>, cursor_x: usize, cursor_y: usize, c: char) {
    // cursor_x == line we're on
    let line = &mut buffer[cursor_y];

    line.insert(cursor_x, c);
}

fn create_and_save_file(buffer: &Vec<String>, filename: &String) -> io::Result<()> {
    if !filename.is_empty() {
        let mut file = File::create(filename)?;
        file.write_all(buffer.join("\n").as_bytes())?;
    }
    Ok(())
}
