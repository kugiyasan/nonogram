use leptos::{ev::MouseEvent, *};

use crate::services::mouse_buttons::MouseButtons;
use crate::services::point::Point;
use crate::services::puzzle::{Pixel, Puzzle};

type LastPress = Option<(Point, Pixel)>;

#[component]
pub fn PixelComponent(
    cx: Scope,
    point: Point,
    pixel: Pixel,
    set_puzzle: WriteSignal<Puzzle>,
    last_press: ReadSignal<LastPress>,
    set_last_press: WriteSignal<LastPress>,
) -> impl IntoView {
    let is_filled = move || pixel == Pixel::Filled;

    let text = if pixel == Pixel::NonFilled { "X" } else { "" };

    let on_mouse_down = move |_event| {
        log!("on_mouse_down {:?}", point);
        let new_pixel = if pixel == Pixel::Filled {
            Pixel::Unknown
        } else {
            Pixel::Filled
        };
        set_puzzle.update(|p| p.grid.set(point, new_pixel));
        set_last_press(Some((point, new_pixel)));
    };

    let on_mouse_enter = move |event: MouseEvent| {
        let primary = MouseButtons::Primary as u16;
        if event.buttons() & primary == primary {
            if let Some((point, new_pixel)) = last_press.get() {
                set_puzzle.update(|p| p.grid.set(point, new_pixel));
            }
            log!("on_mouse_enter, primary pressed");
        }
    };

    view! {cx,
        <td>
            <button
                class="hover:bg-gray-300 rounded aspect-square w-24"
                class=("bg-black", is_filled)
                class=("hover:bg-gray-700", is_filled)
                on:mousedown=on_mouse_down
                on:mouseenter=on_mouse_enter
            >
                {text}
            </button>
        </td>
    }
}
