use std::{cell::RefCell, collections::BTreeMap};

use editor::Rectangle;
use math::Point2D;
use web_sys::console;
use yew::{classes, html, virtual_dom::VNode, Html, Reducible};

use crate::{CameraState, EqRc};

pub enum ShapeCatalogAction {
    SelectShape(usize),
    UpsertShape {
        id: usize,
        position: Point2D,
        width_height: Point2D,
        selected: bool,
    },
    UnselectAll,
}

#[derive(Debug, PartialEq)]
pub struct ShapeCatalogState {
    shapes: EqRc<RefCell<BTreeMap<usize, Rectangle>>>,
}

impl ShapeCatalogState {
    pub fn next_id(&self) -> usize {
        self.shapes.borrow().len()
    }

    pub fn html(&self, camera: &CameraState) -> VNode {
        self.shapes
            .borrow()
            .iter()
            .map(|(k, s)| {
                let k = format!("{k}");

                let &Rectangle {
                    position,
                    width_height,
                    selected,
                    ..
                } = s;

                let (sx, sy) = position.coord();
                let (sw, sh) = width_height.coord();

                console::log_1(
                    &format!("RERENDER shape\nx: {}, y: {}, w: {}, h: {}", sx, sy, sw, sh).into(),
                );

                let path = format!("M {sx} {sy} h {sw} v {sh} h -{sw} Z");

                let stroke = if selected {
                    "stroke-blue-800"
                } else {
                    "stroke-black"
                };

                let z = camera.zoom();

                let stroke_w = if selected {
                    format!("stroke-width-[{}px]", 2 as f32 / z)
                } else {
                    format!("stroke-width-1")
                };

                let fill = "fill-slate-300";
                let class = classes!(stroke, stroke_w, fill);

                html! {
                    <path key={k} d={path} class={class}/>
                }
            })
            .collect::<Html>()
    }
}

impl Default for ShapeCatalogState {
    fn default() -> Self {
        Self {
            shapes: EqRc::new(RefCell::new(BTreeMap::new())),
        }
    }
}

impl Reducible for ShapeCatalogState {
    type Action = ShapeCatalogAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let shapes = self.shapes.clone();

        match action {
            ShapeCatalogAction::UpsertShape {
                id,
                position,
                width_height,
                selected,
            } => {
                let mut shapes_mut = shapes.borrow_mut(); // Mutably borrow once

                if let Some(rectangle) = shapes_mut.get_mut(&id) {
                    // Update the existing shape
                    rectangle.position = position;
                    rectangle.width_height = width_height;
                    rectangle.selected = selected;
                } else {
                    // Insert a new shape if not found
                    let rectangle = Rectangle::new(position, width_height, selected);
                    shapes_mut.insert(id, rectangle);
                }
            }
            ShapeCatalogAction::UnselectAll => {
                let mut shapes_mut = shapes.borrow_mut(); // Mutably borrow once

                // Iterate and unselect all shapes
                for (_, r) in shapes_mut.iter_mut() {
                    r.selected = false;
                }
            }
            _ => todo!(),
        }

        ShapeCatalogState { shapes }.into()
    }
}
