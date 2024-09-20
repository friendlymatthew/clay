use math::CanvasPoint;

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Rectangle(Rectangle),
    Freehand(Freehand),
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
pub struct Freehand {
    pub position: Vec<CanvasPoint>,
    pub selected: bool,
}

impl Freehand {
    pub fn new(p1: CanvasPoint, selected: bool) -> Self {
        Self {
            position: vec![p1],
            selected,
        }
    }

    pub fn update(&mut self, position: CanvasPoint, selected: bool) {
        self.position.push(position);
        self.selected = selected;
    }

    pub fn paths(&self) -> impl Iterator<Item = String> + '_ {
        self.position.iter().map(|c| {
            let (x, y) = c.coord();
            format!("M {x} {y} h 10.0 v 10.0 h -10.0 Z")
        })
    }
}
