pub mod vec;
pub mod mat;
pub mod noise;

pub use vec::{Vec2, Vec3, Vec4};
pub use mat::{Mat4, look_at_rh, perspective_rh, viewport, rotation_y};
pub use noise::{fbm};