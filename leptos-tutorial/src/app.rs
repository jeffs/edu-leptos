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

    /// Percent < 90, so as to always be well within the parent element.
    top: u32,

    /// Percent < 90, so as to always be well within the parent element.
    left: u32,
}

impl Position {
    fn advance(&mut self) {
        self.seed = next_random(self.seed);
        self.top = self.seed / 256 % 90;
        self.left = self.seed % 90;
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
        set_counters.update(|counters| {
            last_counter_id += 1;
            counters.push((last_counter_id, create_signal(0)));
        });
    };

    let remove = move |id| {
        move |_| {
            set_counters.update(|counters| counters.retain(|counter| counter.0 != id));
            set_sum.update(|sum| {
                *sum = counters()
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
                <tr>
                    <th>{id}.</th>
                    <td><button on:click=remove(id)>-</button></td>
                    <td><button on:click=increment>{counter}</button></td>
                </tr>
            }
        };

    view! {
        <table>
            <For
                each=counters
                key=|counter| counter.0
                children=render
            />
            <tr class="sum">
                <th></th>
                <td><button class="plus" on:click=append>+</button></td>
                <td>{sum}</td>
            </tr>
        </table>
    }
}

#[component]
pub fn Game() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (position, set_position) = create_signal(Position::default());

    let double_count = move || count() * 2;

    view! {
        <main class:won={move || count() >= GOAL}>
            <ProgressBar progress=count />
            <ProgressBar progress=double_count />
            <button
                class="tile"
                class:red=move || count() % 2 == 1
                style:top={move || position.with(|percent| format!("{}%", percent.top))}
                style:left={move || position.with(|percent| format!("{}%", percent.left))}
                on:click=move |_| {
                    set_count.update(|count| *count += 1);
                    set_position.update(Position::advance);
                    position.with(|position| log!("{position:?}"));
                }
            >
                "Click me: " {count}
            </button>
            <footer>{move || position.with(|r| format!("{r:?}"))}</footer>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <DynamicList />
        <Game />
    }
}
