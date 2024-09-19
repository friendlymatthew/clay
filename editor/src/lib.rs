mod shape;
mod tool;

use math::CanvasPoint;
pub use shape::{Rectangle, Shape};
pub use tool::Tool;

pub fn get_box(p1: CanvasPoint, p2: CanvasPoint) -> (CanvasPoint, CanvasPoint) {
    (p1.min(p2), (p2 - p1).abs())
}
