use leptos::*;

use crate::services::{
    grid::Point,
    puzzle::{Pixel, Puzzle},
};

#[component]
pub fn PixelComponent(
    cx: Scope,
    x: usize,
    y: usize,
    pixel: Pixel,
    set_puzzle: WriteSignal<Puzzle>,
) -> impl IntoView {
    let on_click = move |_event| {
        let new_pixel = if pixel == Pixel::Filled {
            Pixel::Unknown
        } else {
            Pixel::Filled
        };
        set_puzzle.update(|p| p.grid.set(Point { x, y }, new_pixel));
    };
    view! {cx,
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded" on:click=on_click>
            {format!("{:?}", pixel.clone())}
        </button>
    }
}
