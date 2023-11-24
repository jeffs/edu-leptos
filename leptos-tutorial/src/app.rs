use leptos::{
    component, create_signal, logging::log, view, For, IntoView, ReadSignal, SignalUpdate,
    SignalWith, WriteSignal,
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
fn DynamicList() -> impl IntoView {
    let initial_counters: Vec<_> = (1..=5).map(|id| (id, create_signal(0))).collect();
    let mut last_counter_id = initial_counters.len();
    let (counters, set_counters) = create_signal(initial_counters);
    let (sum, set_sum) = create_signal(0);

    let append = move |_| {
        set_counters.update(|v| {
            last_counter_id += 1;
            v.push((last_counter_id, create_signal(0)));
        });
    };

    let remove = move |id| {
        move |_| {
            set_counters.update(|counters_| counters_.retain(|(id_, _)| *id_ != id));
            set_sum.update(|sum_| {
                *sum_ = counters()
                    .into_iter()
                    .map(|(_, (counter, _))| counter())
                    .sum();
            });
        }
    };

    let render =
        move |(id, (counter, set_counter)): (usize, (ReadSignal<i32>, WriteSignal<i32>))| {
            let increment = move |_| {
                set_counter.update(|n| *n += 1);
                set_sum.update(|sum_| *sum_ += 1);
            };
            view! {
                <li value={id}>
                    <button on:click=remove(id)>-</button>
                    <button on:click=increment>{counter}</button>
                </li>
            }
        };

    view! {
        <button class="plus" on:click=append>+</button>
        <ol>
            <For
                each=counters
                key=|counter| counter.0
                children=render
            />
        </ol>
        {sum}
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (pos, set_pos) = create_signal(Position::default());

    let double_count = move || count() * 2;

    view! {
        <main class:won={move || count() >= GOAL}>
            <ProgressBar progress=count />
            <ProgressBar progress=double_count />
            <button
                class="tile"
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
            <DynamicList />
            <footer>{move || pos.with(|r| format!("{r:?}"))}</footer>
        </main>
    }
}
