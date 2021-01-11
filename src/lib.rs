extern crate config;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate toml;

use std::{collections::VecDeque, usize};

use graphics::{color::hex, types::Color};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;

pub const PINK: &str = "E95379";
pub const GREY: &str = "2E303E";
pub const DARK_GREY: &str = "232530";
pub const TEAL: &str = "27D796";

pub struct Game {
    gl: GlGraphics,
    pub grid: Grid,
    pub stones: Grid,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<VecDeque<Stone>>,
}

impl Grid {
    fn neutral() -> Self {
        Grid {
            grid: (0..=6)
                .map(|_| (0..=5).map(|_| Stone::Neutral).collect())
                .collect(),
        }
    }

    fn empty() -> Self {
        Grid {
            grid: (0..=6).map(|_| VecDeque::new()).collect(),
        }
    }

    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs) {
        for (x, row) in self.grid.iter_mut().rev().enumerate() {
            for (y, stone) in row.iter_mut().rev().enumerate() {
                stone.render(6 - x, 5 - y, gl, arg)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Stone {
    Pink,
    Teal,
    Neutral,
}

impl Stone {
    fn color(&self) -> Color {
        match self {
            Stone::Pink => hex(PINK),
            Stone::Teal => hex(TEAL),
            Stone::Neutral => hex(DARK_GREY),
        }
    }

    fn render(&mut self, x: usize, y: usize, gl: &mut GlGraphics, arg: &RenderArgs) {
        let circle =
            graphics::ellipse::circle((x * 70 + 40) as f64, (y * 70 + 105) as f64, 30 as f64);

        gl.draw(arg.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::ellipse(self.color(), circle, transform, gl)
        });
    }
}

impl Game {
    pub fn new(gl: OpenGL) -> Self {
        Game {
            gl: GlGraphics::new(gl),
            grid: Grid::neutral(),
            stones: Grid::empty(),
        }
    }

    pub fn render(&mut self, arg: &RenderArgs) {
        self.gl
            .draw(arg.viewport(), |_c, gl| graphics::clear(hex(GREY), gl));

        self.grid.render(&mut self.gl, arg);
        self.stones.render(&mut self.gl, arg);
    }

    pub fn make_move(&mut self, player: Stone, column: usize) -> Result<(), String> {
        if self.stones.grid[column].len() == 6 {
            return Err(String::from(
                "This column is full! Please choose another one.",
            ));
        }
        self.stones.grid[column].push_front(player);
        Ok(())
    }
}
