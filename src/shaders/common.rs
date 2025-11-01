use crate::math::{Vec3, Vec4};
use crate::renderer::{buffers::Color};
use crate::renderer::uniforms::Uniforms;
use crate::math::fbm;

#[inline] pub fn saturate(x: f32) -> f32 { x.clamp(0.0, 1.0) }
#[inline] pub fn lerp(a: f32, b: f32, t: f32) -> f32 { a + (b - a)*t }
#[inline] pub fn lerp3(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    Vec3::new(lerp(a.x,b.x,t), lerp(a.y,b.y,t), lerp(a.z,b.z,t))
}

#[inline]
pub fn to_color(v: Vec3) -> Color {
    Color::from_f32_rgb(v.x, v.y, v.z)
}

#[inline]
pub fn lambert(n: Vec3, l: Vec3) -> f32 {
    saturate(n.normalize().dot(l.normalize()))
}

#[inline]
pub fn rim(n: Vec3, v: Vec3, power: f32) -> f32 {
    (1.0 - saturate(n.normalize().dot((-v).normalize()))).powf(power)
}

#[inline]
pub fn world_pos_nrm(pos: Vec3, nrm: Vec3, u: &Uniforms) -> (Vec3, Vec3) {
    let p_ws = (u.model * Vec4::from3(pos, 1.0)).xyz();
    let n_ws = (u.model * Vec4::from3(nrm, 0.0)).xyz().normalize();
    (p_ws, n_ws)
}

/// Gradiente por latitud usando la normal Y en espacio mundo
#[inline]
pub fn latitude(v: Vec3) -> f32 {
    // v.y en [-1,1] -> [0,1]
    (v.y * 0.5) + 0.5
}

/// FBM utilitario 
#[inline]
pub fn fbm_3d(p: Vec3, oct: i32, lac: f32, gain: f32, scale: f32) -> f32 {
    fbm(Vec3::new(p.x*scale, p.y*scale, p.z*scale), oct, lac, gain)
}