use crate::use_stack::{ShapeCatalogAction, ShapeCatalogState};
use crate::{CameraState, CameraStateAction};
use editor::{get_box, Tool};
use math::Point2D;
use web_sys::console;
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
) -> Callback<PointerEvent> {
    let shape_catalog = shape_catalog.clone();
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            let (x, y) = (e.client_x(), e.client_y());

            let pointer_position = Point2D::new(x as f32, y as f32);

            initial_drag.set(pointer_position);
            global_pointer_down.set(true);

            console::log_1(&"pointer down".into());

            match current_tool {
                Tool::Hand => temp_canvas_position.set((*camera).canvas_position()),
                Tool::Draw => {
                    shape_catalog.dispatch(ShapeCatalogAction::UnselectAll);
                    let next_id = (*shape_catalog).next_id();
                    active_shape.set(Some(next_id));
                }
                Tool::Select => {}
                Tool::Text => {
                    shape_catalog.dispatch(ShapeCatalogAction::UnselectAll);
                }
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

                    match current_tool {
                        Tool::Hand => {
                            let offset = p2 - p1;
                            camera.dispatch(CameraStateAction::MoveCamera {
                                temp_canvas_position,
                                offset,
                            });
                        }
                        Tool::Draw => {
                            if let Some(id) = *active_shape {
                                let (position, width_height) = get_box(p1, p2);

                                shape_catalog.dispatch(ShapeCatalogAction::UpsertShape {
                                    id,
                                    position,
                                    width_height,
                                    selected: false,
                                });
                            } else {
                                panic!("no active shape")
                            }
                        }
                        Tool::Select => {}
                        Tool::Text => {}
                    }
                }
            }
        }
    })
}

#[hook]
pub fn use_pointer_up_callback(
    current_tool: Tool,
    camera: UseReducerHandle<CameraState>,
    temp_canvas_position: UseStateHandle<Point2D>,
    global_pointer_down: UseStateHandle<bool>,
    shape_catalog: UseReducerHandle<ShapeCatalogState>,
    active_shape: UseStateHandle<Option<usize>>,
) -> Callback<PointerEvent> {
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            global_pointer_down.set(false);

            match current_tool {
                Tool::Hand => temp_canvas_position.set((*camera).canvas_position()),
                Tool::Draw => {
                    let catalog = shape_catalog.clone();

                    console::log_1(&format!("num items: {}", (*catalog).next_id()).into());

                    active_shape.set(None);
                }
                Tool::Select => {}
                Tool::Text => {}
            }
        }
    })
}
