use std::error::Error;
use std::io::{self, stdout, Stdout, Write};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};
use crossterm::style::{Color::{White, DarkGrey, Red, Black, Green, Blue}, Colors, Print, SetColors};

pub struct Renderer {
    stdout: Stdout
}

impl Renderer {
    pub fn new() -> Result<(Renderer), Box<dyn Error>> {
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

        let mut r = Renderer {
            stdout
        };
        Ok(r)
    }

    pub fn clear_square(&mut self, index: (u16, u16)) -> Result<(), Box<dyn Error>> {

        let bg_colour = if (index.0 + index.1) % 2 == 0 {Green} else {Blue};
        self.stdout
            .queue(cursor::MoveTo(2*index.0, index.1))?
            .queue(style::SetBackgroundColor(bg_colour))?
            .queue(style::Print("  "))?;

        self.stdout.queue(cursor::MoveTo(0, 8))?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn write_to_square(&mut self, index: (u16, u16), piece: char, colour: char) -> Result<(), Box<dyn Error>> {

        let bg_colour = if (index.0 + index.1) % 2 == 0 {Green} else {Blue};

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
            .queue(cursor::MoveTo(2*index.0, index.1))?
            .queue(style::SetBackgroundColor(bg_colour))?
            .queue(style::Print(format!("{} ", symbol)))?;
        
        self.stdout.queue(cursor::MoveTo(0, 8))?;
        self.stdout.flush()?;

        Ok(())
    }
}