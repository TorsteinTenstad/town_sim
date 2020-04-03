extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
mod person;
use person::*;
mod entity;
use entity::*;
use entity::ShapeType;
mod bounding_box;
use bounding_box::*;

pub struct Town {
    people: Vec<Person>,
    buildings: Vec<Entity>,
}

impl Town {
    pub fn new() -> Town {
        let mut town = Town {
            people: Vec::<Person>::new(),
            buildings: Vec::<Entity>::new(),
        };
        let mut building = Entity{
            bounding_box: BoundingBox{
                x_pos: 50,
                y_pos: 50,
                x_size: 200,
                y_size: 200,
            },
            color: [0.5, 0.5, 0.5, 1.0],
            shape_type: ShapeType::Rectangle
        };
        let mut person = Person::new();
        person.wander_space = Some(building.bounding_box);
        town.people.push(person);
        town.buildings.push(building);
        town
    }
}

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
            let square = rectangle::square(0.0, 0.0, building.bounding_box.x_size as f64);
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    //.trans(x, y)
                    .trans(building.bounding_box.x_pos as f64, building.bounding_box.y_pos as f64);
                    //.rot_rad(entity.rotation);
                match building.shape_type {
                    ShapeType::Rectangle => rectangle(building.color, square, transform, gl),
                    ShapeType::Ellipse => ellipse(building.color, square, transform, gl),
                    ShapeType::Triangle => ellipse(building.color, square, transform, gl),
                }
            });
        }
        for person in &self.town.people{
            let square = rectangle::square(0.0, 0.0, person.entity.bounding_box.x_size as f64);
            self.gl.draw(args.viewport(), |c, gl| {
                let transform = c
                    .transform
                    //.trans(x, y)
                    .trans(person.entity.bounding_box.x_pos as f64, person.entity.bounding_box.y_pos as f64);
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
