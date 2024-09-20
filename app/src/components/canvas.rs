use gloo::{
    events::{EventListener, EventListenerOptions},
    utils::document,
};
use web_sys::{console, wasm_bindgen::JsCast};
use yew::prelude::*;

use editor::{GuidGenerator, Tool};
use math::CanvasPoint;

use crate::{
    components::{InnerCanvas, Toolbar},
    use_pointer_down_callback, use_pointer_move_callback, use_pointer_up_callback,
    use_shapes::{ShapeCatalogAction, ShapeCatalogState},
    CameraState, CameraStateAction,
};

pub static GUID_GENERATOR: GuidGenerator = GuidGenerator::new();

#[function_component]
pub fn Canvas() -> Html {
    // global management
    // use_effect(|| {
    //     let listener = EventListener::new_with_options(
    //         &window(),
    //         "contextmenu",
    //         EventListenerOptions {
    //             passive: false,
    //             phase: gloo::events::EventListenerPhase::Bubble,
    //         },
    //         move |e| {
    //             e.prevent_default();
    //         },
    //     );

    //     move || drop(listener)
    // });

    let current_tool = use_state(|| Tool::Select);
    let camera = use_reducer(|| CameraState::default());

    let global_pointer_down = use_state(|| false);

    // hand tool
    let initial_drag = use_state(|| CanvasPoint::new(0.0, 0.0));
    let temp_canvas_position = use_state(|| CanvasPoint::new(0.0, 0.0));

    // draw tool
    let shape_catalog = use_reducer(|| ShapeCatalogState::default());
    let active_shape = use_state(|| None);

    // select tool
    let selection_box = use_state(|| None);

    let client_position: UseStateHandle<Option<(i32, i32)>> = use_state(|| None);

    use_effect({
        let camera = camera.clone();
        let current_tool = current_tool.clone();
        let shape_catalog = shape_catalog.clone();
        move || {
            let keydown_listener = EventListener::new_with_options(
                &document(),
                "keydown",
                EventListenerOptions::enable_prevent_default(),
                move |e| {
                    e.prevent_default();

                    let e = e
                        .clone()
                        .dyn_into::<KeyboardEvent>()
                        .expect("failed to cast as KeyboardEvent");

                    let offset_single_unit = 8.0;

                    let camera_state = camera.clone();

                    let event_key = e.key();

                    // arrow keys
                    if let Some((dx, dy)) = match event_key.as_str() {
                        "ArrowDown" => Some((0.0, offset_single_unit)),
                        "ArrowUp" => Some((0.0, -offset_single_unit)),
                        "ArrowLeft" => Some((-offset_single_unit, 0.0)),
                        "ArrowRight" => Some((offset_single_unit, 0.0)),
                        _ => None,
                    } {
                        current_tool.set(Tool::Hand);

                        camera.dispatch(CameraStateAction::MoveCamera {
                            temp_canvas_position: (*camera_state).canvas_position(),
                            offset: CanvasPoint::new(dx, dy),
                        })
                    }

                    if let Some(tool) = match event_key.as_str() {
                        "h" => Some(Tool::Hand),
                        "s" => Some(Tool::Select),
                        "t" => Some(Tool::Text),
                        "r" => Some(Tool::Rect),
                        // "c" => Some(Tool::Circle),
                        // "l" => Some(Tool::Line),
                        // "f" => Some(Tool::Freehand),
                        _ => None,
                    } {
                        current_tool.set(tool);
                    }

                    match event_key.as_str() {
                        "Escape" => shape_catalog.dispatch(ShapeCatalogAction::UnselectAll),
                        "a" => shape_catalog.dispatch(ShapeCatalogAction::SelectAll),
                        "z" => shape_catalog.dispatch(ShapeCatalogAction::DeletePrevious),
                        "Backspace" => shape_catalog.dispatch(ShapeCatalogAction::DeleteSelected),
                        _ => {}
                    }
                },
            );

            move || drop(keydown_listener)
        }
    });

    use_effect_with(*current_tool, {
        move |tool| {
            let document = document();
            let canvas_div = document
                .query_selector("#canvas")
                .expect("no canvas div found")
                .expect("query failed");

            match tool {
                Tool::Rect => {
                    canvas_div
                        .set_attribute("class", "cursor-crosshair")
                        .expect("failed to set");
                }
                Tool::Hand => {
                    canvas_div
                        .set_attribute("class", "cursor-grab")
                        .expect("failed to set");
                }
                Tool::Select => {
                    canvas_div
                        .set_attribute("class", "cursor-normal")
                        .expect("failed to set");
                }
                Tool::Text => {
                    canvas_div
                        .set_attribute("class", "cursor-text")
                        .expect("failed to set");
                }
                _ => todo!(),
            }
        }
    });

    let pointer_down_callback = use_pointer_down_callback(
        *current_tool,
        camera.clone(),
        initial_drag.clone(),
        temp_canvas_position.clone(),
        global_pointer_down.clone(),
        shape_catalog.clone(),
        active_shape.clone(),
        selection_box.clone(),
    );

    let pointer_move_callback = use_pointer_move_callback(
        *current_tool,
        global_pointer_down.clone(),
        *initial_drag,
        *temp_canvas_position,
        camera.clone(),
        shape_catalog.clone(),
        active_shape.clone(),
        client_position.clone(),
        selection_box.clone(),
    );

    let pointer_up_callback = use_pointer_up_callback(
        current_tool.clone(),
        camera.clone(),
        temp_canvas_position.clone(),
        global_pointer_down.clone(),
        shape_catalog.clone(),
        active_shape.clone(),
        selection_box.clone(),
    );

    use_effect_with(camera.clone(), move |camera| {
        console::log_1(&format!("camera z: {}", camera.zoom()).into());
    });

    use_effect_with(global_pointer_down.clone(), move |pointer| {
        console::log_1(&format!("pointer: {:?}", (*pointer)).into())
    });

    html! {
        <div id="canvas" class="overflow-hidden top-0 left-0 relative w-screen h-screen" onpointerdown={pointer_down_callback} onpointermove={pointer_move_callback} onpointerup={pointer_up_callback}>
            <Toolbar current_tool={current_tool} client_position={*client_position} />
            <InnerCanvas camera={camera} shapes={shape_catalog} selection_box={selection_box}/>
        </div>
    }
}
