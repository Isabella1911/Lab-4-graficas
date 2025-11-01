use crate::math::{Vec3, Vec4, rotation_y};
use crate::renderer::{buffers::Color, uniforms::Uniforms, pipeline::{Shader, VertexIn, VertexOut}};
use super::common::*;

#[derive(Copy, Clone, Debug)]
pub struct PlanetaRojo {
    pub base_dark: Vec3,
    pub lava_tint: Vec3,
    pub rot_speed: f32,
}
impl Default for PlanetaRojo {
    fn default() -> Self {
        Self {
            base_dark: Vec3::new(0.18, 0.05, 0.04), // rojo oscuro terroso
            lava_tint: Vec3::new(0.85, 0.25, 0.10), // rojo/naranja incandescente
            rot_speed: 0.45,
        }
    }
}

impl PlanetaRojo {
    fn color_layers(&self, p_ws: Vec3, n_ws: Vec3, view_dir: Vec3, u: &Uniforms) -> Vec3 {
        // ruido para “placas” y vetas
        let f1 = fbm_3d(p_ws * 3.0 + Vec3::new(5.0, -2.3, 1.7), 5, 2.1, 0.5, u.planet.noise_scale * 1.4);
        let f2 = fbm_3d(p_ws * 8.0 + Vec3::new(-3.1, 4.0, 2.6), 3, 2.3, 0.55, u.planet.noise_scale * 1.0);

        // mezcla base rojiza
        let base = lerp3(self.base_dark, Vec3::new(0.45, 0.12, 0.08), f1 * 0.7);

        // “fisuras” incandescentes controladas por f2
        let cracks = smoothstep(0.75, 0.95, f2);
        let hot = self.lava_tint * cracks * 0.9;
        let albedo = (base * (1.0 - cracks) + hot).clamp01();

        // iluminación
        let diff = lambert(n_ws, u.light_dir) * 0.85 + 0.15;
        let rim_k = rim(n_ws, view_dir, 1.8) * 0.25;
        let rim_tint = Vec3::new(0.95, 0.45, 0.30);

        (albedo * diff + rim_tint * rim_k).clamp01()
    }
}

// util local (por si no está pública en common)
#[inline]
fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

impl Shader for PlanetaRojo {
    fn name(&self) -> &'static str { "PlanetaRojo" }

    fn vertex(&mut self, vin: VertexIn, u: &Uniforms) -> VertexOut {
        let self_rot = rotation_y(u.time * self.rot_speed);
        let model = self_rot * u.model;

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
