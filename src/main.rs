use rand::Rng;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

const X_MAX: i16 = 200;
const Y_MAX: i16 = 60;
const SLEEP_TIME: u64 = 10;

enum CellState {
    Live,
    Dead,
}

type Coord = (i16, i16);
type World = HashSet<Coord>;
type Change = (Coord, CellState);

struct Update {
    new_world: World,
    changes: Vec<Change>,
}

fn main() {
    let mut world = HashSet::new();

    // Fill the world with random cells.
    let mut rng = rand::thread_rng();
    for _ in 0..(X_MAX * Y_MAX / 2) {
        let x = rng.gen_range(0..X_MAX);
        let y = rng.gen_range(0..Y_MAX);
        world.insert((x, y));
    }

    loop {
        let update = step_world_forward(X_MAX, Y_MAX, &world);
        display(&update);
        world = update.new_world;
        sleep(Duration::from_millis(SLEEP_TIME));
    }
}

// NOTE: Ansi Escape Codes: https://en.wikipedia.org/wiki/ANSI_escape_code
//
// `CSI 17;5H` is the control code that moves the cursor to 17,5. The screen is 1-indexed from
// top-left.
const ESCAPE: &str = "\x1B";
const CLEAR_SCREEN: &str = "[2J";
const GO_TO_TOP_LEFT: &str = "[1;1H";

fn display(update: &Update) {
    let mut s: String = String::new();

    s += &format!("{}{}", ESCAPE, CLEAR_SCREEN);
    s += &format!("{}{}", ESCAPE, GO_TO_TOP_LEFT);

    for ((x, y), state) in update.changes.iter() {
        s += &format!("{}[{};{}H", ESCAPE, y, x);

        s += match state {
            CellState::Live => "X",
            CellState::Dead => " ",
        };
    }
    print!("{}", s);
}

fn count_neighbours(x: i16, y: i16, old_world: &World) -> i16 {
    let mut n = 0;

    for (dx, dy) in [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        if old_world.contains(&(x + dx, y + dy)) {
            n += 1;
        }
    }

    return n;
}

fn step_world_forward(x_max: i16, y_max: i16, old_world: &World) -> Update {
    let mut new_world = HashSet::new();
    let mut changes = Vec::new();

    for x in 0..x_max {
        for y in 0..y_max {
            let n = count_neighbours(x, y, old_world);

            match (n, old_world.contains(&(x, y))) {
                (2, true) => {
                    new_world.insert((x, y));
                }
                (3, true) => {
                    new_world.insert((x, y));
                }
                (3, false) => {
                    new_world.insert((x, y));
                    changes.push(((x, y), CellState::Live))
                }
                (_, true) => changes.push(((x, y), CellState::Dead)),
                (_, _) => (),
            };
        }
    }

    return Update { new_world, changes };
}
