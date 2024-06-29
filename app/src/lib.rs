use crate::error_template::{AppError, ErrorTemplate};

use game::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod error_template;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <div class="container">
            <div class="row">
                <div class="one-half column">
                    <h1>"Conway's game"</h1>
                    <p>"Hello, World"</p>
                    <GameBoard render_state=vec![]/>
                    <button class="button-primary">"Example"</button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn GameBoard(render_state: Vec<Coordinate>) -> impl IntoView {
    let canvas_el: NodeRef<html::Canvas> = create_node_ref();
    
    view! { 
        <canvas node_ref=canvas_el>
        </canvas>
     }
}
