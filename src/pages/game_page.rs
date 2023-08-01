use leptos::ev::MouseEvent;
use leptos::*;

use crate::components::pixel_component::PixelComponent;
use crate::services::mouse_buttons::MouseButtons;
use crate::services::point::Point;
use crate::services::puzzle::{Pixel, Puzzle};
use crate::vecvec;

fn create_puzzle() -> Puzzle {
    let cols_hints = vecvec!([1], [4], [4], [4], [1, 6], [4], [4], [4]);
    let rows_hints = vecvec![[1], [0], [1], [1], [6], [6], [6], [8]];
    // let cols_hints = vecvec!([1], [3], [1]);
    // let rows_hints = vecvec!([1], [1], [3]);
    Puzzle::new(cols_hints, rows_hints)
}

#[component]
pub fn GamePage(cx: Scope) -> impl IntoView {
    let puzzle = create_puzzle();
    let (puzzle, set_puzzle) = create_signal(cx, puzzle);

    let cols_hints_view = move || {
        puzzle
            .get()
            .cols_hints()
            .iter()
            .map(|hint| {
                let hint = hint
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join("\n");

                view! { cx,
                    <th>{hint}</th>
                }
            })
            .collect_view(cx)
    };

    let (last_press, set_last_press) = create_signal(cx, None);

    let pixels = move || {
        let puzzle = puzzle.get();
        puzzle
            .grid
            .rows()
            .enumerate()
            .map(|(y, row)| {
                let row_hints = puzzle.rows_hints()[y].clone();
                let row_pixels = row
                    .iter()
                    .enumerate()
                    .map(|(x, &pixel)| {
                        let on_mouse_down = move |_event: MouseEvent| {
                            log!("on_mouse_down x:{} y:{}", x, y);
                            let new_pixel = if pixel == Pixel::Filled {
                                Pixel::Unknown
                            } else {
                                Pixel::Filled
                            };
                            set_puzzle.update(|p| p.grid.set(Point { x, y }, new_pixel));
                            set_last_press(Some((Point::new(x, y), new_pixel)));
                        };

                        let on_mouse_enter = move |event: MouseEvent| {
                            let primary = MouseButtons::Primary as u16;
                            if event.buttons() & primary == primary {
                                if let Some((point, new_pixel)) = last_press.get() {
                                    set_puzzle.update(|p| p.grid.set(Point { x, y }, new_pixel));
                                }
                                log!("on_mouse_enter, primary pressed");
                            }
                        };

                        view! {cx,
                            <PixelComponent
                                pixel
                                on_mouse_down
                                on_mouse_enter
                            />
                        }
                    })
                    .collect_view(cx);

                view! { cx,
                    <tr>
                        <th>{row_hints}</th>
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
                <th>Column hints</th>
                {cols_hints_view}
            </tr>
            {pixels}
        </table>
    }
}
