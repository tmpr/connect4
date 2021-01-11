extern crate config;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate snake_game;
extern crate toml;

use snake_game::*;

use glutin_window::GlutinWindow;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

fn main() {
    let settings = WindowSettings::new("Connect 5", [500, 500]);

    let mut window: GlutinWindow = settings.exit_on_esc(true).build().unwrap();

    let mut game = Game::new(OpenGL::V3_2);

    let mut player = Stone::Pink;

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(r) = event.render_args() {
            game.render(&r);
        }

        if let Some(mouse_args) = event.mouse_cursor_args() {
            // Do something
        } 

        if let Some(press_event) = event.press_args() {         
            
            if let Button::Keyboard(key) = press_event {
                let result = match key {
                    Key::Q => break,
                    Key::D1 => game.make_move(player, 0),
                    Key::D2 => game.make_move(player, 1),
                    Key::D3 => game.make_move(player, 2),
                    Key::D4 => game.make_move(player, 3),
                    Key::D5 => game.make_move(player, 4),
                    Key::D6 => game.make_move(player, 5),
                    Key::D7 => game.make_move(player, 6),
                    Key::R => {
                        game = Game::new(OpenGL::V3_2);
                        Ok(())
                    },
                    _ => continue
                };
                match result {
                    Ok(_) => {
                        player = match player {
                            Stone::Pink => Stone::Teal,
                            Stone::Teal => Stone::Pink,
                            _ => Stone::Pink,
                        }
                    }
                    Err(msg) => println!("{}", msg),
                }
            }
        }
    }
}
