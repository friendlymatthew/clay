mod rectangle;
mod tool;
use math::{Point2D, Point3D};
pub use rectangle::Rectangle;
pub use tool::Tool;

pub fn viewport_to_global(point: Point2D, camera: Point3D) -> Point2D {
    let (x, y, z) = camera.coord();

    if z == 0.0 {
        panic!("z cannot be 0");
    }

    (point / Point2D::new(z, z)) - Point2D::new(x, y)
}

pub fn global_to_viewport(point: Point2D, camera: Point3D) -> Point2D {
    let (x, y, z) = camera.coord();

    (point + Point2D::new(x, y)) * Point2D::new(z, z)
}

pub fn get_box(p1: Point2D, p2: Point2D) -> (Point2D, Point2D) {
    (p1.min(p2), (p2 - p1).abs())
}
