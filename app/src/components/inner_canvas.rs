use gloo::utils::document;
use yew::prelude::*;

use crate::{use_stack::ShapeCatalogState, CameraState};

#[derive(Properties, PartialEq)]
pub struct InnerCanvasProps {
    pub camera: UseReducerHandle<CameraState>,
    pub shapes: UseReducerHandle<ShapeCatalogState>,
}

#[function_component]
pub fn InnerCanvas(props: &InnerCanvasProps) -> Html {
    use_effect_with(props.camera.clone(), move |camera| {
        let element = document()
            .get_element_by_id("group")
            .expect("failed to get query");

        let (x, y, z) = (*camera).coord();
        web_sys::console::log_1(&format!("camera coord: x: {}, y: {}, z: {}", x, y, z).into());
        web_sys::console::log_1(&format!("scale({}) translate({}, {})", z, x, y).into());

        element
            .set_attribute(
                "transform",
                &format!("scale({}) translate({}, {})", z, x, y),
            )
            .expect("failed to set transform attribute");
    });

    let camera_state = props.camera.clone();

    html! {
        <svg class="fixed w-full h-screen">
            <defs>
                // defines usable content for the SVG
                <rect id="box" x="100" y="100" height="100" width="100" />
            </defs>
            <g id="group" class="transform transform-gpu">
                {(*props.shapes).html(&*camera_state)}
            </g>
        </svg>
    }
}
