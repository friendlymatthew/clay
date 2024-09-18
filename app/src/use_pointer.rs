use crate::use_shapes::{ShapeCatalogAction, ShapeCatalogState};
use crate::{CameraState, CameraStateAction};
use editor::{get_box, Tool};
use math::Point2D;
use yew::{hook, Callback, PointerEvent, UseReducerHandle, UseStateHandle};

#[hook]
pub fn use_pointer_down_callback(
    current_tool: Tool,
    camera: UseReducerHandle<CameraState>,
    initial_drag: UseStateHandle<Point2D>,
    temp_canvas_position: UseStateHandle<Point2D>,
    global_pointer_down: UseStateHandle<bool>,
    shape_catalog: UseReducerHandle<ShapeCatalogState>,
    active_shape: UseStateHandle<Option<usize>>,
    selection_box: UseStateHandle<Option<(Point2D, Point2D)>>,
) -> Callback<PointerEvent> {
    let shape_catalog = shape_catalog.clone();
    let selection_box = selection_box.clone();
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            let (client_x, client_y) = (e.client_x(), e.client_y());
            let pointer_position = Point2D::new(client_x as f32, client_y as f32);
            let global_pointer_position = (*camera).convert_viewport_to_global(pointer_position);
            initial_drag.set(pointer_position);
            global_pointer_down.set(true);

            match current_tool {
                Tool::Hand => temp_canvas_position.set((*camera).canvas_position()),
                Tool::Rect => {
                    shape_catalog.dispatch(ShapeCatalogAction::UnselectAll);
                    let next_id = (*shape_catalog).next_id();
                    active_shape.set(Some(next_id));
                }
                Tool::Select => {
                    shape_catalog.dispatch(ShapeCatalogAction::UnselectExceptPoint(
                        global_pointer_position,
                    ));

                    if shape_catalog.selected().is_empty() {
                        selection_box.set(Some((pointer_position, Point2D::new(0.0, 0.0))));
                    }
                }
                Tool::Text => {
                    shape_catalog.dispatch(ShapeCatalogAction::UnselectAll);
                }
                _ => todo!(),
            }
        }
    })
}

#[hook]
pub fn use_pointer_move_callback(
    current_tool: Tool,
    global_pointer_down: UseStateHandle<bool>,
    initial_drag: Point2D,
    temp_canvas_position: Point2D,
    camera: UseReducerHandle<CameraState>,
    shape_catalog: UseReducerHandle<ShapeCatalogState>,
    active_shape: UseStateHandle<Option<usize>>,
    client_position: UseStateHandle<Option<(i32, i32)>>,
    selection_box: UseStateHandle<Option<(Point2D, Point2D)>>,
) -> Callback<PointerEvent> {
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            let (client_x, client_y) = (e.client_x(), e.client_y());

            client_position.set(Some((client_x, client_y)));

            match *global_pointer_down {
                false => {}
                true => {
                    let client_position = Point2D::new(client_x as f32, client_y as f32);

                    let camera_state = camera.clone();

                    let p1 = (*camera_state).convert_viewport_to_global(initial_drag);
                    let p2 = (*camera_state).convert_viewport_to_global(client_position);
                    let offset = p2 - p1;

                    let (box_position, box_width_height) = get_box(p1, p2);

                    match current_tool {
                        Tool::Hand => {
                            camera.dispatch(CameraStateAction::MoveCamera {
                                temp_canvas_position,
                                offset,
                            });
                        }
                        Tool::Rect => {
                            if let Some(id) = *active_shape {
                                shape_catalog.dispatch(ShapeCatalogAction::UpsertShape {
                                    id,
                                    position: box_position,
                                    width_height: box_width_height,
                                    selected: false,
                                });
                            } else {
                                panic!("no active shape")
                            }
                        }
                        Tool::Select => {
                            let selection_box = selection_box.clone();
                            match *selection_box {
                                Some((_, _)) => {
                                    selection_box.set(Some((box_position, box_width_height)));
                                    shape_catalog.dispatch(
                                        ShapeCatalogAction::SelectIntersecting {
                                            selection_box: ((box_position, box_width_height)),
                                        },
                                    );
                                }
                                None => {
                                    shape_catalog.dispatch(
                                        ShapeCatalogAction::UpsertSelectedShapes { offset },
                                    );
                                }
                            };
                        }
                        Tool::Text => {}
                        _ => todo!(),
                    }
                }
            }
        }
    })
}

#[hook]
pub fn use_pointer_up_callback(
    current_tool: UseStateHandle<Tool>,
    camera: UseReducerHandle<CameraState>,
    temp_canvas_position: UseStateHandle<Point2D>,
    global_pointer_down: UseStateHandle<bool>,
    shape_catalog: UseReducerHandle<ShapeCatalogState>,
    active_shape: UseStateHandle<Option<usize>>,
    selection_box: UseStateHandle<Option<(Point2D, Point2D)>>,
) -> Callback<PointerEvent> {
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            global_pointer_down.set(false);

            match *current_tool {
                Tool::Hand => temp_canvas_position.set((*camera).canvas_position()),
                Tool::Rect | Tool::Circle | Tool::Line | Tool::Freehand => {
                    active_shape.set(None);
                    current_tool.set(Tool::Hand);

                    shape_catalog.dispatch(ShapeCatalogAction::SaveSelectedIds);
                }
                Tool::Select => {
                    selection_box.set(None);
                    shape_catalog.dispatch(ShapeCatalogAction::SaveSelectedIds);
                }
                Tool::Text => {}
            }
        }
    })
}
