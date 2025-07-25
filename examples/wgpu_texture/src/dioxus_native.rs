use color::{palette::css::WHITE, parse_color};
use dioxus::prelude::*;
use mini_dxn::use_wgpu;
use std::any::Any;

use crate::{limits, Color, DemoMessage, DemoPaintSource, FEATURES, STYLES};

pub fn launch_dx_native() {
    let config: Vec<Box<dyn Any>> = vec![Box::new(FEATURES), Box::new(limits())];
    mini_dxn::launch_cfg(app, Vec::new(), config);
}

fn app() -> Element {
    let mut show_cube = use_signal(|| true);

    let color_str = use_signal(|| String::from("red"));
    let color = use_memo(move || {
        parse_color(&color_str())
            .map(|c| c.to_alpha_color())
            .unwrap_or(WHITE)
            .split()
            .0
    });

    use_effect(move || println!("{:?}", color().components));

    rsx!(
        style { {STYLES} }
        div { id:"overlay",
            h2 { "Control Panel" },
            button {
                onclick: move |_| *show_cube.write() = !show_cube(),
                if show_cube() {
                    "Hide cube"
                } else {
                    "Show cube"
                }
            }
            br {}
            ColorControl { label: "Color:", color_str },
            p { "This overlay demonstrates that the custom WGPU content can be rendered beneath layers of HTML content" }
        }
        div { id:"underlay",
            h2 { "Underlay" },
            p { "This underlay demonstrates that the custom WGPU content can be rendered above layers and blended with the content underneath" }
        }
        header {
            h2 { "Blitz WGPU Demo" }
        }
        if show_cube() {
            SpinningCube { color }
        }
    )
}

#[component]
fn ColorControl(label: &'static str, color_str: Signal<String>) -> Element {
    rsx!(div {
        class: "color-control",
        { label },
        input {
            value: color_str(),
            oninput: move |evt| {
                *color_str.write() = evt.value()
            }
        }
    })
}

#[component]
fn SpinningCube(color: Memo<Color>) -> Element {
    // Create custom paint source and register it with the renderer
    let paint_source = DemoPaintSource::new();
    let sender = paint_source.sender();
    let paint_source_id = use_wgpu(move || paint_source);

    use_effect(move || {
        sender.send(DemoMessage::SetColor(color())).unwrap();
    });

    rsx!(
        div { id:"canvas-container",
            canvas {
                id: "demo-canvas",
                "src": paint_source_id
            }
        }
    )
}
