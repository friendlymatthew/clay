mod guid;
mod shape;
mod tool;

pub use guid::GuidGenerator;
use math::CanvasPoint;
pub use shape::*;
pub use tool::Tool;

pub fn get_box(p1: CanvasPoint, p2: CanvasPoint) -> (CanvasPoint, CanvasPoint) {
    (p1.min(p2), (p2 - p1).abs())
}
