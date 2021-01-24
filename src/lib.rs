extern crate config;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate toml;
// extern crate music;

use std::usize;

use graphics::{color::hex, types::Color};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
// use music::{play_sound, Repeat};

pub const PINK: &str = "E95379";
pub const GREY: &str = "2E303E";
pub const DARK_GREY: &str = "232530";
pub const LIGHT_GREY: &str = "343747";
pub const TEAL: &str = "27D796";

pub struct Score {
    teal: u8,
    pink: u8,
}

impl Score {
    pub fn new() -> Self{
        Score {teal: 0, pink: 0}
    }
    
    pub fn win(&mut self, player: Stone) {
        match player {
            Stone::Pink => self.pink += 1,
            Stone::Teal => self.teal += 1,
            Stone::Neutral => (),
        }
    }

    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs, gc: Gly) {
        
    }

}

pub struct Game {
    gl: GlGraphics,
    pub grid: Grid,
    pub stones: Grid,
    pub foci: Vec<[f64; 4]>,
    pub score: Score
}

pub enum Move {
    SetStone,
    Pass,
    Invalid(String),
    Kill,
}

#[derive(Debug)]
pub struct Grid {
    grid: Vec<Vec<Stone>>,
}

impl Grid {
    fn neutral() -> Self {
        Grid {
            grid: vec![vec![Stone::Neutral; 6]; 7],
        }
    }

    fn empty() -> Self {
        Grid {
            grid: vec![Vec::new(); 7],
        }
    }

    fn render(&mut self, gl: &mut GlGraphics, arg: &RenderArgs) {
        for (x, row) in self.grid.iter_mut().enumerate() {
            for (y, stone) in row.iter_mut().enumerate() {
                stone.render(6 - x, 5 - (y as i32), gl, arg)
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

    fn render(&mut self, x: usize, y: i32, gl: &mut GlGraphics, arg: &RenderArgs) {
        let circle =
            graphics::ellipse::circle((x * 70 + 40) as f64, (y * 70 + 105) as f64, 30 as f64);

        gl.draw(arg.viewport(), |c, gl| {
            graphics::ellipse(self.color(), circle, c.transform, gl)
        });
    }
}

impl Game {
    pub fn new(gl: OpenGL) -> Self {
        Game {
            gl: GlGraphics::new(gl),
            grid: Grid::neutral(),
            stones: Grid::empty(),
            foci: (0..=6)
                .map(|i| (i * 70 + 6) as f64)
                .map(|x| graphics::rectangle::rectangle_by_corners(x, 0., x + 68., 1000.))
                .collect(),
            score: Score::new()
        }
    }

    pub fn render(&mut self, arg: &RenderArgs, mouse_x: f64, player: Stone) {
        self.gl
            .draw(arg.viewport(), |_, gl| graphics::clear(hex(GREY), gl));

        if let Some((x, focus)) = self
            .foci
            .iter()
            .enumerate()
            .find(|(_, rect)| rect[0] <= mouse_x && rect[0] + rect[2] >= mouse_x)
        {
            self.gl.draw(arg.viewport(), |c, gl| {
                graphics::rectangle(hex(LIGHT_GREY), focus.clone(), c.transform, gl);

                player.clone().render(x, -1, gl, arg)
            });
        }
        self.grid.render(&mut self.gl, arg);
        self.stones.render(&mut self.gl, arg);
    }

    pub fn add_stone(&mut self, player: Stone, column: usize) -> Move {
        if self.stones.grid[column].len() == 6 {
            return Move::Invalid(String::from(
                "This column is full! Please choose another one.",
            ));
        }
        self.stones.grid[column].push(player);
        // play_sound(&1, Repeat::Times(3), 0.3);
        Move::SetStone
    }
}
