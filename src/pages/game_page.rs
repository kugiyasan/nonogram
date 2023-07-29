use leptos::*;

use crate::components::pixel_component::PixelComponent;
use crate::services::puzzle::Puzzle;
use crate::vecvec;

#[component]
pub fn GamePage(cx: Scope) -> impl IntoView {
    let cols_hints = vecvec!([1], [4], [4], [4], [1, 6], [4], [4], [4]);
    let rows_hints = vecvec![[1], [0], [1], [1], [6], [6], [6], [8]];
    let puzzle = Puzzle::new(cols_hints, rows_hints);

    let (puzzle, set_puzzle) = create_signal(cx, puzzle);

    let signalGrid = (1..=8).map(|idx| create_signal(cx, idx)).enumerate();

    let pixels = move || {
        {
            puzzle
                .get()
                .grid
                .enumerate()
                .map(|((x, y), &pixel)| view! {cx, <PixelComponent x y pixel set_puzzle/>})
        }
        .collect_view(cx)
    };

    view! { cx,
        <ul>{pixels}</ul>
    }
}
