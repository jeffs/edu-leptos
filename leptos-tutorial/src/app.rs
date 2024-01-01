use leptos::{component, create_signal, ev::KeyboardEvent, view, IntoView, SignalUpdate};

const SCALE: u32 = 10;

const DUNGEON_WIDTH: u32 = 60; // m
const DUNGEON_HEIGHT: u32 = 40; // m

const PLAYER_WIDTH: u32 = 1; // m
const PLAYER_HEIGHT: u32 = 2; // m

fn do_both<F: FnOnce(), G: FnOnce()>(f: F, g: G) {
    f();
    g();
}

#[component]
pub fn App() -> impl IntoView {
    let (x, set_x) = create_signal((DUNGEON_WIDTH - PLAYER_WIDTH) / 2);
    let (y, set_y) = create_signal((DUNGEON_HEIGHT - PLAYER_HEIGHT) / 2);

    let go_north = move || set_y.update(|y| *y -= 1);
    let go_east = move || set_x.update(|x| *x += 1);
    let go_south = move || set_y.update(|y| *y += 1);
    let go_west = move || set_x.update(|x| *x -= 1);

    let handle_keypress = move |ev: KeyboardEvent| match ev.key().as_str() {
        "h" => go_west(),
        "j" => go_south(),
        "k" => go_north(),
        "l" => go_east(),
        "y" => do_both(go_north, go_west),
        "u" => do_both(go_north, go_east),
        "b" => do_both(go_south, go_west),
        "n" => do_both(go_south, go_east),
        _ => (),
    };

    view! {
        <main
            style:border="2px solid red"
            style:width=format!("{}px", DUNGEON_WIDTH * SCALE)
            style:height=format!("{}px", DUNGEON_HEIGHT * SCALE)

            tabindex=0
            autofocus=true
            style:outline="none"
            on:keypress=handle_keypress
        >
            <div
                class="mob"

                style:position="absolute"
                style:translate=move || format!("{}px {}px", x() * SCALE, y() * SCALE)

                style:border="2px solid silver"
                style:box-shadow="2px 2px 2px black"

                style:width=format!("{}px", PLAYER_WIDTH * SCALE)
                style:height=format!("{}px", PLAYER_HEIGHT * SCALE)
            />

            <p style:position="fixed" style:bottom=0>{x} {y}</p>
        </main>
    }
}
