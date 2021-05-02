mod sdf;
mod geometry;
mod camera;

use std::{
    time::{SystemTime},
    sync::{mpsc},
    io::{stdout, Write, Stdout},
    process,
    thread::{self}
};
use crossterm::{
    event::{self, Event},
    ExecutableCommand, QueueableCommand,
    cursor, style::{self, Colorize}, Result, terminal
};
use geometry::{Vec2D, Vec3D};
use camera::Camera;
use sdf::scene_sdf;

pub mod viewport_sizes {
    pub const TINY: (u16, u16) = (25, 10);
    pub const SMALL: (u16, u16) = (50, 20);
    pub const NORMAL: (u16, u16) = (75, 30);
    pub const BIG: (u16, u16) = (100, 40);
    pub const HUGE: (u16, u16) = (150, 60);
}

const CHARSET: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const CHARSET_HD: [char; 70] = [' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$'];

const MAX_MARCH_STEPS: u64 = 100;
const FAR_CLIP: f64 = 10.0;
const MIN_DIST: f64 = 0.01;

pub struct Config {
    viewport_size: (u16, u16),
    hd: bool,
}

impl Config {
    pub fn new(viewport_size: (u16, u16), hd: bool) -> Self {
        Self { viewport_size, hd }
    }
}

pub struct RayMarcher {
    camera: Camera,
    stdout: Stdout,
    config: Config
}

impl RayMarcher {
    pub fn new(config: Config) -> Self {
        let camera_position = Vec3D::new(-0.5, 0.5, 3.0);
        let camera_direction = Vec3D::new(0.0, 0.0, -1.0);
        Self {
            camera: Camera::new(camera_position, camera_direction),
            stdout: stdout(),
            config
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            terminal::enable_raw_mode().unwrap();
            loop {
                match event::read().unwrap() {
                    Event::Key(_) => { break },
                    Event::Resize(_, _) => { break },
                    Event::Mouse(_) => {}
                }
            } 
            terminal::disable_raw_mode().unwrap();
            tx.send(0).unwrap();
        });
        
        let start = SystemTime::now();
    
        self.stdout
            .execute(cursor::MoveDown(1))?
            .execute(cursor::Hide)?;

        let (_, start_y) = if cfg!(windows) { 
            cursor::position().unwrap_or_else(|_err| { (0,0) })
        } else {
            (0,0)
        };

        if start_y == 0 {
            self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        }

        loop {
            let now = SystemTime::now().duration_since(start);

            self.draw(now.unwrap().as_millis(), start_y)?;

            if rx.try_recv().is_ok() {
                self.stdout.execute(cursor::Show)?;
                self.stdout.execute(cursor::MoveToNextLine(1))?;
                process::exit(0) 
            }
        }
    }

    fn draw(&mut self, time: u128, y_offset: u16) -> Result<()>{
        // crop viewport to top right for optimization
        let vp_cropped = (
            (self.config.viewport_size.0 as f64*0.66) as u16,
            (self.config.viewport_size.1 as f64*0.66) as u16,
        );

        for y in 0..vp_cropped.1 {
            for x in 0..vp_cropped.0 {
                let px = self.get_char(self.get_pixel_brightness(x, y, time));
                self.stdout
                    .queue(cursor::MoveTo(x,y + y_offset))?
                    .queue(style::PrintStyledContent(px.magenta()))?;
            }
        }
        self.stdout.flush()?;
    
        Ok(())
    }

    fn get_pixel_brightness(&self, x: u16, y: u16, time: u128) -> f64 {
        let uv_coord = Vec2D::new(x as f64 / self.config.viewport_size.0 as f64, y as f64 / self.config.viewport_size.1 as f64);
        let t = (time as f64) / 1000.0;

        // lights
        let light_position = Vec3D::new(1.0, 2.0, -1.0);
    
        // camera
        let ray_direction = self.camera.unproject(uv_coord);
    
        // action!
        let depth = raymarch(self.camera.position, ray_direction, FAR_CLIP, t);
        let mut brightness = 0.0;
    
        if depth > 0.0 {
            let p = self.camera.position + ray_direction * depth;
            brightness = diffuse_lighting(p, estimate_normal(p, t), light_position);
        }
    
        brightness
    }

    fn get_char(&self, brightness: f64) -> char {
        let charset_len = if self.config.hd { CHARSET_HD.len() } else { CHARSET.len() };

        let mut index = (brightness * charset_len as f64).floor() as usize;

        if index > charset_len - 1 {
            index = charset_len - 1;
        }
        
        if self.config.hd { CHARSET_HD[index] } else { CHARSET[index] }
    }
}

fn raymarch(ray_origin: Vec3D, ray_direction: Vec3D, far_clip: f64, t: f64) -> f64 {
    let mut depth = 0.0;

    for _ in 0..MAX_MARCH_STEPS {
        // Calculate dist to closest surface
        let dist = scene_sdf(ray_origin + ray_direction * depth, t);
        
        if dist < MIN_DIST { break; }

        depth += dist;

        if depth > far_clip { 
            depth = -1.0;
            break;
        }
    }

    depth
}

fn estimate_normal(p: Vec3D, t: f64) -> Vec3D {
    let e = 0.0001;

    Vec3D::new(
        scene_sdf(Vec3D::new(p.x + e, p.y, p.z), t) - scene_sdf(Vec3D::new(p.x - e, p.y, p.z), t),
        scene_sdf(Vec3D::new(p.x, p.y + e, p.z), t) - scene_sdf(Vec3D::new(p.x, p.y - e, p.z), t),
        scene_sdf(Vec3D::new(p.x, p.y, p.z + e), t) - scene_sdf(Vec3D::new(p.x, p.y, p.z - e), t),
    ).normalized()
}

fn diffuse_lighting(p: Vec3D, normal: Vec3D, light: Vec3D) -> f64 {
    let light_dir = (p - light).normalized();

    normal.dot(&light_dir)
}