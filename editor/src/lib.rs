mod rectangle;
mod tool;
use math::Point2D;
pub use rectangle::Rectangle;
pub use tool::Tool;

pub fn get_box(p1: Point2D, p2: Point2D) -> (Point2D, Point2D) {
    (p1.min(p2), (p2 - p1).abs())
}
