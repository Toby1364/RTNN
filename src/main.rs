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
use std::{thread, time};

mod neuralnetwork;
mod creature;

pub struct App {
    gl: GlGraphics,

    creatures: Vec<creature::Creature>,
    food: [[bool; 500]; 500],
    food_spawn: u8,

    cam_x: f64,
    cam_y: f64,

    zoom: f64,

    plus: bool,
    minus: bool,

    w: bool,
    a: bool,
    s: bool,
    d: bool,

    up: bool,
    down: bool,

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

        { //Cam//
            if self.w {
                self.cam_y += 100.0 / self.delta;
            }
            if self.s {
                self.cam_y -= 100.0 / self.delta;
            }
            if self.a {
                self.cam_x += 100.0 / self.delta;
            }
            if self.d {
                self.cam_x -= 100.0 / self.delta;
            }
            if self.plus {
                self.zoom += (0.5 / self.delta) * (self.zoom / 10.0);
            }
            if self.minus {
                self.zoom -= (0.5 / self.delta) * (self.zoom / 10.0);
            }
        }

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
                                2 => {color = [1.0, 1.0, 0.0, 1.0]}
                                3 => {color = [1.0, 0.0, 1.0, 1.0]}
                                4 => {color = [0.0, 1.0, 1.0, 1.0]}
                                5 => {color = [1.0, 0.0, 0.0, 1.0]}
                                _ => {}
                            }

                            rectangle(
                                color,
                                [self.cam_x, self.cam_y, 4.0 * self.zoom, 4.0 * self.zoom],
                                c.transform.trans(
                                    ((4.0 * x as f64) + (32.0 * cr.x)) * self.zoom, 
                                    ((4.0 * y as f64) + (32.0 * cr.y)) * self.zoom),
                                gl,
                            );

                            x += 1;
                        }
                        y += 1;
                    }
                }  
            
                let mut x = 0;
                while x < 500 {
                    let mut y = 0;
                    while y < 500 {
                        if self.food[x][y] {
                            rectangle(
                                [0.0, 1.0, 0.0, 1.0],
                                [16.0 + self.cam_x, 16.0 + self.cam_y, 8.0 * self.zoom, 8.0 * self.zoom],
                                c.transform.trans(
                                    (32.0 * x as f64) * self.zoom, 
                                    (32.0 * y as f64) * self.zoom),
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

                let mut index = 0;
                let mut x = 0;
                let mut y = 0;
                while y < 8 {
                    while x < 8 {

                        match cr.body[y][x] {
                            2 => {
                                if cr.x > 0.0 && self.food[cr.x as usize - 1][cr.y as usize] {cr.network.neurons[index].value = 1.0}
                            }
                            3 => {
                                if cr.x < 499.0 && self.food[cr.x as usize + 1][cr.y as usize] {cr.network.neurons[index].value = 1.0}
                            }
                            4 => {
                                if cr.y > 0.0 && self.food[cr.x as usize][cr.y as usize - 1] {cr.network.neurons[index].value = 1.0}
                            }
                            5 => {
                                if cr.y < 499.0 && self.food[cr.x as usize][cr.y as usize + 1] {cr.network.neurons[index].value = 1.0}
                            }
                            _ => {}
                        }

                        x += 1;
                        index += 1;
                    }
                    y += 1;
                }

                cr.network.neurons[64].value = cr.x;
                cr.network.neurons[65].value = cr.y;

                if cr.network.neurons[66].value > 0.0 && cr.x < 499.0 {cr.x += 1.0}
                if cr.network.neurons[66].value < 0.0 && cr.x > 0.0 {cr.x -= 1.0}
                if cr.network.neurons[67].value > 0.0 && cr.y < 499.0 {cr.y += 1.0}
                if cr.network.neurons[67].value < 0.0 && cr.y > 0.0 {cr.y -= 1.0}

                cr.energy -= 0.05;

                cr.network.update();
            }

            let mut i = 0;

            while i < self.creatures.len() {
                if self.creatures[i].energy >= 10.0 {
                    self.creatures[i].energy -= 10.0;
                    let mut cr = self.creatures[i].clone();
                    cr.mutate(1);
                    self.creatures.push(cr);
                }

                if self.creatures[i].energy <= 0.0 {
                    self.creatures.remove(i);
                }

                i += 1;
            }
            
            //println!("{:#?}", &self.creatures[0]);

            let mut rng = rand::thread_rng();
            let mut i = 0;
            while i < self.food_spawn {
                self.food[rng.gen_range(200..300)][rng.gen_range(200..300)] = true;
                i += 1;
            }
            //thread::sleep(time::Duration::from_millis(100));

            if self.up {
                self.up = false;
                self.food_spawn += 1;
            }
            if self.down {
                self.down = false;
                self.food_spawn -= 1;
            }
        }
        
        { //FPS//
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => {
                    self.time = n.as_secs() as f64;
                    self.mtime = (n.as_secs() * 500 + n.subsec_nanos() as u64 / 1_000_000) as f64;
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

        let x:bool = true;
        
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {self.up = x}
                Key::Down => {self.down = x}
                Key::W => {self.w = x}
                Key::A => {self.a = x}
                Key::S => {self.s = x}
                Key::D => {self.d = x}
                Key::NumPadPlus => {self.plus = x}
                Key::NumPadMinus => {self.minus = x}
                _=>{}
            }
        }
    }

    fn release(&mut self, args: &Button) {

        let x:bool = false;
        
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {self.up = x}
                Key::Down => {self.down = x}
                Key::W => {self.w = x}
                Key::A => {self.a = x}
                Key::S => {self.s = x}
                Key::D => {self.d = x}
                Key::NumPadPlus => {self.plus = x}
                Key::NumPadMinus => {self.minus = x}
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

    let mut creatures = vec![];

    let mut i = 0;
    while i < 2000 {
        creatures.push(creature::Creature::new(66, 2));
        i += 1;
    }

    for creature in &mut creatures {
        creature.mutate(200);
    }
    
    let mut app = App {
        gl: GlGraphics::new(opengl),

        creatures,
        food: [[false; 500]; 500],
        food_spawn: 30,

        cam_x: -3500.0,
        cam_y: -3500.0,

        zoom: 0.5,

        plus: false,
        minus: false,

        w: false,
        a: false,
        s: false,
        d: false,

        up:false,
        down: false,

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