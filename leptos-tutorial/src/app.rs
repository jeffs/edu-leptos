use leptos::{
    component, create_signal, logging::log, view, CollectView, IntoView, SignalUpdate, SignalWith,
};

const GOAL: i32 = 8;

fn next_random(seed: u32) -> u32 {
    // https://stackoverflow.com/a/3062783/3116635
    1103515245u32.wrapping_mul(seed).wrapping_add(12345)
}

#[derive(Debug, Default)]
struct Position {
    seed: u32,
    top: u32,
    left: u32,
}

impl Position {
    fn advance(&mut self) {
        self.seed = next_random(self.seed);
        self.top = self.seed / 256 % 90; // % of viewport height
        self.left = self.seed % 90; // % of viewport width
    }
}

/// Shows progress toward a goal.
#[component]
fn ProgressBar<F: Fn() -> i32 + 'static>(
    /// The maximum goal.
    #[prop(default = GOAL as u16)]
    max: u16,
    /// How much progress should be shown.
    progress: F,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (pos, set_pos) = create_signal(Position::default());

    let double_count = move || count() * 2;

    let buttons = (0..5)
        .map(create_signal)
        .map(|(count, set_count)| {
            view! {
                <li>
                    <button
                        on:click=move |_| set_count.update(|n| *n += 1)
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <main
            class:won={move || count() >= GOAL}
        >
            <ProgressBar progress=count />
            <ProgressBar progress=double_count />
            <button
                class:red=move || count() % 2 == 1
                style:top={move || pos.with(|r| format!("{}vh", r.top))}
                style:left={move || pos.with(|r| format!("{}vw", r.left))}
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                    set_pos.update(|p| p.advance());
                    pos.with(|p| log!("{p:?}"));
                }
            >
                "Click me: " {count}
            </button>
            <ul>{buttons}</ul>
            <footer>{move || pos.with(|r| format!("{r:?}"))}</footer>
        </main>
    }
}
