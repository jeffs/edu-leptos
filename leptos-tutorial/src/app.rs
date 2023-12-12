use leptos::prelude::*;
use leptos::{component, event_target_value, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
    let (input, set_input) = create_signal(String::new());

    let (answer, set_answer) = create_signal(None);

    let mut fibs: Vec<u64> = vec![0, 1];
    let handle_input = move |event| {
        let input_value = event_target_value(&event);
        let answer = input_value.parse().ok().and_then(|index: usize| {
            for _ in fibs.len()..=index.checked_add(1)? {
                let n = fibs.len();
                fibs.push(fibs[n - 2].checked_add(fibs[n - 1])?);
            }
            Some(fibs[index])
        });

        set_input(input_value.clone());
        set_answer(answer);
    };

    view! {
        <input on:input=handle_input prop:value=input autofocus/>
        "fib("
        {input}
        ") = "
        {answer}
    }
}
