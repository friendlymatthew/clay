use gloo::utils::document;
use math::Point2D;
use yew::prelude::*;

use crate::{use_shapes::ShapeCatalogState, CameraState};

#[derive(Properties, PartialEq)]
pub struct InnerCanvasProps {
    pub camera: UseReducerHandle<CameraState>,
    pub shapes: UseReducerHandle<ShapeCatalogState>,
    pub selection_box: UseStateHandle<Option<(Point2D, Point2D)>>,
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
                "style",
                &format!(
                    "transform: scale({z}) translate({x}px, {y}px); transform-origin: center;"
                ),
            )
            .expect("failed to set element");
    });

    use_effect_with(props.selection_box.clone(), move |selection_box_state| {
        let element = document()
            .get_element_by_id("selected")
            .expect("failed to get query");

        if let Some((position, width_height)) = **selection_box_state {
            let (x, y) = position.coord();
            let (w, h) = width_height.coord();

            element
                .set_attribute(
                    "class",
                    &format!("fill-sky-100 stroke-sky-900 stroke-width-1 opacity-40"),
                )
                .expect("failed to set element");

            element
                .set_attribute("d", &format!("M {x} {y} h {w} v {h} h -{w} Z"))
                .expect("failed to set element");
        } else {
            element
                .set_attribute("class", "hidden")
                .expect("failed to set attribute");
        }
    });

    let camera_state = props.camera.clone();

    html! {
        <svg class="fixed w-screen h-screen">
            <defs>
                // defines usable content for the SVG
                <rect id="box" x="100" y="100" height="100" width="100" />
                <circle id="circle" cx="200" cy="200" r="50" />
            </defs>
            <g id="group">
                {(*props.shapes).html(&*camera_state)}
                <path id="selected" />
            </g>
        </svg>
    }
}
