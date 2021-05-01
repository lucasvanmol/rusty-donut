mod sdf;
mod geometry;
mod camera;

use std::{
    time::{SystemTime, Duration},
    sync::{mpsc},
    io::{stdout, Write},
    process,
    thread::{self, sleep}
};
use crossterm::{
    event::{self, Event},
    ExecutableCommand, QueueableCommand,
    cursor, style::{self, Colorize}, Result
};
use geometry::{Vec2D, Vec3D};
use camera::Camera;
use sdf::scene_sdf;

const SIZE_X: u16 = 75;
const SIZE_Y: u16 = 30;

pub struct RayMarcher {
    camera: Camera,
}

impl RayMarcher {
    pub fn new() -> Self {
        let camera_position = Vec3D::new(-0.5, 0.5, 3.0);
        let camera_direction = Vec3D::new(0.0, 0.0, -1.0);
        Self {
            camera: Camera::new(camera_position, camera_direction)
        }
    }

    pub fn run(&self) {
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            event::read().unwrap();
            tx.send(0).unwrap();
        });
        
        let start = SystemTime::now();
    
        let mut stdout = stdout();

        stdout.execute(cursor::Hide).expect("Error hiding cursor");

        loop {
            let now = SystemTime::now().duration_since(start);

            self.draw(now.unwrap().as_millis()).expect("Error drawing to terminal");

            sleep(Duration::from_millis(1));

            if rx.try_recv().is_ok() { 
                stdout.execute(cursor::Show).expect("Error showing cursor");
                process::exit(0) 
            }
        }
    }

    fn draw(&self, time: u128) -> Result<()>{
        let mut stdout = stdout();
    
        for y in 0..SIZE_Y {
            for x in 0..SIZE_X {
                let px = self.get_char(self.get_pixel_brightness(x, y, time));
                stdout
                    .queue(cursor::MoveTo(x,y))?
                    .queue(style::PrintStyledContent(px.magenta()))?;
            }
        }
        stdout.flush()?;
    
        Ok(())
    }

    fn get_pixel_brightness(&self, x: u16, y: u16, time: u128) -> f64 {
        let point = Vec2D::new(x as f64 / SIZE_X as f64, y as f64 / SIZE_Y as f64);

        self.get_uv_brightness(point, time)
    }
    
    
    fn get_uv_brightness(&self, uv_coord: Vec2D, time: u128) -> f64 {
        let t = (time as f64) / 1000.0;
        // lights
        let light_position = Vec3D::new(1.0, 2.0, -1.0);
    
    
        // camera
        let ray_direction = self.camera.unproject(uv_coord);
    
        // action!
        let depth = raymarch(self.camera.position, ray_direction, 10.0, t);
        let mut brightness = 0.0;
    
        if depth > 0.0 {
            let p = self.camera.position + ray_direction * depth;
            brightness = diffuse_lighting(p, estimate_normal(p, t), light_position);
            //brightness = 1.0;
        }
    
        brightness
    }

    fn get_char(&self, brightness: f64) -> char {
        const CHARS: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    
        let mut index = (brightness * CHARS.len() as f64).floor() as usize;
        if index > CHARS.len() - 1 {
            index = CHARS.len() - 1;
        }
    
        CHARS[index]
    }
}

fn raymarch(ray_origin: Vec3D, ray_direction: Vec3D, far_clip: f64, t: f64) -> f64 {
    let mut depth = 0.0;
    let epsilon = 0.001;
    const MAX_MARCH_STEPS: u64 = 1000;

    for _ in 0..MAX_MARCH_STEPS {
        // Calculate dist to closest surface
        let dist = scene_sdf(ray_origin + ray_direction * depth, t);
        
        if dist < epsilon { break; }

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