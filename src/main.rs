use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;

use crabablanca::board::Board;
use crabablanca::renderer::Renderer;

use crossterm::{execute, cursor};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

fn main() -> Result<(), Box<dyn Error>>{

    let mut board: Board = Board::new(); 

    let mut renderer = Renderer::new()?;

    // renderer.parse_board(&board)?;

    /*
    let move_list = board.generate_move_list();

    
    for move_ in move_list {
        renderer.parse_board(&move_)?;
        std::thread::sleep(std::time::Duration::from_secs(1))
    }
    */

    enable_raw_mode()?;

    loop {
        renderer.parse_board(&board)?;
        let mut input = String::new();
        // Capture input
        loop {
            if let Event::Key(key_event) = read()? {
                match key_event.code {
                    KeyCode::Enter => break,
                    KeyCode::Backspace => {
                        input.pop();
                        print!("\x08 \x08");
                        io::stdout().flush().unwrap();
                    }
                    KeyCode::Char(c) => {
                        input.push(c);
                        print!("{}", c);
                        io::stdout().flush().unwrap();
                    }
                    _ => {}
                }
            }
        }


        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine)
        )?;

        if input == "exit" || input == "quit" {
            disable_raw_mode()?;
            println!();
            return Ok(())
        } else {
            let boardop = board.parse_input(&input);
            match boardop {
                Some(b) => board = b,
                None => {
                    println!("Invalid or ambiguous command");
                    execute!(
                        io::stdout(),
                        cursor::MoveToColumn(0),
                        Clear(ClearType::CurrentLine)
                    )?;
                }
            }
        }

        input.clear();
        
    }
}

