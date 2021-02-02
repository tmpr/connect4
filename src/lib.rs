extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
// extern crate music;

use std::usize;

use graphics::{color::hex, types::Color};
use opengl_graphics::GlGraphics;
use piston::input::*;

// Color codes, called via `hex(const_name)` 🎨
pub const PINK: &str = "E95379";
pub const GREY: &str = "2E303E";
pub const DARK_GREY: &str = "232530";
pub const LIGHT_GREY: &str = "343747";
pub const TEAL: &str = "27D796";

/// Struct to keep score of wins. 
pub struct Score {
    pub teal: u8,
    pub pink: u8,
}

impl Score {
    pub fn new() -> Self {
        Score { teal: 0, pink: 0 }
    }

    pub fn win(&mut self, player: Stone) {
        match player {
            Stone::Pink => self.pink += 1,
            Stone::Teal => self.teal += 1,
            Stone::Neutral => (),
        }
    }
}

/// Possible action by player. ♟️
pub enum Move<'a> {
    SetStone,
    Nothing,
    Invalid(&'a str),
    Kill,
    Win(Stone),
}

/// 7 vectors of Stones which can be rendered. 
#[derive(Debug)]
pub struct Grid {
    pub grid: Vec<Vec<Stone>>,
}

impl Grid {
    /// Neutral Grid already filled. Virtually works as a background grid.
    pub fn neutral() -> Self {
        Grid {
            grid: vec![vec![Stone::Neutral; 6]; 7],
        }
    }

    /// Return empty grid: Used for player stones.
    pub fn empty() -> Self {
        Grid {
            grid: vec![Vec::new(); 7],
        }
    }

    /// Renders grid. 🖌️
    pub fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs) {
        for (x, row) in self.grid.iter_mut().enumerate() {
            for (y, stone) in row.iter_mut().enumerate() {
                stone.render(6 - x, 5 - (y as i32), gl, arg)
            }
        }
    }
}

/// Player Stone with a particular color. 
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Stone {
    Pink,
    Teal,
    Neutral,
}

impl Stone {
    /// Return Color based on Stone variant.
    pub fn color(&self) -> Color {
        match self {
            Stone::Pink => hex(PINK),
            Stone::Teal => hex(TEAL),
            Stone::Neutral => hex(DARK_GREY),
        }
    }

    /// Render Stone at some (manipulated) position. 🖌️
    pub fn render(&mut self, x: usize, y: i32, gl: &mut GlGraphics, arg: &RenderArgs) {
        let circle =
            graphics::ellipse::circle((x * 70 + 40) as f64, (y * 70 + 105) as f64, 30 as f64);

        gl.draw(arg.viewport(), |c, gl| {
            graphics::ellipse(self.color(), circle, c.transform, gl)
        });
    }
}
