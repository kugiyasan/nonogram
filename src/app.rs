use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::pixel::PixelComponent;
use crate::puzzle::Puzzle;
use crate::vecvec;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/game" view=GamePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
fn GamePage(cx: Scope) -> impl IntoView {
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

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
