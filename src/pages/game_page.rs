use leptos::*;

use crate::components::pixel_component::PixelComponent;
use crate::services::point::Point;
use crate::services::puzzle::Puzzle;
use crate::vecvec;

fn create_puzzle() -> Puzzle {
    let cols_hints = vecvec!([1], [4], [4], [4], [1, 6], [4], [4], [1]);
    let rows_hints = vecvec![[1], [], [1], [1], [6], [6], [6], [8]];
    // let cols_hints = vecvec!([1], [3], [1]);
    // let rows_hints = vecvec!([1], [1], [3]);
    Puzzle::new(cols_hints, rows_hints)
}

fn hint_to_string(hint: &[u32]) -> String {
    if hint.len() == 0 {
        return "0".to_string();
    }

    hint.iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

#[component]
pub fn GamePage(cx: Scope) -> impl IntoView {
    let puzzle = create_puzzle();
    let (puzzle, set_puzzle) = create_signal(cx, puzzle);
    let (last_press, set_last_press) = create_signal(cx, None);

    let cols_hints_view = move || {
        let puzzle = puzzle.get();

        puzzle
            .cols_hints()
            .iter()
            .enumerate()
            .map(|(x, hint)| {
                let is_column_done = puzzle.is_column_done(x);
                let hint = hint_to_string(&hint);
                view! { cx,
                    <th class=("bg-gray-300", is_column_done)>
                        {hint}
                    </th>
                }
            })
            .collect_view(cx)
    };

    let pixels = move || {
        let puzzle = puzzle.get();
        puzzle
            .map_rows(|(y, row)| {
                let is_row_done = puzzle.is_row_done(y);
                let row_hints_view = hint_to_string(&puzzle.rows_hints()[y]);
                let row_pixels = row
                    .iter()
                    .enumerate()
                    .map(|(x, &pixel)| {
                        let point = Point::new(x, y);
                        view! {cx,
                            <PixelComponent point pixel set_puzzle last_press set_last_press />
                        }
                    })
                    .collect_view(cx);

                view! { cx,
                    <tr>
                        <th class=("bg-gray-300", is_row_done)>
                            {row_hints_view}
                        </th>
                        {row_pixels}
                    </tr>
                }
            })
            .collect_view(cx)
    };

    // TODO use a canvas, to be able to swipe to change state of a row
    // actually buttons with mousedown and mouseover events are enough

    view! { cx,
        <table>
            <tr>
                <th class="w-24 h-24">Hints</th>
                {cols_hints_view}
            </tr>
            {pixels}
        </table>
    }
}
