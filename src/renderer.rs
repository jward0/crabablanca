use std::error::Error;
use std::io::{self, Stdout, Write};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};
use crossterm::style::{Color::{self, White, Reset, Black, Green, Blue}, Colors};

use crate::board::Board;
use crate::bit_functions::{iterate_over, bit_to_coord};

pub struct Renderer {
    stdout: Stdout
}

impl Renderer {
    pub fn new() -> Result<Renderer, Box<dyn Error>> {
        let mut stdout = io::stdout();

        stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for y in 0..8 {
            for x in 0..8 {
                if (x + y) % 2 == 0 {
                    stdout
                        .queue(cursor::MoveTo(2*x, y))?
                        .queue(style::PrintStyledContent("██".green()))?;
                } else {
                    stdout
                        .queue(cursor::MoveTo(2*x, y))?
                        .queue(style::PrintStyledContent("██".blue()))?;
                }
            }
        }

        stdout.queue(cursor::MoveTo(0, 8))?;
        stdout.flush()?;

        let r = Renderer {
            stdout
        };

        Ok(r)
    }

    pub fn clear_board(&mut self) -> Result<(), Box<dyn Error>> {

        let mut bg_colour: Color;

        for y in 0..8 {
            for x in 0..8 {
                bg_colour = if (x+y) % 2 == 0 {Green} else {Blue};
                self.stdout
                    .queue(cursor::MoveTo(2*x, y))?
                    .queue(style::SetBackgroundColor(bg_colour))?
                    .queue(style::Print("  "))?;
            }
        }

        Ok(())
    }

    pub fn clear_square(&mut self, index: (u16, u16)) -> Result<(), Box<dyn Error>> {

        let bg_colour = if (index.0 + index.1) % 2 == 0 {Green} else {Blue};
        self.stdout
            .queue(cursor::MoveTo(2*index.0, index.1))?
            .queue(style::SetBackgroundColor(bg_colour))?
            .queue(style::Print("  "))?;

        self.reset_cursor()?;

        Ok(())
    }

    pub fn write_to_square(&mut self, index: (u16, u16), piece: char, colour: char) -> Result<(), Box<dyn Error>> {

        let bg_colour = if (index.0 + index.1) % 2 == 1 {Green} else {Blue};

        let symbol = match piece {
            'k' => '♚',
            'q' => '♛',
            'r' => '♜',
            'b' => '♝',
            'n' => '♞',
            'p' => '♟',
            _ =>  unreachable!("Invalid piece type")
        };

        if colour == 'w' {
            self.stdout.queue(style::SetForegroundColor(White))?;
        } else {
            self.stdout.queue(style::SetForegroundColor(Black))?;
        }

        self.stdout
            .queue(cursor::MoveTo(2*index.0, 7-index.1))?
            .queue(style::SetBackgroundColor(bg_colour))?
            .queue(style::Print(format!("{} ", symbol)))?;
        
        self.reset_cursor()?;

        Ok(())
    }

    pub fn reset_cursor(&mut self) -> Result<(), Box<dyn Error>> {
        self.stdout
            .queue(cursor::MoveTo(0, 8))?
            .queue(style::SetColors(Colors::new(Reset, Reset)))?
            .flush()?;

        Ok(())
    }

    pub fn parse_board(&mut self, board: &Board) -> Result<(), Box<dyn Error>> {

        self.clear_board()?;

        for p in iterate_over(board.white_pawns).iter() {
            self.write_to_square(bit_to_coord(*p), 'p', 'w')?;
        }

        for p in iterate_over(board.black_pawns).iter() {
            self.write_to_square(bit_to_coord(*p), 'p', 'b')?;
        }

        for p in iterate_over(board.white_bishops).iter() {
            self.write_to_square(bit_to_coord(*p), 'b', 'w')?;
        }

        for p in iterate_over(board.black_bishops).iter() {
            self.write_to_square(bit_to_coord(*p), 'b', 'b')?;
        }

        for p in iterate_over(board.white_knights).iter() {
            self.write_to_square(bit_to_coord(*p), 'n', 'w')?;
        }

        for p in iterate_over(board.black_knights).iter() {
            self.write_to_square(bit_to_coord(*p), 'n', 'b')?;
        }

        for p in iterate_over(board.white_rooks).iter() {
            self.write_to_square(bit_to_coord(*p), 'r', 'w')?;
        }

        for p in iterate_over(board.black_rooks).iter() {
            self.write_to_square(bit_to_coord(*p), 'r', 'b')?;
        }

        for p in iterate_over(board.white_queens).iter() {
            self.write_to_square(bit_to_coord(*p), 'q', 'w')?;
        }

        for p in iterate_over(board.black_queens).iter() {
            self.write_to_square(bit_to_coord(*p), 'q', 'b')?;
        }

        for p in iterate_over(board.white_king).iter() {
            self.write_to_square(bit_to_coord(*p), 'k', 'w')?;
        }

        for p in iterate_over(board.black_king).iter() {
            self.write_to_square(bit_to_coord(*p), 'k', 'b')?;
        }

        self.reset_cursor()?;

        Ok(())
    }

}