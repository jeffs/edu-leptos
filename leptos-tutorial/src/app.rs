use leptos::{component, event_target_value, view, CollectView, IntoView};
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

    view! {
        <h1>"Error Handling"</h1>
        <label>
            "Type a number (or not!)" <input on:input=on_input/>
            <ErrorBoundary fallback=|errors| {
                view! {
                    "Not a number.  Errors:"
                    <ul>

                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, err)| {
                                    view! { <li>{err.to_string()}</li> }
                                })
                                .collect_view()
                        }}

                    </ul>
                }
            }>
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
