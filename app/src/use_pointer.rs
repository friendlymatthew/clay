use crate::use_stack::{ShapeCatalogAction, ShapeCatalogState};
use editor::{get_box, viewport_to_global, Tool};
use math::{Point2D, Point3D};
use web_sys::console;
use yew::{hook, Callback, PointerEvent, UseReducerHandle, UseStateHandle};

#[hook]
pub fn use_pointer_down_callback(
    current_tool: Tool,
    camera: Point3D,
    initial_drag: UseStateHandle<Point2D>,
    initial_camera: UseStateHandle<Point3D>,
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
                Tool::Hand => initial_camera.set(camera),
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
    initial_camera: Point3D,
    camera: UseStateHandle<Point3D>,
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

                    let camera_coord = *camera;

                    let p1 = viewport_to_global(initial_drag, camera_coord);
                    let p2 = viewport_to_global(client_position, camera_coord);

                    match current_tool {
                        Tool::Hand => {
                            let (initial_x, inital_y, _) = initial_camera.coord();
                            let camera_z = camera_coord.three();

                            let new_camera = Point3D::new(initial_x, inital_y, camera_z);
                            camera.set(new_camera.add_with_point2d(p2 - p1));
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
    camera: Point3D,
    initial_camera: UseStateHandle<Point3D>,
    global_pointer_down: UseStateHandle<bool>,
    shape_catalog: UseReducerHandle<ShapeCatalogState>,
    active_shape: UseStateHandle<Option<usize>>,
) -> Callback<PointerEvent> {
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            global_pointer_down.set(false);

            match current_tool {
                Tool::Hand => initial_camera.set(camera),
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
