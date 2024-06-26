use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;
use std::env;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

use crabablanca::board::Board;
use crabablanca::renderer::Renderer;
use crabablanca::engine::Node;

use crossterm::{execute, cursor};
use crossterm::event::{read, Event, KeyCode};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

fn main() -> Result<(), Box<dyn Error>>{

    env::set_var("RUST_BACKTRACE", "full");

    let mut board: Board = Board::new(); 

    let mut renderer = Renderer::new()?;

    let player_colour: u8 = 1; // 1 for white, 0 for black

    enable_raw_mode()?;

    loop {
        
        renderer.parse_board(&board)?;

        let search_node = Rc::new(RefCell::new(Node::new(&board)));

        // The issue is that search_node gets borrowed while iterating over children
        // Which then means that it cannot be mutably borrowed when trying to propagate values into it
        // There'll be an elegant way to do this, I just need to work out what
        Node::search_next_ply(&search_node);
        // for child in search_node.borrow().children.iter() {
        //     Node::search_next_ply(child);
        // }
        
        println!("{}, {}", search_node.borrow().static_eval, search_node.borrow().deep_eval);
        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine)
        )?;

        if board.white_checkmate {
            println!("Checkmate - black wins");
            execute!(
                io::stdout(),
                cursor::MoveToColumn(0),
                Clear(ClearType::CurrentLine)
            )?;
            disable_raw_mode()?;
            break;
        } else if board.black_checkmate {
            println!("Checkmate - white wins");
            execute!(
                io::stdout(),
                cursor::MoveToColumn(0),
                Clear(ClearType::CurrentLine)
            )?;
            disable_raw_mode()?;
            break;
        }

        if board.to_move == player_colour {

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
                let boardop: Option<Board> = board.parse_input(&input);
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
        } else {
            if let Some(next_move) = search_node.borrow().best_next_move.clone() {
                board = next_move;
            };
        }

    }
    Ok(())
}

