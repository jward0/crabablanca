use std::error::Error;
use std::io::{self, Write};

use crabablanca::board::Board;
use crabablanca::renderer::Renderer;

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};
use crossterm::style::{Color::{Green, White}, Colors, Print, SetColors};

use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{self, Duration};


fn main() -> Result<(), Box<dyn Error>>{
    let board: Board = Board::new(); 

    let mut renderer = Renderer::new()?;

    std::thread::sleep(std::time::Duration::from_secs(3));

    renderer.write_to_square((2, 3), 'p', 'b')?;

    std::thread::sleep(std::time::Duration::from_secs(3));

    renderer.write_to_square((2, 3), 'n', 'b')?;

    std::thread::sleep(std::time::Duration::from_secs(3));

    renderer.write_to_square((2, 3), 'n', 'w')?;

    std::thread::sleep(std::time::Duration::from_secs(3));

    renderer.clear_square((2, 3))?;

    Ok(())
}

