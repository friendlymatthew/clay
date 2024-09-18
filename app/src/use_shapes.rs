use std::collections::BTreeMap;

use editor::{Rectangle, Shape};
use math::Point2D;
use web_sys::console;
use yew::{classes, html, virtual_dom::VNode, Html, Reducible};

use crate::CameraState;

pub enum ShapeCatalogAction {
    UpsertShape {
        id: usize,
        position: Point2D,
        width_height: Point2D,
        selected: bool,
    },
    UpsertSelectedShapes {
        offset: Point2D,
    },
    SelectIntersecting {
        selection_box: (Point2D, Point2D),
    },
    UnselectAll,
    UnselectExceptPoint(Point2D),
    SaveSelectedIds,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShapeCatalogState {
    shapes: BTreeMap<usize, Shape>,
}

impl ShapeCatalogState {
    pub fn next_id(&self) -> usize {
        self.shapes.len()
    }

    pub fn selected(&self) -> Vec<usize> {
        self.shapes
            .iter()
            .filter(|(_, s)| match s {
                Shape::Rectangle(r) => r.selected,
            })
            .map(|(&id, _)| id)
            .collect::<Vec<_>>()
    }

    pub fn html(&self, camera: &CameraState) -> VNode {
        self.shapes
            .iter()
            .map(|(k, s)| {
                let k = format!("{k}");

                match s {
                    Shape::Rectangle(r) => {
                        let &Rectangle {
                            position,
                            width_height,
                            selected,
                            ..
                        } = r;

                        let (sx, sy) = position.coord();
                        let (sw, sh) = width_height.coord();

                        console::log_1(
                            &format!("RERENDER shape\nx: {}, y: {}, w: {}, h: {}", sx, sy, sw, sh)
                                .into(),
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

                        let fill = if selected {
                            "fill-green-300"
                        } else {
                            "fill-orange-300"
                        };
                        let class = classes!(stroke, stroke_w, fill);

                        html! {
                            <path key={k} d={path} class={class}/>
                        }
                    }
                }
            })
            .collect::<Html>()
    }
}

impl Default for ShapeCatalogState {
    fn default() -> Self {
        Self {
            shapes: BTreeMap::new(),
        }
    }
}

impl Reducible for ShapeCatalogState {
    type Action = ShapeCatalogAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut shapes = self.shapes.clone();
        match action {
            ShapeCatalogAction::UpsertShape {
                id,
                position,
                width_height,
                selected,
            } => {
                if let Some(shape) = shapes.get_mut(&id) {
                    match shape {
                        Shape::Rectangle(rectangle) => {
                            // Update the existing shape
                            rectangle.position = position;
                            rectangle.width_height = width_height;
                            rectangle.selected = selected;
                        }
                    }
                } else {
                    // Insert a new shape if not found
                    let rectangle = Rectangle::new(position, width_height, selected);
                    shapes.insert(id, Shape::Rectangle(rectangle));
                }
            }
            ShapeCatalogAction::UnselectAll => {
                // Iterate and unselect all shapes
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => r.selected = false,
                    }
                }
            }
            ShapeCatalogAction::UnselectExceptPoint(point) => {
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => {
                            if !r.is_inside(point) {
                                r.selected = false;
                            }
                        }
                    }
                }
            }
            ShapeCatalogAction::UpsertSelectedShapes { offset } => {
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => {
                            if r.selected {
                                let temp_position = if let Some(tp) = r.temp_position {
                                    tp
                                } else {
                                    r.position
                                };

                                r.position = temp_position + offset;
                            }
                        }
                    }
                }
            }
            ShapeCatalogAction::SelectIntersecting { selection_box } => {
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => match r.intersects(selection_box) {
                            true => r.selected = true,
                            false => r.selected = false,
                        },
                    }
                }
            }
            ShapeCatalogAction::SaveSelectedIds => {
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => {
                            if r.selected {
                                r.temp_position = Some(r.position);
                            }
                        }
                    }
                }
            }
        }

        ShapeCatalogState { shapes }.into()
    }
}
