#![cfg(target_arch = "wasm32")]
#![feature(simd_wasm64)]

pub use point::{Point2D, Point3D};

mod point;
