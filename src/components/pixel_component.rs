use leptos::{ev::MouseEvent, *};

use crate::services::puzzle::Pixel;

#[component]
pub fn PixelComponent<F, G>(
    cx: Scope,
    pixel: Pixel,
    on_mouse_down: F,
    on_mouse_enter: G,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
    G: Fn(MouseEvent) + 'static,
{
    let is_filled = move || pixel == Pixel::Filled;

    view! {cx,
        <td>
            <button
                class="hover:bg-gray-300 rounded aspect-square w-24"
                class=("bg-black", is_filled)
                class=("hover:bg-gray-700", is_filled)
                on:mousedown=on_mouse_down
                on:mouseenter=on_mouse_enter
            >
            </button>
        </td>
    }
}
