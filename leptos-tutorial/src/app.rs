use leptos::{component, view, IntoView};
use leptos::{prelude::*, CollectView};

#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into, default = Signal::derive(|| 42))] progress: Signal<i32>,
) -> impl IntoView {
    view! { <progress max=max value=progress></progress> }
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count() * 2;

    let html = "<p>This HTML will be injected.</p>";

    let values = vec!["hello", "world"];

    let length = 5;
    let counters = (1..=length).map(|i| create_signal(i));
    let counter_buttons = counters
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button on:click=move |_| set_count.update(|n| *n += 1)>{count}</button>
                </li>
            }
        })
        .collect_view();

    view! {
        <button
            class:red=move || count() % 2 == 1
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >

            "Click me: "
            {count}
        </button>
        <ProgressBar progress=count/>
        <ProgressBar/>
        <ProgressBar progress=Signal::derive(double_count)/>
        <div inner_html=html></div>
        <ul>{values.into_iter().map(|s| view! { <li>{s}</li> }).collect::<Vec<_>>()}</ul>
        <ul>{counter_buttons}</ul>
    }
}
