use leptos::mount;

use leptos::html::*;
use leptos::prelude::*;

use std::f64::consts::PI;

use wasm_bindgen::JsCast;

#[component]
pub fn CircularColorPicker() -> impl IntoView {
    let (color, set_color) = create_signal("#FF0000".to_string());
    let (position, set_position) = create_signal((150.0, 150.0));
    let (is_dragging, set_is_dragging) = create_signal(false);

    let on_mouse_down = move |_: web_sys::MouseEvent| {
        set_is_dragging.set(true);
    };

    let el: NodeRef<Div> = NodeRef::new();

    let on_mouse_move = move |event: web_sys::MouseEvent| {
        if is_dragging.get_untracked() {
            if let Some(el) = el.get() {
                let rect = el.get_bounding_client_rect();
                let x = event.client_x() as f64 - rect.left();
                let y = event.client_y() as f64 - rect.top();

                let dx = x - 150.0;
                let dy = y - 150.0;
                let distance = (dx * dx + dy * dy).sqrt().max(150.0);

                if distance <= 150.0 {
                    set_position.set((x, y));
                    let angle = dy.atan2(dx);
                    let hue = (angle.to_degrees() + 180.0) % 360.0;
                    let saturation = distance / 150.0;
                    let color = hsv_to_hex(hue, saturation, 1.0);
                    set_color.set(color);
                }
            }
        }
    };

    let on_mouse_up = move |_: web_sys::MouseEvent| {
        set_is_dragging.set(false);
    };

    view! {
        <div class="flex flex-col gap-4 items-center">
            <div
                node_ref=el
                class="relative rounded-full w-[300px] h-[300px]"
                on:mousedown=on_mouse_down
                on:mouseup=on_mouse_up
                on:mousemove=on_mouse_move
                style="cursor: crosshair; background: radial-gradient(white, transparent 70%),
                conic-gradient(from -90deg, #e43f00, #fae410, #55cc3b, #09adff, #6b0efd, #e70d86, #e43f00);
                border-radius: 50%;"
            >
                <div
                    class="absolute w-4 h-4 rounded-full border-2 border-black"
                    style=move || {
                        format!(
                            "left: {}px; top: {}px; transform: translate(-50%, -50%);",
                            position.get().0,
                            position.get().1,
                        )
                    }
                ></div>
            </div>
            <div>
                Selected Color:
                <span
                    class="inline-block py-2 px-4 rounded"
                    style=move || format!("background-color: {};", color.get())
                >
                    {color}
                </span>
            </div>
        </div>
    }
}

fn hsv_to_hex(h: f64, s: f64, v: f64) -> String {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u8;
    let g = ((g + m) * 255.0) as u8;
    let b = ((b + m) * 255.0) as u8;

    format!("#{:02X}{:02X}{:02X}", r, g, b)
}
// #[component]
// pub fn CircularColorPicker(
//     selected_color: ReadSignal<String>,
//     set_selected_color: WriteSignal<String>,
//     brightness: ReadSignal<f64>,
//     set_brightness: WriteSignal<f64>,
// ) -> impl IntoView {
//     // Define the base HSL colors for the color picker
//     let base_colors = vec![
//         (0, 100, 50),   // Red
//         (60, 100, 50),  // Yellow
//         (120, 100, 50), // Green
//         (180, 100, 50), // Cyan
//         (240, 100, 50), // Blue
//         (300, 100, 50), // Magenta
//     ];

//     // Calculate the angle for each color
//     let angle_step = 360.0 / base_colors.len() as f64;

//     view! {
//         <div class="flex flex-col gap-4 items-center">
//             <div class="overflow-hidden relative w-48 h-48 rounded-full">
//                 {base_colors
//                     .into_iter()
//                     .enumerate()
//                     .map(|(index, (h, s, l))| {
//                         let angle = angle_step * index as f64;
//                         let style = format!(
//                             "transform: rotate({}deg) translate(6rem) rotate(-{}deg); background-color: hsl({}, {}%, {}%);",
//                             angle,
//                             angle,
//                             h,
//                             s,
//                             l,
//                         );

//                         view! {
//                             <div
//                                 class="absolute top-0 left-1/2 w-24 h-24 origin-bottom cursor-pointer"
//                                 style=style
//                                 on:click=move |_| {
//                                     let adjusted_l = (l as f64 * brightness.get_untracked()) as u8;
//                                     set_selected_color
//                                         .set(format!("hsl({}, {}%, {}%)", h, s, adjusted_l));
//                                 }
//                             ></div>
//                         }
//                     })
//                     .collect::<Vec<_>>()}
//                 <div
//                     class="absolute top-1/2 left-1/2 w-12 h-12 rounded-full border-2 border-gray-800 transform -translate-x-1/2 -translate-y-1/2"
//                     style=format!("background-color: {}", selected_color.get())
//                 ></div>
//             </div>

//             // Brightness Slider
//             <div class="w-full max-w-xs">
//                 <label class="block text-sm font-medium text-gray-700">"Brightness"</label>
//                 <input
//                     type="range"
//                     min="0"
//                     max="1"
//                     step="0.01"
//                     prop:value=brightness
//                     on:input=move |ev| {
//                         let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
//                         set_brightness.set(value);
//                         let (h, s, _) = parse_hsl(&selected_color.get_untracked());
//                         let l = (50.0 * value) as u8;
//                         set_selected_color.set(format!("hsl({}, {}%, {}%)", h, s, l));
//                     }
//                     class="w-full h-2 bg-gray-200 rounded-lg appearance-none cursor-pointer"
//                 />
//             </div>
//         </div>
//     }
// }

// // Helper function to parse HSL color
// fn parse_hsl(hsl: &str) -> (u16, u8, u8) {
//     let stripped = hsl
//         .strip_prefix("hsl(")
//         .unwrap_or(hsl)
//         .strip_suffix(')')
//         .unwrap_or(hsl);
//     let parts: Vec<&str> = stripped
//         .split(',')
//         .map(|s| s.trim().trim_end_matches('%'))
//         .collect();
//     let h = parts[0].parse::<u16>().unwrap_or(0);
//     let s = parts[1].parse::<u8>().unwrap_or(100);
//     let l = parts[2].parse::<u8>().unwrap_or(50);
//     (h, s, l)
// }

fn main() {
    console_error_panic_hook::set_once();
    mount::mount_to_body(CircularColorPicker);
}
