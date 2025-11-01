pub mod common;
pub mod planeta_rocosso;
pub mod planeta_anillos;
pub mod planeta_verde;
pub mod anillos_vs;
pub mod luna_vs;

// nuevos
pub mod planeta_gaseoso;
pub mod planeta_rojo;

use crate::renderer::pipeline::Shader;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ShaderKind {
    Rocky,
    Gas,
    Ice,
    Moon,
    Rings,
    // nuevos
    GasBlue,  // planeta_gaseoso
    Red,      // planeta_rojo
}

pub fn make_shader(kind: ShaderKind) -> Box<dyn Shader> {
    match kind {
        ShaderKind::Rocky => Box::new(planeta_rocosso::Rocky::default()),
        ShaderKind::Gas   => Box::new(planeta_anillos::Gas::default()),
        ShaderKind::Ice   => Box::new(planeta_verde::Ice::default()),
        ShaderKind::Moon  => Box::new(luna_vs::Moon::default()),
        ShaderKind::Rings => Box::new(anillos_vs::Rings::default()),
        ShaderKind::GasBlue => Box::new(planeta_gaseoso::PlanetaGaseoso::default()),
        ShaderKind::Red     => Box::new(planeta_rojo::PlanetaRojo::default()),
    }
}

