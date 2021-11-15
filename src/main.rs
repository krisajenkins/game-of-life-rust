use rand::Rng;
use std::collections::HashSet;
use std::thread::sleep;
use std::time::Duration;

const X_MAX: i16 = 80;
const Y_MAX: i16 = 60;

fn main() {
    let mut world = HashSet::new();

    let mut rng = rand::thread_rng();

    for _ in 0..500 {
        let x = rng.gen_range(0..X_MAX);
        let y = rng.gen_range(0..Y_MAX);
        world.insert((x, y));
    }

    loop {
        let new_world = iterate(X_MAX, Y_MAX, &world);
        display(X_MAX, Y_MAX, &new_world, &world);
        sleep(Duration::from_millis(50));
        world = new_world;
    }
}

fn display(
    x_max: i16,
    y_max: i16,
    new_world: &HashSet<(i16, i16)>,
    old_world: &HashSet<(i16, i16)>,
) {
    print!("\x1B[2J\x1B[1;1H");

    for y in 0..y_max {
        for x in 0..x_max {
            let point = (x, y);
            if new_world.contains(&point) {
                print!("O");
            } else if old_world.contains(&point) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn iterate(x_max: i16, y_max: i16, old_world: &HashSet<(i16, i16)>) -> HashSet<(i16, i16)> {
    let mut new_world = HashSet::new();

    for x in 0..x_max {
        for y in 0..y_max {
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
                    n = n + 1;
                }
            }

            if n == 2 && old_world.contains(&(x, y)) {
                new_world.insert((x, y));
            }
            if n == 3 {
                new_world.insert((x, y));
            }
        }
    }

    new_world
}
