use gloo::utils::document;
use math::Point3D;
use yew::prelude::*;

use crate::use_stack::ShapeCatalogState;

#[derive(Properties, PartialEq)]
pub struct InnerCanvasProps {
    pub camera: UseStateHandle<Point3D>,
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
        if z == 0.0 {
            panic!("Z can't be 0");
        }

        web_sys::console::log_1(&format!("scale({}) translate({}, {})", z, x, y).into());

        element
            .set_attribute(
                "transform",
                &format!("scale({}) translate({}, {})", z, x, y),
            )
            .expect("failed to set transform attribute");
    });

    html! {
        <svg class="fixed w-full h-screen">
            <defs>
                // defines usable content for the SVG
                <rect id="box" x="100" y="100" height="100" width="100" />
                <g id="group" class="transform transform-gpu">
                    {(*props.shapes).html(*props.camera)}
                </g>
            </defs>
        </svg>
    }
}
