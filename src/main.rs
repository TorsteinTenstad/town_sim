extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod bounding_box;
mod building;
mod config;
mod entity;
mod location_history;
mod misc_functions;
mod person;
mod town;
mod vec2D;

use entity::ShapeType;
use town::Town;

pub struct App {
    town: Town,
    gl: GlGraphics,
}

impl App {
    pub fn new(opengl: OpenGL) -> App {
        App {
            town: Town::new(),
            gl: GlGraphics::new(opengl),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(config::BACKGROUND_COLOR, gl);
        });
        for entity in self
            .town
            .buildings
            .iter()
            .map(|building| &building.entity)
            .chain(self.town.people.iter().map(|person| &person.entity))
        {
            let square = rectangle::rectangle_by_corners(
                0.0,
                0.0,
                entity.bounding_box.size.x,
                entity.bounding_box.size.y,
            );
            //let square = rectangle::square(0.0, 0.0, entity.bounding_box.size.x);
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    .trans(entity.bounding_box.pos.x, entity.bounding_box.pos.y);
                match entity.shape_type {
                    ShapeType::Rectangle => rectangle(entity.color, square, transform, gl),
                    ShapeType::Ellipse => ellipse(entity.color, square, transform, gl),
                    ShapeType::Triangle => ellipse(entity.color, square, transform, gl),
                }
            });
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.town.update(args.dt);
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("App", [2560, 1440])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap();

    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    let mut frame = 0;
    while let Some(e) = events.next(&mut window) {
        frame += 1;
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            if frame > 800 {
                app.update(&args);
            }
        }
    }
}
