use crate::math::{Vec3, Vec4};
use crate::renderer::{buffers::Color, uniforms::Uniforms, pipeline::{Shader, VertexIn, VertexOut}};
use super::common::*;

#[derive(Copy, Clone, Debug)]
pub struct Moon {
    pub radius: f32,    // radio de órbita
    pub scale:  f32,    
}
impl Default for Moon {
    fn default() -> Self {
        Self { radius: 2.4, scale: 0.35 }
    }
}

impl Shader for Moon {
    fn name(&self) -> &'static str { "MoonShader" }

    fn vertex(&mut self, vin: VertexIn, u: &Uniforms) -> VertexOut {
        let angle  = u.time * 0.4;
        let center = (u.model * Vec4::from3(Vec3::new(0.0, 0.0, 0.0), 1.0)).xyz();
        let offset = Vec3::new(self.radius * angle.cos(), 0.45, self.radius * angle.sin());

        let p = center + Vec3::new(vin.pos.x * self.scale, vin.pos.y * self.scale, vin.pos.z * self.scale) + offset;

        let clip   = u.proj * u.view * Vec4::from3(p, 1.0);
        let pos_ws = p;
        let nrm_ws = Vec3::new(vin.nrm.x, vin.nrm.y, vin.nrm.z).normalize();

        VertexOut { clip_pos: clip, pos_ws, nrm_ws, uv: vin.uv }
    }

    fn fragment(&mut self, vary: &crate::renderer::raster::Varyings, u: &Uniforms) -> Color {
        // Luna celeste claro
        // Ruido para “mares” y parches
        let f = fbm_3d(vary.pos_ws * 0.9, 4, 2.0, 0.5, 1.2);

        // Paleta: de celeste suave → celeste brillante
        let celeste_lo = Vec3::new(0.60, 0.78, 0.90);
        let celeste_hi = Vec3::new(0.82, 0.93, 1.00);

        // Moteado sutil en cian para variedad (muy leve)
        let tint = Vec3::new(0.00, 0.07, 0.08) * 0.08;

        // Albedo final
        let mut albedo = lerp3(celeste_lo, celeste_hi, f).clamp01() + tint;

        // Iluminación: lambert + rim azulado suave
        let diff = lambert(vary.nrm_ws, u.light_dir) * 0.85 + 0.15;
        let view_dir = (u.camera_pos - vary.pos_ws).normalize();
        let rim_k = rim(vary.nrm_ws, view_dir, 2.0) * 0.28; // un pelín más visible
        let rim_tint = Vec3::new(0.78, 0.92, 1.00); // brillo celeste

        to_color((albedo * diff + rim_tint * rim_k).clamp01())
    }
}
