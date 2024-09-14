#![cfg(target_arch = "wasm32")]
#![feature(simd_wasm64)]

pub use point2d::Point2D;
pub use point3d::Point3D;

mod point2d;
mod point3d;
