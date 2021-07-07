#![no_std]
#![no_main]
extern crate snake_vm_sdk as sdk;

use core::panic::PanicInfo;
use sdk::{Direction, Observation};
use core::hint::unreachable_unchecked;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
extern fn main() {
    loop {
        let dir = rand_dir();
        if is_dir_safe(dir) {
            sdk::set_direction(dir);
            sdk::move_snake();
            sdk::sleep_remaining_tick();
        }
    }
}

fn rand_dir() -> Direction {
    match sdk::rand(0,3) {
        0 => Direction::North,
        1 => Direction::East,
        2 => Direction::South,
        3 => Direction::West,
        _ => unreachable!()
    }
}

fn is_dir_safe(dir: Direction) -> bool {
    let (x,y) = sdk::get_current_pos();
    let (width, height) = sdk::get_arena_size();
    let (target_x, target_y) = match dir {
        Direction::North => {
            if y == 0 {
                return false;
            }
            (x, y-1)
        },
        Direction::East => {
            if x == width -1 {
                return false;
            }
            (x+1, y)
        }
        Direction::South => {
            if y == height -1 {
                return false;
            }
            (x, y+1)
        }
        Direction::West => {
            if x == 0 {
                return false;
            }
            (x-1, y)
        }
    };
    match sdk::observe(target_x, target_y) {
        Observation::Empty | Observation::Food => true,
        _ => false
    }
}

fn move_to_origin() {
    loop {
        let (x, y) = sdk::get_current_pos();
        if (x, y) == (0, 0) {
            return;
        }
        if x > 0 {
            sdk::set_direction(Direction::West);
            sdk::sleep_remaining_tick();
        } else if y > 0 {
            sdk::set_direction(Direction::North);
            sdk::sleep_remaining_tick();
        }
    }
}