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

use rand::Rng;

mod neuralnetwork;
mod creature;

pub struct App {
    gl: GlGraphics,

    creatures: Vec<creature::Creature>,
    food: [[bool; 30]; 50],

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

        { //Render//
            self.gl.draw(args.viewport(), |c, gl| {
                clear([0.0, 0.0, 0.0, 1.0], gl);

                for cr in &self.creatures {
                    let mut y = 0;
                    while y < 8 {
                        let mut x = 0;
                        while x < 8 {
                            let mut color = [0.0, 0.0, 0.0, 0.0];
                            match cr.body[y][x] {
                                1 => {color = [1.0, 1.0, 1.0, 1.0]}
                                _ => {}
                            }

                            rectangle(
                                color,
                                [0.0, 0.0, 4.0, 4.0],
                                c.transform.trans(
                                    (4.0 * x as f64) + (32.0 * cr.x), 
                                    (4.0 * y as f64) + (32.0 * cr.y)),
                                gl,
                            );

                            x += 1;
                        }
                        y += 1;
                    }
                }  
            
                let mut x = 0;
                while x < 50 {
                    let mut y = 0;
                    while y < 30 {
                        if self.food[x][y] {
                            rectangle(
                                [0.0, 1.0, 0.0, 1.0],
                                [8.0, 8.0, 8.0, 8.0],
                                c.transform.trans(
                                    32.0 * x as f64, 
                                    32.0 * y as f64),
                                gl,
                            );
                        }
                        y += 1;
                    }
                    x += 1;
                }
            });
        }

        { //Update//
            for cr in &mut self.creatures {
                if self.food[cr.x as usize][cr.y as usize] {
                    self.food[cr.x as usize][cr.y as usize] = false;
                    cr.energy += 3.0;
                }

                if cr.x > 0.0  && self.food[cr.x as usize - 1][cr.y as usize] {cr.network.neurons[0].value = 1.0}
                if cr.x < 49.0 && self.food[cr.x as usize + 1][cr.y as usize] {cr.network.neurons[1].value = 1.0}
                if cr.y > 0.0  &&self.food[cr.x as usize][cr.y as usize - 1] {cr.network.neurons[2].value = 1.0}
                if cr.y < 29.0 && self.food[cr.x as usize][cr.y as usize + 1] {cr.network.neurons[3].value = 1.0}

                if cr.network.neurons[4].value > 0.9 && cr.x < 49.0 {cr.x += 1.0}
                if cr.network.neurons[5].value > 0.9 && cr.x > 0.0 {cr.x -= 1.0}
                if cr.network.neurons[6].value > 0.9 && cr.y < 29.0 {cr.y += 1.0}
                if cr.network.neurons[7].value > 0.9 && cr.y > 0.0 {cr.y -= 1.0}
            }

            self.creatures[0].network.update();

            //println!("{:#?}", &self.creatures[0]);

            let mut rng = rand::thread_rng();
            while 0 == rng.gen_range(0..2) {
                self.food[rng.gen_range(0..50)][rng.gen_range(0..30)] = true;
            }
        }
        
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

    let mut window: GlutinWindow = WindowSettings::new("RTNN", [1600.0, 960.0])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let opengl = OpenGL::V3_2;

    let mut creatures = vec![
        creature::Creature::new(4, 4),
    ];

    let mut i = 0;
    while i < 20 {
        creatures[0].network.mutate();
        i += 1;
    }
    
    let mut app = App {
        gl: GlGraphics::new(opengl),

        creatures,
        food: [[false; 30]; 50],

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