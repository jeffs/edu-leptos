use leptos::{component, create_signal, view, IntoView, SignalUpdate};

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0u32);

    view! {
        <main
            style:border="2px solid red"
            style:width="600px"
            style:height="400px"

            on:click=move |_| set_count.update(|count| *count += 1)
        >
            <div
                class="mob"

                style:position="absolute"
                style:translate=move || format!("{0}px {0}px", count() * 10)

                style:border="2px solid silver"
                style:box-shadow="6px 6px 6px black"

                style:width="100px"
                style:height="100px"
            />

            <p style:position="fixed" style:bottom=0>{count}</p>
        </main>
    }
}
