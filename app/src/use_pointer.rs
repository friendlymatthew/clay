use crate::EqRc;
use editor::{get_box, viewport_to_global, Rectangle, ShapeCatalog, Tool};
use math::{Point2D, Point3D};
use std::cell::RefCell;
use web_sys::console;
use yew::{hook, Callback, PointerEvent, UseStateHandle};

#[hook]
pub fn use_pointer_down_callback(
    current_tool: Tool,
    camera: Point3D,
    initial_drag: UseStateHandle<Point2D>,
    initial_camera: UseStateHandle<Point3D>,
    global_pointer_down: UseStateHandle<bool>,
    shape_catalog: UseStateHandle<EqRc<RefCell<ShapeCatalog>>>,
    active_shape: UseStateHandle<Option<usize>>,
) -> Callback<PointerEvent> {
    let shape_catalog = shape_catalog.clone();
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            let (x, y) = (e.client_x(), e.client_y());

            let pointer_position = Point2D::new(x as f32, y as f32);
            let global_pointer_position = viewport_to_global(pointer_position, camera);

            initial_drag.set(pointer_position);
            global_pointer_down.set(true);

            let p = viewport_to_global(pointer_position, camera);

            console::log_1(&"pointer down".into());

            match current_tool {
                Tool::Hand => initial_camera.set(camera),
                Tool::Draw => {
                    console::log_1(&"pointer draw".into());
                    shape_catalog.borrow_mut().unselect_all();
                    let rectangle = Rectangle::new(p, p);
                    let next_id = shape_catalog.borrow_mut().add_shape(rectangle);
                    active_shape.set(Some(next_id));
                    console::log_1(&"set shape".into());
                }
                Tool::Select => {}
                Tool::Text => {}
            }
        }
    })
}

#[hook]
pub fn use_pointer_move_callback(
    current_tool: Tool,
    global_pointer_down: bool,
    initial_drag: Point2D,
    initial_camera: Point3D,
    camera: UseStateHandle<Point3D>,
    shape_catalog: UseStateHandle<EqRc<RefCell<ShapeCatalog>>>,
    active_shape: UseStateHandle<Option<usize>>,
    client_position: UseStateHandle<Option<(i32, i32)>>,
) -> Callback<PointerEvent> {
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            let (x, y) = (e.client_x(), e.client_y());

            client_position.set(Some((x, y)));

            if !global_pointer_down {
                return;
            }

            console::log_1(&format!("print x, y: {}, {}", e.client_x(), e.client_y(),).into());
            let p = Point2D::new(x as f32, y as f32);

            let camera_coord = *camera;

            let p1 = viewport_to_global(initial_drag, camera_coord);
            let p2 = viewport_to_global(p, camera_coord);

            let offset = p2 - p1;

            console::log_1(&format!("{}, {}", offset.coord().0, offset.coord().1).into());

            // todo: roundabout way.
            let (x, y, _) = initial_camera.coord();
            let p = Point3D::new(x, y, camera_coord.two());

            match current_tool {
                Tool::Hand => camera.set(p.add_with_point2d(offset)),
                Tool::Draw => {
                    if let Some(active_shape) = *active_shape {
                        let (position, width_height) = get_box(p1, p2);
                        let catalog_state = shape_catalog.clone();
                        let catalog = (*catalog_state).clone();

                        catalog
                            .borrow_mut()
                            .update_shape(
                                &active_shape,
                                Some(position),
                                Some(width_height),
                                Some(false),
                            )
                            .expect("failed to update shape state");

                        shape_catalog.set(catalog);
                    }
                }
                _ => unreachable!(),
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
    shape_catalog: UseStateHandle<EqRc<RefCell<ShapeCatalog>>>,
    active_shape: UseStateHandle<Option<usize>>,
) -> Callback<PointerEvent> {
    Callback::from({
        move |e: PointerEvent| {
            e.prevent_default();

            global_pointer_down.set(false);

            match current_tool {
                Tool::Hand => initial_camera.set(camera),
                Tool::Draw => {
                    let shape_catalog_state = shape_catalog.clone();
                    let catalog_state = shape_catalog.clone();
                    let number_of_shapes = (*shape_catalog_state).borrow().catalog.len();
                    console::log_1(&format!("num: {number_of_shapes}").into());
                    active_shape.set(None);
                }
                Tool::Select => {}
                Tool::Text => {}
            }
        }
    })
}
