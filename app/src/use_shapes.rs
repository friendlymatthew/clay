use std::collections::{BTreeMap, BTreeSet};

use editor::{Circle, Freehand, Rectangle, Shape, Tool};
use math::CanvasPoint;
use yew::{html, virtual_dom::VNode, Classes, Html, Reducible};

use crate::CameraState;

pub enum ShapeCatalogAction {
    UpsertShape {
        id: u32,
        position: CanvasPoint,
        width_height: CanvasPoint,
        selected: bool,
        current_tool: Tool,
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
                Shape::Circle(c) => c.selected,
                Shape::Freehand(f) => f.selected,
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
                            <path key={k} d={path} class={class} />
                        }
                    }
                    Shape::Freehand(f) => {
                        let line = f
                            .points
                            .windows(2)
                            .enumerate()
                            .map(|(id, p)| {

                                let p1 = p[0];
                                let p2 = p[1];

                                let midpoint = p1.midpoint(p2);

                                let (start_x, start_y) = p1.coord();
                                let (mid_x, mid_y) = midpoint.coord();
                                let (end_x, end_y) = p2.coord();


                                html! {
                                    <>
                                    <line x1={start_x.to_string()} y1={start_y.to_string()} x2={mid_x.to_string()} y2={mid_y.to_string()} stroke="black" stroke-width="4" stroke-linecap="round" />
                                    <line x1={mid_x.to_string()} y1={mid_y.to_string()} x2={end_x.to_string()} y2={end_y.to_string()} stroke="black" stroke-width="4" stroke-linecap="round" />
                                    </>
                                }
                            })
                            .collect::<Html>();

                        html! {
                            <g key={k}>
                                {line}
                            </g>
                        }
                    }
                    Shape::Circle(c) => {
                        let (x, y) = c.center.coord();
                        let r = format!("{}", c.radius);
                        let class = if c.selected {
                            selected.clone()
                        } else {
                            unselected.clone()
                        };

                        html! {
                            <circle class={class} cx={format!("{x}")} cy={format!("{y}")} r={r} />
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
                current_tool,
            } => {
                if let Some(shape) = shapes.get_mut(&id) {
                    match shape {
                        Shape::Rectangle(rectangle) => {
                            // Update the existing shape
                            rectangle.position = position;
                            rectangle.width_height = width_height;
                            rectangle.selected = selected;
                        }
                        Shape::Circle(circle) => {
                            circle.center = position;
                            circle.radius = position.euclid_dist(width_height);
                            circle.selected = selected;
                        }
                        Shape::Freehand(f) => {
                            f.points.push(position);
                            f.selected = selected;
                        }
                    }
                } else {
                    let new_shape = match current_tool {
                        Tool::Circle => {
                            let circle = Circle::new(position, 0.0, selected);

                            Shape::Circle(circle)
                        }
                        Tool::Rect => {
                            let rectangle = Rectangle::new(position, width_height, selected);
                            Shape::Rectangle(rectangle)
                        }
                        Tool::Freehand => Shape::Freehand(Freehand::new(position, selected)),
                        _ => panic!("unallowed tool"),
                    };

                    shapes.insert(id, new_shape);
                }
            }
            ShapeCatalogAction::UnselectAll => {
                // Iterate and unselect all shapes
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => r.selected = false,
                        Shape::Circle(c) => c.selected = false,
                        Shape::Freehand(f) => f.selected = false,
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
                        Shape::Circle(c) => {
                            if c.is_inside(point) {
                                if !c.selected {
                                    c.selected = true;
                                    new_selection.insert(*shape_id);
                                }

                                not_inside_any_shapes = false;
                            }
                        }
                        Shape::Freehand(_f) => {
                            // todo! implement freehand is inside
                        }
                    }
                }

                if not_inside_any_shapes {
                    for (_, s) in shapes.iter_mut() {
                        match s {
                            Shape::Rectangle(r) => r.selected = false,
                            Shape::Circle(c) => c.selected = false,
                            Shape::Freehand(f) => f.selected = false,
                        }
                    }
                }

                if !new_selection.is_empty() {
                    for (shape_id, s) in shapes.iter_mut() {
                        match s {
                            Shape::Rectangle(r) => r.selected = new_selection.contains(shape_id),
                            Shape::Circle(c) => c.selected = new_selection.contains(shape_id),
                            Shape::Freehand(f) => f.selected = new_selection.contains(shape_id),
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
                        Shape::Circle(c) => {
                            let temp_center = if let Some(tp) = c.temp_center {
                                tp
                            } else {
                                c.center
                            };

                            c.center = temp_center + offset;
                        }
                        Shape::Freehand(_f) => {}
                    }
                }
            }
            ShapeCatalogAction::SelectIntersecting { selection_box } => {
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => r.selected = r.intersects(selection_box),
                        Shape::Circle(c) => c.selected = c.intersects(selection_box),
                        Shape::Freehand(f) => f.selected = f.intersects(selection_box),
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
                        Shape::Circle(c) => {
                            if c.selected {
                                c.temp_center = Some(c.center);
                            }
                        }
                        Shape::Freehand(f) => {
                            if f.selected {
                                todo!();
                            }
                        }
                    }
                }
            }
            ShapeCatalogAction::SelectAll => {
                for (_, s) in shapes.iter_mut() {
                    match s {
                        Shape::Rectangle(r) => r.selected = true,
                        Shape::Circle(c) => c.selected = true,
                        Shape::Freehand(f) => f.selected = true,
                    }
                }
            }
            ShapeCatalogAction::DeleteSelected => {
                shapes.retain(|_, s| match s {
                    Shape::Rectangle(r) => !r.selected,
                    Shape::Circle(c) => !c.selected,
                    Shape::Freehand(f) => !f.selected,
                });
            }
            ShapeCatalogAction::DeletePrevious => {}
        }

        ShapeCatalogState { shapes }.into()
    }
}
