use leptos::{component, event_target_value, view, CollectView, Errors, IntoView};
use leptos::{prelude::*, ErrorBoundary};

#[component]
fn NumericInput() -> impl IntoView {
    let (value, set_value) = create_signal(Ok(0));

    // when input changes, try to parse a number from the input
    let on_input = move |ev| {
        let value = event_target_value(&ev);
        if value.is_empty() {
            set_value(Ok(0));
        } else {
            set_value(value.parse::<i32>());
        }
    };

    let fallback_items = |errors: RwSignal<Errors>| {
        errors()
            .into_iter()
            .map(|(_, err)| {
                view! { <li>{move || format!("{err:?}")}</li> }
            })
            .collect_view()
    };

    let fallback = move |errors: RwSignal<Errors>| {
        view! {
            <h2>"Oopsies:"</h2>
            <ul>{move || fallback_items(errors)}</ul>
        }
    };

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or not!)"
            <input on:input=on_input/>
            <ErrorBoundary fallback>
                <p>"You entered " <strong>{value}</strong></p>
            </ErrorBoundary>
        </label>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <NumericInput/>
        </main>
    }
}
