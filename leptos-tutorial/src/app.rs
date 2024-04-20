use leptos::{
    component, create_signal, ev::KeyboardEvent, view, For, IntoView, ReadSignal, SignalSet,
    SignalUpdate, WriteSignal,
};

const SCALE: u32 = 10;

const DUNGEON_WIDTH: u32 = 60; // m
const DUNGEON_HEIGHT: u32 = 40; // m

const PLAYER_WIDTH: u32 = 2; // m
const PLAYER_HEIGHT: u32 = 2; // m
const PLAYER_SPEED: u32 = 1; // m/s

const WALL_WIDTH: u32 = 2; // m
const WALL_HEIGHT: u32 = 2; // m
const WALL_COUNT: usize = 20;

#[derive(Clone, Copy, PartialEq)]
struct PlayerPos(u32, u32); // x, y

#[derive(Clone, Copy)]
struct WallPos(u32, u32); // x, y

/// Linear Congruential Generator (LCG).  See also:
/// https://stackoverflow.com/a/3062783/3116635
#[derive(Default)]
struct Random(u32);

impl Random {
    fn next_u32(&mut self) -> u32 {
        self.0 = 1103515245u32.wrapping_mul(self.0).wrapping_add(12345);
        self.0
    }

    fn next_wall(&mut self) -> WallPos {
        let x = self.next_u32();
        let y = self.next_u32();
        WallPos(
            x % (DUNGEON_WIDTH - WALL_WIDTH),
            y % (DUNGEON_HEIGHT - WALL_HEIGHT),
        )
    }

    fn next_walls(&mut self) -> Vec<WallPos> {
        (0..WALL_COUNT).map(|_| self.next_wall()).collect()
    }
}

/// Axis-Aligned Bounding Box (AABB) collision detector.
fn is_player_touching_wall(player: PlayerPos, wall: WallPos) -> bool {
    ((wall.0..wall.0 + WALL_WIDTH).contains(&player.0)
        || (wall.0 + 1..wall.0 + WALL_WIDTH).contains(&(player.0 + PLAYER_WIDTH)))
        && ((wall.1..wall.1 + WALL_HEIGHT).contains(&player.1)
            || (wall.1 + 1..wall.1 + WALL_HEIGHT).contains(&(player.1 + PLAYER_HEIGHT)))
}

fn update_position(pos: PlayerPos, ev: KeyboardEvent) -> PlayerPos {
    let PlayerPos(x0, y0) = pos;
    let key = ev.key();
    let x1 = match key.as_str() {
        "h" | "y" | "b" => x0.saturating_sub(PLAYER_SPEED),
        "l" | "u" | "n" => (x0 + PLAYER_SPEED).min(DUNGEON_WIDTH - PLAYER_WIDTH),
        _ => x0,
    };
    let y1 = match key.as_str() {
        "k" | "y" | "u" => y0.saturating_sub(PLAYER_SPEED),
        "j" | "b" | "n" => (y0 + PLAYER_SPEED).min(DUNGEON_HEIGHT - PLAYER_HEIGHT),
        _ => y0,
    };
    PlayerPos(x1, y1)
}

fn update_walls_and_score(
    set_walls: WriteSignal<Vec<(usize, WallPos)>>,
    set_score: WriteSignal<usize>,
    pos: PlayerPos,
    rng: &mut Random,
) {
    set_walls.update(|walls| {
        debug_assert!(!walls.is_empty());
        let next_generation = walls[0].0 / WALL_COUNT + 1;
        let len_before = walls.len();
        walls.retain(|&(_, wall)| !is_player_touching_wall(pos, wall));
        if walls.len() == len_before {
            return;
        }
        set_score.update(|score| *score += len_before - walls.len());
        if !walls.is_empty() {
            return;
        }
        // Pair each new wall a unique ID based on generation and index.
        let new_walls = rng
            .next_walls()
            .into_iter()
            .enumerate()
            .map(|(index, wall)| (next_generation * WALL_COUNT + index, wall));
        walls.extend(new_walls);
    });
}

#[component]
fn Wall(x: u32, y: u32) -> impl IntoView {
    view! {
        <span
           class="entity wall"
           style:translate=move || format!("{}px {}px", x * SCALE, y * SCALE)
           style:width=format!("{}px", WALL_WIDTH * SCALE)
           style:height=format!("{}px", WALL_HEIGHT * SCALE)
        >
            "🧱"
        </span>
    }
}

#[component]
fn Player(pos: ReadSignal<PlayerPos>) -> impl IntoView {
    view! {
        <span
            class="entity mob"
            style:translate=move || {
                let PlayerPos(x, y) = pos();
                format!("{}px {}px", x * SCALE, y * SCALE)
            }
            style:width=format!("{}px", PLAYER_WIDTH * SCALE)
            style:height=format!("{}px", PLAYER_HEIGHT * SCALE)
        >
            "🚜"
        </span>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (player, set_player) = create_signal(PlayerPos(
        (DUNGEON_WIDTH - PLAYER_WIDTH) / 2,
        (DUNGEON_HEIGHT - PLAYER_HEIGHT) / 2,
    ));

    // Pair each wall with its initial index, for use as a unique ID.
    let mut rng = Random::default();
    let walls: Vec<(usize, WallPos)> = rng.next_walls().into_iter().enumerate().collect();
    let (walls, set_walls) = create_signal(walls);

    let (score, set_score) = create_signal(0);

    let handle_keypress = move |ev: KeyboardEvent| {
        let pos0 = player();
        let pos1 = update_position(pos0, ev);
        if pos0 == pos1 {
            return;
        }
        set_player.set(pos1);
        update_walls_and_score(set_walls, set_score, pos1, &mut rng);
    };

    view! {
        <main
            style:position="relative"
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
                key=|wall| wall.0
                let:child
            >
                <Wall x={child.1.0} y={child.1.1}/>
            </For>
            <Player pos=player/>
            <p style:position="absolute" style:bottom=0>{move || score}</p>
        </main>
    }
}
