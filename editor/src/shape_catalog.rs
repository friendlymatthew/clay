use std::collections::BTreeMap;

use math::Point2D;

#[derive(Debug)]
pub struct Rectangle {
    pub position: Point2D,
    pub width_height: Point2D,
    pub selected: bool,
    pub temp_position: Option<Point2D>,
}

impl Rectangle {
    pub fn new(p1: Point2D, p2: Point2D) -> Self {
        Self {
            position: p1,
            width_height: p2,
            selected: false,
            temp_position: None,
        }
    }
}

#[derive(Debug)]
pub struct ShapeCatalog {
    next_id: usize,
    pub catalog: BTreeMap<usize, Rectangle>,
}

impl ShapeCatalog {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            catalog: BTreeMap::new(),
        }
    }

    pub fn add_shape(&mut self, rectangle: Rectangle) -> usize {
        let id = self.next_id;
        self.catalog.insert(id, rectangle);
        self.next_id += 1;

        id
    }

    pub fn update_shape(
        &mut self,
        id: &usize,
        position: Option<Point2D>,
        width_height: Option<Point2D>,
        selected: Option<bool>,
    ) -> eyre::Result<()> {
        let rectangle = self.catalog.get_mut(id).expect("msg");

        if let Some(position) = position {
            rectangle.position = position;
        }

        if let Some(wh) = width_height {
            rectangle.width_height = wh;
        }

        if let Some(selected) = selected {
            rectangle.selected = selected;
        }

        eyre::Ok(())
    }

    pub fn unselect_all(&mut self) {
        for (_, rect) in &mut self.catalog {
            rect.selected = false;
        }
    }
}
