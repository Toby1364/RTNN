extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::time::SystemTime;

use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

mod neuralnetwork;
mod creature;

pub struct App {
    gl: GlGraphics,

    time: f64,
    mtime: f64,
    last_time: f64,
    last_mtime: f64,
    fps: f64,
    delta: f64
}

impl App {

    fn render_update(&mut self, args: &RenderArgs) {
        use graphics::*;
        self.gl.draw(args.viewport(), |c, gl| {
            clear([0.0, 0.0, 0.0, 1.0], gl);

            let creature = creature::Creature::new(2, 2);

            rectangle(
                [1.0, 1.0, 1.0, 1.0],
                [0.0, 0.0, 50.0, 50.0],
                c.transform.trans(
                    0.0, 
                    0.0),
                gl,
            );
            

            
        });
        
        { //FPS//
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => {
                    self.time = n.as_secs() as f64;
                    self.mtime = (n.as_secs() * 1000 + n.subsec_nanos() as u64 / 1_000_000) as f64;
                }
                Err(_) => panic!("SystemTime before UNIX EPOCH!"),
            }
            if self.time == self.last_time {
                self.fps += 1.0;
            }
            else {
                println!("FPS: {}", self.fps);
                self.delta = self.fps / 10.0;
                self.fps = 0.0;
            }
            self.last_time = self.time;
            self.last_mtime = self.mtime;
        }
    
    }

    fn press(&mut self, args: &Button) {

        //let x:bool = true;
        
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => {}
                _=>{}
            }
        }
    }

    fn release(&mut self, args: &Button) {

        //let x:bool = false;
        
        if let &Button::Keyboard(key) = args {
            match key {
                Key::W => {}
                _=>{}
            }
        }
    }
}

fn main() {

    let mut window: GlutinWindow = WindowSettings::new("RTNN", [1500.0, 1000.0])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let opengl = OpenGL::V3_2;
    
    let mut app = App {
        gl: GlGraphics::new(opengl),

        time: 0.0,
        mtime: 0.0,
        last_time: 0.0,
        last_mtime: 0.0,
        fps: 60.0,
        delta: 50.0,

    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render_update(&r);
        }
        if let Some(b) = e.press_args() {
            app.press(&b);
        }

        if let Some(b) = e.release_args() {
            app.release(&b);
        }
    }
}