use math::CanvasPoint;

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rectangle {
    pub position: CanvasPoint,
    pub width_height: CanvasPoint,
    pub selected: bool,
    pub temp_position: Option<CanvasPoint>,
}

impl Rectangle {
    pub fn new(p1: CanvasPoint, p2: CanvasPoint, selected: bool) -> Self {
        Self {
            position: p1,
            width_height: p2,
            selected,
            temp_position: None,
        }
    }

    pub fn is_inside(&self, global_pointer: CanvasPoint) -> bool {
        global_pointer >= self.position && global_pointer <= self.position + self.width_height
    }

    pub fn intersects(&self, selection_box: (CanvasPoint, CanvasPoint)) -> bool {
        !((self.position + self.width_height).le_or(selection_box.0)
            || (selection_box.0 + selection_box.1).le_or(self.position))
    }

    pub fn path(&self) -> String {
        let (x, y) = self.position.coord();
        let (w, h) = self.width_height.coord();
        format!("M {x} {y} h {w} v {h} h -{w} Z")
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Circle {
    pub center: CanvasPoint,
    pub radius: f32,
    pub selected: bool,
    pub temp_center: Option<CanvasPoint>,
}

impl Circle {
    pub fn new(center: CanvasPoint, radius: f32, selected: bool) -> Self {
        Self {
            center,
            radius,
            selected,
            temp_center: None,
        }
    }

    pub fn is_inside(&self, global_pointer: CanvasPoint) -> bool {
        self.center.euclid_dist(global_pointer) <= self.radius
    }

    pub fn intersects(&self, selection_box: (CanvasPoint, CanvasPoint)) -> bool {
        todo!();
    }
}
