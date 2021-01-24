extern crate connect4;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use connect4::*;

use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

fn main() {
    // Set up
    let settings = WindowSettings::new("Connect 4", [500, 500]);

    let mut window: GlutinWindow = settings.exit_on_esc(true).build().unwrap();

    let mut game = Game::new(OpenGL::V3_2, Score::new());

    let mut player = Stone::Pink;

    let mut events = Events::new(EventSettings::new());

    let mut mouse_x = -1.;


    // Event Loop
    while let Some(event) = events.next(&mut window) {

        if let Some(r) = event.render_args() {
            game.render(&r, mouse_x, player);
        }

        window.window.set_title(&format!("Pink: {} | Teal: {}", game.score.pink, game.score.teal));

        if let Some(mouse_args) = event.mouse_cursor_args() {
            mouse_x = mouse_args[0];
        }

        match event.press_args() {
            Some(button) => {
                let move_ = match button {
                    Button::Keyboard(key) => match key {
                        Key::Q => Move::Kill,
                        Key::R => {
                            game = Game::new(OpenGL::V3_2, Score::new());
                            Move::Nothing
                        }
                        _ => Move::Nothing,
                    },
                    Button::Mouse(click) => match click {
                        MouseButton::Left => {
                            match game.foci.iter().rev().enumerate().find(|(_, rect)| {
                                rect[0] <= mouse_x && rect[0] + rect[2] >= mouse_x
                            }) {
                                Some((i, _)) => game.add_stone(player, i),
                                None => Move::Nothing,
                            }
                        }
                        _ => Move::Nothing,
                    },
                    _ => Move::Nothing,
                };
                match move_ {
                    Move::Nothing => (),
                    Move::SetStone => {
                        player = match player {
                            Stone::Pink => Stone::Teal,
                            Stone::Teal => Stone::Pink,
                            _ => Stone::Pink,
                        }
                    }
                    Move::Kill => break,
                    Move::Invalid(msg) => {
                        println!("{}", msg)
                    },
                    Move::Win(player) => {
                        game.score.win(player);
                        game = Game::new(OpenGL::V3_2, game.score);
                    }
                }
            }
            _ => continue,
        }
    }
}
