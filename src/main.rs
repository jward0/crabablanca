use core::time;
use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;
use std::env;

use std::rc::{Rc, Weak};
use std::cell::RefCell;

use std::collections::HashMap;

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

    let mut player_colour: Vec<u8> = vec![1, 0]; // [1] for white, [0] for black, [] for engine vs. engine, [1, 0] for self vs. self
    let depth: usize = 4;
    let mut showme = false;

    // TODO: doubled pawn eval, en passant, transposition tables, multithreading
    // Performance improvements: block_ray, knight_move_mask, move_piece (maybe?)

    enable_raw_mode()?;

    for _ in 0..100 {
        // std::thread::sleep(time::Duration::from_secs(1));

        let search_node = Rc::new(RefCell::new(Node::new(&board)));
        
        renderer.parse_board(&board)?;

        if showme {
            let move_list = board.generate_move_list();

            for move_ in move_list {
                renderer.parse_board(&move_)?;
                std::thread::sleep(std::time::Duration::from_secs(1))
            }
        }

        renderer.parse_board(&board)?;

        // Node::search_n_plys(&search_node, depth);
        Node::process_node_cell(&search_node, depth);
        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            Clear(ClearType::CurrentLine)
        )?;
        println!("{}, {}, {}, {}", search_node.borrow().children.len(), search_node.borrow().static_eval, search_node.borrow().deep_eval, depth);
                
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

        if player_colour.contains(&board.to_move){

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

            match input.as_str() {
                "exit" => {
                    disable_raw_mode()?;
                    println!();
                    return Ok(())
                },
                "quit" => {
                    disable_raw_mode()?;
                    println!();
                    return Ok(())
                },
                "next" => {
                    if let Some(next_move) = search_node.borrow().best_next_move.clone() {
                        board = next_move;
                    };
                },
                "preview" => {
                    if let Some(next_move) = search_node.borrow().best_next_move.clone() {
                        renderer.parse_board(&next_move)?;
                        std::thread::sleep(time::Duration::from_secs(3));
                        renderer.parse_board(&board)?;
                    };
                },
                "play" => player_colour = vec![],
                "white" => player_colour = vec![1],
                "black" => player_colour = vec![2],
                "showme" => showme = true,
                "!showme" => showme = false,
                _ => {
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
                            std::thread::sleep(time::Duration::from_secs(1));
                        }
                    }
                }
            }

            // if input == "exit" || input == "quit" {
            //     disable_raw_mode()?;
            //     println!();
            //     return Ok(())
            // } else if input == "next" {
            //     if let Some(next_move) = search_node.borrow().best_next_move.clone() {
            //         board = next_move;
            //     };
            // } else if input == "play" {
            //     player_colour = vec![];
            // } else if input == "white" {
            //     player_colour = vec![1];
            // } else if input == "black" {
            //     player_colour = vec![0];
            // } else if input == "showme" {
            //     showme = true;
            // } else {
            //     let boardop: Option<Board> = board.parse_input(&input);
            //     match boardop {
            //         Some(b) => board = b,
            //         None => {
            //             println!("Invalid or ambiguous command");
            //             execute!(
            //                 io::stdout(),
            //                 cursor::MoveToColumn(0),
            //                 Clear(ClearType::CurrentLine)
            //             )?;
            //             std::thread::sleep(time::Duration::from_secs(1));
            //         }
            //     }
            // }

            input.clear();    
        } else {
            if let Some(next_move) = search_node.borrow().best_next_move.clone() {
                board = next_move;
            };
        }

    }
    disable_raw_mode()?;
    Ok(())
}

