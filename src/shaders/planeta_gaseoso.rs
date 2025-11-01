use crate::math::{Vec3, Vec4, rotation_y};
use crate::renderer::{buffers::Color, uniforms::Uniforms, pipeline::{Shader, VertexIn, VertexOut}};
use super::common::*;

#[derive(Copy, Clone, Debug)]
pub struct PlanetaGaseoso {
    pub main_a: Vec3,
    pub main_b: Vec3,
    pub band_freq: f32,
}
impl Default for PlanetaGaseoso {
    fn default() -> Self {
        Self {
            // Azul vibrante: eléctrico → profundo
            main_a: Vec3::new(0.10, 0.40, 1.00),
            main_b: Vec3::new(0.00, 0.10, 0.60),
            band_freq: 7.0,
        }
    }
}

impl PlanetaGaseoso {
    #[inline]
    fn lat_from_normal(n_ws: Vec3) -> f32 { latitude(n_ws) }

    fn color_layers(&self, p_ws: Vec3, n_ws: Vec3, view_dir: Vec3, u: &Uniforms) -> Vec3 {
        // Bandas por latitud + turbulencia
        let lat = Self::lat_from_normal(n_ws);
        let phi = lat * std::f32::consts::TAU * self.band_freq;

        let turb = fbm_3d(
            p_ws + Vec3::new(1.2, 8.1, 3.4),
            5, 2.2, 0.5,
            u.planet.noise_scale * 1.5
        );

        let s = (phi + turb * 4.0).sin() * 0.5 + 0.5;

        // bandas con contraste
        let s2 = (s - 0.5) * 1.2 + 0.5;
        let mut bands = lerp3(self.main_a, self.main_b, s2);

        // “mancha” sutil
        let spot_dir  = Vec3::new(1.0, 0.0, 0.0);
        let dot_spot  = saturate(n_ws.dot(spot_dir));
        let spot      = dot_spot.powf(40.0) * 0.45;
        let spot_tint = Vec3::new(0.15, 0.55, 1.00);
        bands = bands * (1.0 - spot) + spot_tint * spot;

        // iluminación
        let diff = lambert(n_ws, u.light_dir) * 0.85 + 0.15;
        let rim_k = rim(n_ws, view_dir, 2.0) * 0.35;
        let rim_tint = Vec3::new(0.25, 0.55, 1.00);

        (bands * diff + rim_tint * rim_k).clamp01()
    }
}

impl Shader for PlanetaGaseoso {
    fn name(&self) -> &'static str { "PlanetaGaseoso_AzulVibrante" }

    fn vertex(&mut self, vin: VertexIn, u: &Uniforms) -> VertexOut {
        let rot   = rotation_y(u.time * u.planet.rotation_speed * 0.7);
        let model = rot * u.model;
        let clip   = u.proj * u.view * model * Vec4::from3(vin.pos, 1.0);
        let pos_ws = (model * Vec4::from3(vin.pos, 1.0)).xyz();
        let nrm_ws = (model * Vec4::from3(vin.nrm, 0.0)).xyz().normalize();
        VertexOut { clip_pos: clip, pos_ws, nrm_ws, uv: vin.uv }
    }

    fn fragment(&mut self, vary: &crate::renderer::raster::Varyings, u: &Uniforms) -> Color {
        let view_dir = (u.camera_pos - vary.pos_ws).normalize();
        let c = self.color_layers(vary.pos_ws, vary.nrm_ws, view_dir, u);
        to_color(c)
    }
}
