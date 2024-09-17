use math::Point2D;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Rectangle(Rectangle),
}

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

    pub fn is_inside(&self, global_pointer: Point2D) -> bool {
        global_pointer >= self.position && global_pointer <= self.position + self.width_height
    }

    pub fn intersects(&self, selection_box: (Point2D, Point2D)) -> bool {
        !(self.position + self.width_height < selection_box.0
            || self.position > selection_box.0 + selection_box.1)
    }
}
