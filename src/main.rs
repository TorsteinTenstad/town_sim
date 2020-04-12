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
mod entity;
mod person;
mod town;
mod vec2D;
mod location_history;

use bounding_box::*;
use building::*;
use entity::*;
use person::*;
use town::*;
use vec2D::*;

pub struct App {
    town: Town,
    gl: GlGraphics, // OpenGL drawing backend.
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
        //let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear([0.0, 0.0, 0.0, 1.0], gl);
        });
        for entity in self
            .town
            .buildings
            .iter()
            .map(|building| &building.entity)
            .chain(self.town.people.iter().map(|person| &person.entity))
        {
            let square = rectangle::square(0.0, 0.0, entity.bounding_box.size.x);
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    //.trans(x, y)
                    .trans(
                        entity.bounding_box.pos.x,
                        entity.bounding_box.pos.y,
                    );
                //.rot_rad(entity.rotation);
                match entity.shape_type {
                    ShapeType::Rectangle => rectangle(entity.color, square, transform, gl),
                    ShapeType::Ellipse => ellipse(entity.color, square, transform, gl),
                    ShapeType::Triangle => ellipse(entity.color, square, transform, gl),
                }
            });
        }
        /*
        for building in &self.town.buildings {
            let square = rectangle::square(0.0, 0.0, building.entity.bounding_box.size.x as f64);
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    //.trans(x, y)
                    .trans(building.entity.bounding_box.pos.x as f64, building.entity.bounding_box.pos.y as f64);
                    //.rot_rad(entity.rotation);
                match building.entity.shape_type {
                    ShapeType::Rectangle => rectangle(building.entity.color, square, transform, gl),
                    ShapeType::Ellipse => ellipse(building.entity.color, square, transform, gl),
                    ShapeType::Triangle => ellipse(building.entity.color, square, transform, gl),
                }
            });
        }
        for person in &self.town.people{
            let square = rectangle::square(0.0, 0.0, person.entity.bounding_box.size.x as f64);
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    //.trans(x, y)
                    .trans(person.entity.bounding_box.pos.x as f64, person.entity.bounding_box.pos.y as f64);
                    //.rot_rad(entity.rotation);
                match person.entity.shape_type {
                    ShapeType::Rectangle => rectangle(person.entity.color, square, transform, gl),
                    ShapeType::Ellipse => ellipse(person.entity.color, square, transform, gl),
                    ShapeType::Triangle => ellipse(person.entity.color, square, transform, gl),
                }
            });
        }
        */
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.town.update(args.dt);
    }
}

fn main() {
    //let a = Vec2D::<f64>{x: 6.0, y: 4.0} as Vec2D::<i32>;
    //let b = Vec2D::<f64>{x: 3.0, y: 4.0};
    //let c = a - b;

    //println!("a: {:?}", a);
    //println!("b: {:?}", b);
    //println!("c: {:?}", c);
    //println!("b.mag: {:?}", b.magnitude());

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("App", [1440, 1440])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .fullscreen(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    let mut frame = 0;
    while let Some(e) = events.next(&mut window) {
        frame += 1;
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            if frame > 2000{
                app.update(&args);
            }
        }
    }
}
