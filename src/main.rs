extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod vec2D;
mod bounding_box;
mod entity;
mod person;
mod town;

use vec2D::*;
use bounding_box::*;
use entity::*;
use person::*;
use town::*;

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
        for building in &self.town.buildings{
            let square = rectangle::square(0.0, 0.0, building.bounding_box.size.x as f64);
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    //.trans(x, y)
                    .trans(building.bounding_box.pos.x as f64, building.bounding_box.pos.y as f64);
                    //.rot_rad(entity.rotation);
                match building.shape_type {
                    ShapeType::Rectangle => rectangle(building.color, square, transform, gl),
                    ShapeType::Ellipse => ellipse(building.color, square, transform, gl),
                    ShapeType::Triangle => ellipse(building.color, square, transform, gl),
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
    }

    fn update(&mut self, args: &UpdateArgs) {
        for person in &mut self.town.people{
            person.update(args.dt);
        }
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
    let mut window: Window = WindowSettings::new("App", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
