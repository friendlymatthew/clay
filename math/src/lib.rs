#![cfg(target_arch = "wasm32")]
#![feature(simd_wasm64)]

pub use point::CanvasPoint;

mod point;
