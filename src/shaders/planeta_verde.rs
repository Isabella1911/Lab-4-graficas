use crate::math::{Vec3, Vec4, rotation_y};
use crate::renderer::{buffers::Color, uniforms::Uniforms, pipeline::{Shader, VertexIn, VertexOut}};
use super::common::*;


#[inline]
fn smoothstep(e0: f32, e1: f32, x: f32) -> f32 {
    let t = ((x - e0) / (e1 - e0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[derive(Copy, Clone, Debug)]
pub struct Ice {
    pub frost: Vec3,
    pub rot_speed: f32,
}
impl Default for Ice {
    fn default() -> Self {
        // verde menta
        Self { frost: Vec3::new(0.30, 0.85, 0.35), rot_speed: 0.35 }
    }
}

impl Ice {
    fn color_layers(&self, p_ws: Vec3, n_ws: Vec3, view_dir: Vec3, u: &Uniforms) -> Vec3 {
        // base 
        let base = Vec3::new(0.04, 0.10, 0.06);

        
        let crack = fbm_3d(
            p_ws * 4.0 + Vec3::new(7.0, 3.0, -2.0),
            5, 2.2, 0.45,
            u.planet.noise_scale * 2.0
        );
        let cracks = saturate((crack - 0.5) * 3.0);

        let frost_layer = lerp3(base, self.frost, crack * 0.9);

        let lat = p_ws.y.clamp(-1.0, 1.0);
        let bands = smoothstep(0.0, 0.35, (lat * 0.8).abs());
        let band_strength = bands * 0.5; // escalar en [0, 0.5]
        let tinted = frost_layer * (1.0 - 0.4 * band_strength);
        let layered = lerp3(frost_layer, tinted, band_strength);
        let diff = lambert(n_ws, u.light_dir);
        let rim_k = rim(n_ws, view_dir, u.planet.rim_power * 1.2) * 0.6;
        let rim_tint = Vec3::new(0.70, 0.95, 0.80);

        let col_lit = layered * (0.4 + 0.6 * diff) + rim_tint * rim_k * 0.5;
        let crack_tint = Vec3::new(0.09, 0.07, 0.05);
        (col_lit * (1.0 - cracks) + crack_tint * cracks).clamp01()
    }
}

impl Shader for Ice {
    fn name(&self) -> &'static str { "IcePlanet_Green" }

    fn vertex(&mut self, vin: VertexIn, u: &Uniforms) -> VertexOut {
        let orbit_rot = rotation_y(u.time * u.planet.rotation_speed * 0.6);
        let self_rot  = rotation_y(u.time * self.rot_speed);
        let model     = orbit_rot * u.model * self_rot;

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
