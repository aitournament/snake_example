#![no_main]
#![no_std]

/*
To compile:
cargo build --target=wasm32-unknown-unknown --release

wasm file will be built in ./target/wasm32-unknown-unknown/release/
 */

extern crate snake_vm_sdk as sdk;

use core::cmp::Ordering;
use core::panic::PanicInfo;
use sdk::{Direction, Observation};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn recurse(x: u32) -> u32 {
    recurse(x) + recurse(x + 1)
}

#[no_mangle]
extern "C" fn main() {
    let width = sdk::get_arena_width();
    let height = sdk::get_arena_height();
    let mut target_pos = None;

    recurse(0);

    loop {
        // check 20 random squares for food
        if target_pos.is_none() {
            for _ in 0..20 {
                let x = sdk::rand(0, width - 1);
                let y = sdk::rand(0, height - 1);
                if let Observation::Food(food_info) = sdk::observe(x, y) {
                    if food_info.health_value >= 0 {
                        target_pos = Some((x, y));
                        break;
                    }
                }
            }
        }

        if let Some(food_target) = target_pos {
            go_to(food_target);
            target_pos = None;
        } else {
            random_single_move();
            sdk::sleep_remaining_tick();
        }
    }
}

fn go_to(target: (u32, u32)) {
    while let Some(direction) = direction_towards(target) {
        if is_dir_safe(direction) {
            sdk::set_direction(direction);
            sdk::move_snake();
            sdk::sleep_remaining_tick();
        } else {
            random_single_move();
            sdk::sleep_remaining_tick();
        }
    }
}

fn direction_towards(target: (u32, u32)) -> Option<Direction> {
    let current_pos = sdk::get_current_pos();
    match target.1.cmp(&current_pos.1) {
        Ordering::Less => return Some(Direction::North),
        Ordering::Equal => {}
        Ordering::Greater => return Some(Direction::South),
    };
    match target.0.cmp(&current_pos.0) {
        Ordering::Less => Some(Direction::West),
        Ordering::Equal => None,
        Ordering::Greater => Some(Direction::East),
    }
}

fn random_single_move() {
    loop {
        let dir = rand_dir();
        if is_dir_safe(dir) {
            sdk::set_direction(dir);
            sdk::move_snake();
            return;
        }
    }
}

fn rand_dir() -> Direction {
    match sdk::rand(0, 3) {
        0 => Direction::North,
        1 => Direction::East,
        2 => Direction::South,
        3 => Direction::West,
        _ => unreachable!(),
    }
}

fn is_dir_safe(dir: Direction) -> bool {
    let (x, y) = sdk::get_current_pos();
    let (width, height) = sdk::get_arena_size();
    let (target_x, target_y) = match dir {
        Direction::North => {
            if y == 0 {
                return false;
            }
            (x, y - 1)
        }
        Direction::East => {
            if x == width - 1 {
                return false;
            }
            (x + 1, y)
        }
        Direction::South => {
            if y == height - 1 {
                return false;
            }
            (x, y + 1)
        }
        Direction::West => {
            if x == 0 {
                return false;
            }
            (x - 1, y)
        }
    };
    match sdk::observe(target_x, target_y) {
        Observation::Empty => true,
        Observation::Food(food_info) => food_info.health_value >= 0,
        _ => false,
    }
}
