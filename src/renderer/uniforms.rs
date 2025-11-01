use crate::math::{Vec3, Mat4};

#[derive(Copy, Clone, Debug)]
pub struct PlanetParams {
    pub base_color: super::buffers::Color,
    pub band_freq: f32,
    pub noise_scale: f32,
    pub rim_power: f32,
    pub rotation_speed: f32,
    pub has_rings: bool,
    pub has_moon: bool,
}

impl Default for PlanetParams {
    fn default() -> Self {
        Self {
            base_color: super::buffers::Color::rgb(180, 180, 200),
            band_freq: 6.0,
            noise_scale: 2.0,
            rim_power: 2.0,
            rotation_speed: 0.5,
            has_rings: false,
            has_moon: false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Uniforms {
    pub time: f32,
    pub light_dir: Vec3,
    pub view: Mat4,
    pub proj: Mat4,
    pub model: Mat4,
    pub camera_pos: Vec3,
    pub planet: PlanetParams,
}

impl Default for Uniforms {
    fn default() -> Self {
        Self {
            time: 0.0,
            light_dir: Vec3::new(0.5, 0.7, 0.2).normalize(),
            view: Mat4::identity(),
            proj: Mat4::identity(),
            model: Mat4::identity(),
            camera_pos: Vec3::new(0.0, 0.0, 3.0),
            planet: PlanetParams::default(),
        }
    }
}