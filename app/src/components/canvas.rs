use gloo::{
    events::EventListener,
    utils::{document, window},
};
use web_sys::console;
use yew::prelude::*;

use editor::Tool;
use math::Point2D;

use crate::{
    components::{InnerCanvas, Toolbar},
    use_pointer_down_callback, use_pointer_move_callback, use_pointer_up_callback,
    use_stack::ShapeCatalogState,
    CameraState,
};

#[function_component]
pub fn Canvas() -> Html {
    // global management
    use_effect(|| {
        let listener = EventListener::new(&window(), "contextmenu", move |e| {
            e.prevent_default();
        });

        move || drop(listener)
    });

    let current_tool = use_state(|| Tool::Hand);
    let camera = use_reducer(|| CameraState::default());

    let global_pointer_down = use_state(|| false);

    // hand tool
    let initial_drag = use_state(|| Point2D::new(0.0, 0.0));
    let temp_canvas_position = use_state(|| Point2D::new(0.0, 0.0));

    // draw tool
    let shape_catalog = use_reducer(|| ShapeCatalogState::default());

    let active_shape: UseStateHandle<Option<usize>> = use_state(|| None);

    let client_position: UseStateHandle<Option<(i32, i32)>> = use_state(|| None);

    use_effect_with(*current_tool, {
        move |tool| {
            let document = document();
            let canvas_div = document
                .query_selector("#canvas")
                .expect("no canvas div found")
                .expect("query failed");

            match tool {
                Tool::Draw => {
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
                        .set_attribute("class", "cursor-pointer")
                        .expect("failed to set");
                }
                Tool::Text => {
                    canvas_div
                        .set_attribute("class", "cursor-text")
                        .expect("failed to set");
                }
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
    );

    let pointer_up_callback = use_pointer_up_callback(
        *current_tool,
        camera.clone(),
        temp_canvas_position.clone(),
        global_pointer_down.clone(),
        shape_catalog.clone(),
        active_shape.clone(),
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
            <InnerCanvas camera={camera} shapes={shape_catalog} />
        </div>
    }
}
