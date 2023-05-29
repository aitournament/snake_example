use core::cmp::Ordering;
use snake_sdk::{Direction, ObservationItem};

#[no_mangle]
extern "C" fn main() {
    let width = snake_sdk::get_arena_width();
    let height = snake_sdk::get_arena_height();
    let mut target_pos = None;

    loop {
        // check 20 random squares for food
        if target_pos.is_none() {
            for _ in 0..20 {
                let x = snake_sdk::rand(0, width - 1);
                let y = snake_sdk::rand(0, height - 1);
                if let Some(ObservationItem::Food(_)) = snake_sdk::observe(x, y).item {
                    target_pos = Some((x, y));
                    break;
                }
            }
        }

        if let Some(food_target) = target_pos {
            go_to(food_target);
            target_pos = None;
        } else {
            random_single_move();
            snake_sdk::sleep_remaining_tick();
        }
    }
}

fn go_to(target: (u32, u32)) {
    while let Some(direction) = direction_towards(target) {
        if is_dir_safe(direction) {
            snake_sdk::set_direction(direction);
            snake_sdk::move_snake();
            snake_sdk::sleep_remaining_tick();
        } else {
            random_single_move();
            snake_sdk::sleep_remaining_tick();
        }
    }
}

fn direction_towards(target: (u32, u32)) -> Option<Direction> {
    let current_pos = snake_sdk::get_current_pos();
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
            snake_sdk::set_direction(dir);
            snake_sdk::move_snake();
            return;
        }
    }
}

fn rand_dir() -> Direction {
    match snake_sdk::rand(0, 3) {
        0 => Direction::North,
        1 => Direction::East,
        2 => Direction::South,
        3 => Direction::West,
        _ => unreachable!(),
    }
}

fn is_dir_safe(dir: Direction) -> bool {
    let (x, y) = snake_sdk::get_current_pos();
    let (width, height) = snake_sdk::get_arena_size();
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
    let observation = snake_sdk::observe(target_x, target_y);
    if observation.poison > 0 {
        return false;
    }

    match observation.item {
        None => true,
        Some(ObservationItem::Food(_)) => true,
        Some(ObservationItem::SnakeHead(_)) | Some(ObservationItem::SnakeBody(_)) => false,
    }
}
