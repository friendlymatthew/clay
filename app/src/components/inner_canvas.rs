use std::cell::RefCell;

use editor::{Rectangle, ShapeCatalog};
use gloo::utils::document;
use math::Point3D;
use yew::prelude::*;

use crate::EqRc;

#[derive(Properties, PartialEq)]
pub struct InnerCanvasProps {
    pub camera: UseStateHandle<Point3D>,
    pub shapes: UseStateHandle<EqRc<RefCell<ShapeCatalog>>>,
}

#[function_component]
pub fn InnerCanvas(props: &InnerCanvasProps) -> Html {
    use_effect_with(props.camera.clone(), move |camera| {
        let element = document()
            .get_element_by_id("group")
            .expect("failed to get query");

        let (x, y, z) = (*camera).coord();
        web_sys::console::log_1(&format!("x: {}, y: {}, z: {}", x, y, z).into());
        web_sys::console::log_1(&format!("scale({}) translate({}, {})", z, x, y).into());

        element
            .set_attribute(
                "transform",
                &format!("scale({}) translate({}, {})", z, x, y),
            )
            .expect("failed to set transform attribute");
    });

    let shapes = props
        .shapes
        .borrow()
        .catalog
        .iter()
        .map(|(k, s)| {
            let k = format!("{}", k);
            println!("drawing");

            let &Rectangle {
                position,
                width_height,
                selected,
                ..
            } = s;

            let (x, y) = position.coord();
            let (w, h) = width_height.coord();

            let (_, _, z) = props.camera.coord();

            let path = format!("M {} {} h {} v {} h -{} Z", x, y, w, h, w);

            let stroke = if selected { "blue" } else { "black" };
            let stroke_width = if selected {
                format!("{}", 2.0 / z)
            } else {
                "1".to_string()
            };
            let fill = "lightgray";

            let style = format!(
                "stroke: {}; stroke-width: {}; fill: {}",
                stroke, stroke_width, fill
            );

            html! {
                <path key={k} d={path} style={style} />
            }
        })
        .collect::<Html>();

    html! {
        <svg class="fixed w-full h-screen">
            <defs>
                // defines usable content for the SVG
                <rect id="box" x="100" y="100" height="100" width="100" />
                <g id="group" class="transform transform-gpu">
                    {shapes}
                </g>
            </defs>
        </svg>
    }
}
