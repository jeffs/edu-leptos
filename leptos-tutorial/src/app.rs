use leptos::{
    component, create_signal, ev::KeyboardEvent, view, For, IntoView, ReadSignal, SignalSet,
    SignalUpdate,
};

const SCALE: u32 = 10;

const DUNGEON_WIDTH: u32 = 60; // m
const DUNGEON_HEIGHT: u32 = 40; // m

const PLAYER_WIDTH: u32 = 1; // m
const PLAYER_HEIGHT: u32 = 2; // m
const PLAYER_SPEED: u32 = 1; // m/s

const WALL_WIDTH: u32 = 2; // m
const WALL_HEIGHT: u32 = 4; // m
const WALL_COUNT: usize = 20;

/// Linear Congruential Generator (LCG).  See also:
/// https://stackoverflow.com/a/3062783/3116635
fn next_random(seed: u32) -> u32 {
    1103515245u32.wrapping_mul(seed).wrapping_add(12345)
}

/// Axis-Aligned Boundary Box (AABB) collision detector.
fn is_player_touching_wall(player: (u32, u32), wall: (u32, u32)) -> bool {
    ((wall.0..wall.0 + WALL_WIDTH).contains(&player.0)
        || (wall.0 + 1..wall.0 + WALL_WIDTH).contains(&(player.0 + PLAYER_WIDTH)))
        && ((wall.1..wall.1 + WALL_HEIGHT).contains(&player.1)
            || (wall.1 + 1..wall.1 + WALL_HEIGHT).contains(&(player.1 + PLAYER_HEIGHT)))
}

#[component]
fn Wall(x: u32, y: u32) -> impl IntoView {
    view! {
        <div
           class="entity wall"
           style:translate=move || format!("{}px {}px", x * SCALE, y * SCALE)
           style:width=format!("{}px", WALL_WIDTH * SCALE)
           style:height=format!("{}px", WALL_HEIGHT * SCALE)
        />
    }
}

#[component]
fn Player(x: ReadSignal<u32>, y: ReadSignal<u32>) -> impl IntoView {
    view! {
        <div
            class="entity mob"
            style:translate=move || format!("{}px {}px", x() * SCALE, y() * SCALE)
            style:width=format!("{}px", PLAYER_WIDTH * SCALE)
            style:height=format!("{}px", PLAYER_HEIGHT * SCALE)
        />
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Signals
    //
    // TODO: Remove walls if the player bumps into them.
    let (x, set_x) = create_signal((DUNGEON_WIDTH - PLAYER_WIDTH) / 2);
    let (y, set_y) = create_signal((DUNGEON_HEIGHT - PLAYER_HEIGHT) / 2);

    let mut walls = vec![];
    let mut seed = 0;
    for _ in 0..WALL_COUNT {
        let x = next_random(seed);
        seed = next_random(x);
        walls.push((
            x % (DUNGEON_WIDTH - WALL_WIDTH),
            seed % (DUNGEON_HEIGHT - WALL_HEIGHT),
        ));
    }

    let (walls, set_walls) = create_signal(walls);

    let handle_keypress = move |ev: KeyboardEvent| {
        let (mut x, mut y) = (x(), y());

        match ev.key().as_str() {
            "h" => x -= PLAYER_SPEED,
            "j" => y += PLAYER_SPEED,
            "k" => y -= PLAYER_SPEED,
            "l" => x += PLAYER_SPEED,
            "y" => (x, y) = (x - PLAYER_SPEED, y - PLAYER_SPEED),
            "u" => (x, y) = (x + PLAYER_SPEED, y - PLAYER_SPEED),
            "b" => (x, y) = (x - PLAYER_SPEED, y + PLAYER_SPEED),
            "n" => (x, y) = (x + PLAYER_SPEED, y + PLAYER_SPEED),
            _ => (),
        }

        set_x.set(x);
        set_y.set(y);

        set_walls.update(|walls| {
            walls.retain(|&wall| !is_player_touching_wall((x, y), wall));
        });
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
            <For
                each=walls
                key=|wall| wall.clone()
                let:child
            >
                <Wall x={child.0} y={child.1}/>
            </For>

            <Player x y/>

            <p>{move || walls().len()}</p>
        </main>
    }
}
