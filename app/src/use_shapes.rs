use std::collections::{BTreeMap, BTreeSet};

use editor::{Rectangle, Shape};
use math::CanvasPoint;
use yew::{classes, html, virtual_dom::VNode, Classes, Html, Reducible};

use crate::CameraState;

pub enum ShapeCatalogAction {
    UpsertShape {
        id: u32,
        position: CanvasPoint,
        width_height: CanvasPoint,
        selected: bool,
    },
    UpsertSelectedShapes {
        offset: CanvasPoint,
    },
    SelectIntersecting {
        selection_box: (CanvasPoint, CanvasPoint),
    },
    SelectAll,
    UnselectAll,
    DeleteSelected,
    DeletePrevious,
    UnselectExceptPoint(CanvasPoint),
    SaveSelectedIds,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ShapeCatalogState {
    shapes: BTreeMap<u32, Shape>,
}

impl ShapeCatalogState {
    pub fn next_id(&self) -> usize {
        self.shapes.len()
    }

    pub fn any_selected(&self) -> bool {
        self.shapes
            .iter()
            .filter(|(_, s)| match s {
                Shape::Rectangle(r) => r.selected,
            })
            .map(|(&id, _)| id)
            .next()
            .is_none()
    }

    pub fn html(&self, camera: &CameraState) -> VNode {
        let z = camera.zoom();

        let selected: Classes = format!("stroke-blue-800 stroke-w-[{z}px] fill-green-300").into();
        let unselected: Classes = "stroke-black stroke-w-1 fill-orange-300".into();

        self.shapes
            .iter()
            .map(|(k, s)| {
                let k = format!("{k}");

                match s {
                    Shape::Rectangle(r) => {
                        let path = r.path();

                        let class = if r.selected {
                            selected.clone()
                        } else {
                            unselected.clone()
                        };

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
                // new_selection is a flag to check whether there is a new selection box.
                // if a new selection box is created, first unselect all selected shapes.
                let mut new_selection = BTreeSet::new();
                let mut not_inside_any_shapes = true;
                for (shape_id, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => {
                            if r.is_inside(point) {
                                if !r.selected {
                                    r.selected = true;
                                    new_selection.insert(*shape_id);
                                }

                                not_inside_any_shapes = false;
                            }
                        }
                    }
                }

                if not_inside_any_shapes {
                    for (_, s) in shapes.iter_mut() {
                        match s {
                            Shape::Rectangle(r) => r.selected = false,
                        }
                    }
                }

                if !new_selection.is_empty() {
                    for (shape_id, s) in shapes.iter_mut() {
                        match s {
                            Shape::Rectangle(r) => r.selected = new_selection.contains(shape_id),
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
            ShapeCatalogAction::SelectAll => {
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => {
                            r.selected = true;
                        }
                    }
                }
            }
            ShapeCatalogAction::DeleteSelected => {
                shapes.retain(|_, s| match s {
                    Shape::Rectangle(r) => !r.selected,
                });
            }
            ShapeCatalogAction::DeletePrevious => {}
        }

        ShapeCatalogState { shapes }.into()
    }
}
