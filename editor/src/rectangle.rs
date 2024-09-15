use math::Point2D;

#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub position: Point2D,
    pub width_height: Point2D,
    pub selected: bool,
    pub temp_position: Option<Point2D>,
}

impl Rectangle {
    pub fn new(p1: Point2D, p2: Point2D, selected: bool) -> Self {
        Self {
            position: p1,
            width_height: p2,
            selected,
            temp_position: None,
        }
    }
}
