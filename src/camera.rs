use crate::geometry::{Vec3D, Vec2D};

pub struct Camera {
    pub position: Vec3D,
    pub direction: Vec3D,
    e1: Vec3D,
    e2: Vec3D,
}

impl Camera {
    pub fn new(position: Vec3D, direction: Vec3D) -> Self {
        let mut c = Self { position, direction, e1: Vec3D::new(0.,0.,0.), e2: Vec3D::new(0.,0.,0.) };
        c.update_projection_matrix();
        c
    }

    pub fn unproject(&self, uv: Vec2D) -> Vec3D {
        self.direction + self.e1 * (uv.x - 0.5) + self.e2 * (uv.y - 0.5)
    }

    pub fn update_projection_matrix(&mut self) {
        let n = self.direction.normalized();

        let coord_system: [Vec3D; 3] = [
            Vec3D::new(1.0, 0.0, 0.0),
            Vec3D::new(0.0, 1.0, 0.0),
            Vec3D::new(0.0, 0.0, 1.0),
        ];
        
        let mut e1 = n.cross(&coord_system[0]);
        let mut best_length = 0.0;
        for v in coord_system.iter() {
            let e = n.cross(&v);
            if e.length() > best_length {
                e1 = e;
                best_length = e.length();
            }
        }
        e1 = e1.normalized();

        self.e2 = e1;

        self.e1 = e1.cross(&n);
    }
}