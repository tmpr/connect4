extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate connect4;

use std::usize;

use graphics::color::hex;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;

// Color codes, called via `hex(const_name)`.
pub const GREY: &str = "2E303E";
pub const LIGHT_GREY: &str = "343747";

use connect4::*;


/// Game  instance which serves as the main API in the main loop.
pub struct Game {
    gl: GlGraphics,
    pub grid: Grid,
    pub stones: Grid,
    pub foci: Vec<[f64; 4]>,
    pub score: Score,
}


impl Game {
    /// Create a new empty game.
    pub fn new(gl: OpenGL, score: Score) -> Self {
        Game {
            gl: GlGraphics::new(gl),
            grid: Grid::neutral(),
            stones: Grid::empty(),
            foci: (0..=6)
                .map(|i| (i * 70 + 6) as f64)
                .map(|x| graphics::rectangle::rectangle_by_corners(x, 0., x + 68., 1000.))
                .collect(),
            score,
        }
    }

    /// Render Game and all of its fields. 🖌️
    pub fn render(&mut self, arg: &RenderArgs, mouse_x: f64, player: Stone) {
        // Render Window and Background
        self.gl
            .draw(arg.viewport(), |_, gl| graphics::clear(hex(GREY), gl));

        // Render Focus based on mouse args.
        if let Some((x, focus)) = self
            .foci
            .iter()
            .enumerate()
            .find(|(_, rect)| rect[0] <= mouse_x && rect[0] + rect[2] >= mouse_x)
        {
            self.gl.draw(arg.viewport(), |c, gl| {
                graphics::rectangle(hex(LIGHT_GREY), focus.clone(), c.transform, gl);

                // Render Preview / Focus Stone
                player.clone().render(x, -1, gl, arg)
            });
        }

        self.grid.render(&mut self.gl, arg);
        self.stones.render(&mut self.gl, arg);
    }

    /// Add stone to column.
    /// The rightmost column has index 0, the leftmost column has index 6.
    pub fn add_stone(&mut self, player: Stone, column: usize) -> Move {
        if self.stones.grid[column].len() == 6 {
            return Move::Invalid("This column is full! Please choose another one.");
        }
        self.stones.grid[column].push(player);

        let y = (self.stones.grid[column].len() - 1) as i8;
        let x = column as i8;

        let down_diag = self.check_neighbours(x, y, 1, 1, &player)
                          + self.check_neighbours(x, y, -1, -1, &player);

        let up_diag = self.check_neighbours(x, y, -1, 1, &player)
                          + self.check_neighbours(x, y, 1, -1, &player);

        let horizontal = self.check_neighbours(x, y, -1, 0, &player)
                          + self.check_neighbours(x, y, 1, 0, &player);

        let vertical = self.check_neighbours(x, y, 0, -1, &player);

        if horizontal >= 3 || vertical >= 3 || down_diag >= 3 || up_diag >= 3 {
            return Move::Win(player);
        }

        Move::SetStone
    }

    pub fn check_neighbours(&self, x: i8, y: i8, dx: i8, dy: i8, player: &Stone) -> u32 {
        let mut i = 1;
        let mut count = 0;
        // Horizontal Check
        loop {
            if (dx == -1 && x == 0) || (dy == -1 && y == 0) {
                break;
            }
            match self.stones.grid.get((x + (i * dx)) as usize) {
                Some(col) => match col.get((y + (i * dy)) as usize) {
                    Some(stone) => {
                        if stone == player {
                            count += 1;
                            i += 1;
                        } else {
                            break;
                        }
                    }
                    None => break,
                },
                None => break,
            }
        }
        return count;
    }
}
