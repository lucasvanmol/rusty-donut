use crate::geometry::{Vec2D, Vec3D};
use std::f64::consts::PI;

pub fn scene_sdf(p: Vec3D, t: f64) -> f64 {
    //sphere_sdf(p, Vec3D::new(0.0, 0.0, 0.0), 1.0)
    let rot = Vec3D::new(PI/3.5, 2.0* t, 0.0);
    torus_sdf(p, Vec2D::new(0.5, 0.25), rot)
}

#[allow(unused)]
fn sphere_sdf(p: Vec3D, origin: Vec3D, radius: f64) -> f64 {
    (p - origin).length() - radius
}

fn torus_sdf(p: Vec3D, dims: Vec2D, r: Vec3D) -> f64 {
    let rot_mat = get_rot_matrix(r);

    let p = rotate_vec(p, rot_mat).unwrap();

    let p_xz = Vec2D::new(p.x, p.z);
    let q = Vec2D::new(p_xz.length() - dims.x, p.y);
    q.length() - dims.y
}

fn get_rot_matrix(r: Vec3D) -> Vec<Vec3D> {
    let cosx = r.x.cos();
    let cosy = r.y.cos();
    let cosz = r.z.cos();
    let sinx = r.x.sin();
    let siny = r.y.sin();
    let sinz = r.z.sin();

    vec![
        Vec3D::new( cosx*cosy, cosx*siny*sinz - sinx*cosz, cosx*siny*cosz + sinx*sinz),
        Vec3D::new( sinx*cosy, sinx*siny*sinz + cosx*cosz, sinx*siny*cosz - cosx*sinz),
        Vec3D::new(     -siny,                  cosy*sinz,                  cosy*cosz),
    ]
}

fn rotate_vec<'a>(p: Vec3D, rot_mat: Vec<Vec3D>) -> Result<Vec3D, &'a str> {
    if rot_mat.len() != 3 {
        return Err("Rotation matrix is not the right size!");
    }

    Ok(Vec3D::new(
        rot_mat[0].x * p.x + rot_mat[0].y * p.y + rot_mat[0].z * p.z,
        rot_mat[1].x * p.x + rot_mat[1].y * p.y + rot_mat[1].z * p.z,
        rot_mat[2].x * p.x + rot_mat[2].y * p.y + rot_mat[2].z * p.z,
    ))
}