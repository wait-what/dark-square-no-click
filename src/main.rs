#![windows_subsystem = "windows"]

extern crate rand;

use macroquad::{audio::{load_sound_from_bytes, play_sound_once, Sound}, prelude::*};
use rand::Rng;

const SQUARE_COUNT: usize = 4;
const TARGET_COUNT: usize = 3;
const TIME: f32 = 15.;

#[derive(PartialEq, Copy, Clone)]
enum CellState {
    Blank,
    Target,
    Failure,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Dark square, No click!".to_owned(),
        window_width: 640,
        window_height: 660,
        ..Default::default()
    }
}

fn add_random_target(board: &[[CellState; SQUARE_COUNT]; SQUARE_COUNT]) -> (usize, usize) {
    let mut rng = rand::thread_rng();

    loop {
        let (x, y) = (
            rng.gen_range(0..SQUARE_COUNT),
            rng.gen_range(0..SQUARE_COUNT),
        );

        if board[y][x] == CellState::Target {
            continue;
        } else {
            return (x, y);
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Setup
    show_mouse(false);

    let mut rng = rand::thread_rng();

    let sound_bytes: [&[u8]; 6] = [
        include_bytes!("./sounds/sound_1.ogg"),
        include_bytes!("./sounds/sound_2.ogg"),
        include_bytes!("./sounds/sound_3.ogg"),
        include_bytes!("./sounds/sound_4.ogg"),
        include_bytes!("./sounds/sound_5.ogg"),
        include_bytes!("./sounds/sound_6.ogg"),
    ];
    let sounds: [Sound; 6] = [
        load_sound_from_bytes(sound_bytes[0]).await.expect("sound_1.ogg is corrupt"),
        load_sound_from_bytes(sound_bytes[1]).await.expect("sound_2.ogg is corrupt"),
        load_sound_from_bytes(sound_bytes[2]).await.expect("sound_3.ogg is corrupt"),
        load_sound_from_bytes(sound_bytes[3]).await.expect("sound_4.ogg is corrupt"),
        load_sound_from_bytes(sound_bytes[4]).await.expect("sound_5.ogg is corrupt"),
        load_sound_from_bytes(sound_bytes[5]).await.expect("sound_6.ogg is corrupt"),
    ];

    let mut keydown = false;

    let mut score: usize = 0;
    let mut combo: f32 = 0.;
    let mut failed = false;
    let mut time = TIME;

    let mut board_state: [[CellState; SQUARE_COUNT]; SQUARE_COUNT] =
        [[CellState::Blank; SQUARE_COUNT]; SQUARE_COUNT];
    for _ in 0..TARGET_COUNT {
        let (x, y) = add_random_target(&board_state);
        board_state[y][x] = CellState::Target;
    }

    loop {
        // Check if keys are pressed
        if is_mouse_button_released(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Right)
            || is_key_released(KeyCode::Z)
            || is_key_released(KeyCode::X)
            || is_key_released(KeyCode::Space)
        {
            keydown = false;
        }

        // Pre-calculation
        let smallest_dimension = if screen_height() > screen_width() {
            screen_width()
        } else {
            screen_height()
        };

        let square_size = smallest_dimension / SQUARE_COUNT as f32;
        let gap_size = smallest_dimension / (25. * SQUARE_COUNT as f32);

        let mouse_transform = if screen_height() > screen_width() {
            vec2(1., screen_height() / screen_width())
        } else {
            vec2(screen_width() / screen_height(), 1.)
        };
        let (mouse_x, mouse_y): (usize, usize) = (
            ((mouse_position_local().x + 1.) * (SQUARE_COUNT / 2) as f32 * mouse_transform.x)
                as usize,
            ((mouse_position_local().y + 1.) * (SQUARE_COUNT / 2) as f32 * mouse_transform.y)
                as usize,
        );

        // Rendering
        clear_background(GRAY);

        for x in 0..SQUARE_COUNT {
            for y in 0..SQUARE_COUNT {
                draw_rectangle(
                    x as f32 * square_size + gap_size / 2.,
                    y as f32 * square_size + gap_size / 2.,
                    square_size - gap_size,
                    square_size - gap_size,
                    match board_state[y][x] {
                        CellState::Blank => BLACK,
                        CellState::Target => WHITE,
                        CellState::Failure => RED,
                    },
                );
            }
        }

        draw_rectangle(
            0. + gap_size / 2.,
            smallest_dimension,
            smallest_dimension * combo,
            smallest_dimension / 64.,
            match (combo * 5.) as usize + 1 {
                1 => RED,
                2 => YELLOW,
                3 => GREEN,
                4 => BLUE,
                5 => PURPLE,
                _ => PINK,
            }
        );

        draw_rectangle(
            0. + gap_size / 2.,
            smallest_dimension + smallest_dimension / 64.,
            smallest_dimension * clamp(time / TIME, 0., 1.),
            smallest_dimension / 64.,
            WHITE,
        );

        draw_text_ex(
            score.to_string().as_str(),
            20.,
            smallest_dimension / 10. + 20.,
            TextParams {
                font_size: smallest_dimension as u16 / 5,
                color: color_u8!(128, 128, 256, 128),
                ..TextParams::default()
            }
        );

        draw_circle(
            (mouse_position_local().x + 1.) * (screen_width() / 2.),
            (mouse_position_local().y + 1.) * (screen_height() / 2.),
            smallest_dimension / 50.,
            color_u8!(64, 255, 64, 192),
        );

        if failed {
            draw_rectangle(0., 0., screen_width(), screen_height(), color_u8!(255, 128, 128, 32));
        }

        // Logic
        if !failed {
            combo = clamp(combo - get_frame_time() / 10., 0., 1.);
            if score > 0 {
                time -= get_frame_time();
            }
        }

        if time <= 0. {
            failed = true;
        }

        if !keydown && is_key_down(KeyCode::Space) {
            keydown = true;

            score = 0;
            failed = false;
            time = TIME;
            combo = 0.;

            for x in 0..SQUARE_COUNT {
                for y in 0..SQUARE_COUNT {
                    board_state[y][x] = CellState::Blank;
                }
            }

            for _ in 0..TARGET_COUNT {
                let (x, y) = add_random_target(&board_state);
                board_state[y][x] = CellState::Target;
            }
        }

        if !keydown && !failed
            && (is_mouse_button_down(MouseButton::Left)
                || is_mouse_button_down(MouseButton::Right)
                || is_key_down(KeyCode::Z)
                || is_key_down(KeyCode::X))
        {
            keydown = true;
            if mouse_x > SQUARE_COUNT - 1 || mouse_y > SQUARE_COUNT - 1 {
                continue;
            }

            if board_state[mouse_y][mouse_x] == CellState::Target {
                play_sound_once(sounds[rng.gen_range(0..6)]);

                let (random_x, random_y) = add_random_target(&board_state);
                board_state[random_y][random_x] = CellState::Target;

                score += (combo * 5.) as usize + 1;
                combo = clamp(combo + 0.05 * (4. / (combo + 2.) - 1.), 0., 5.);

                board_state[mouse_y][mouse_x] = CellState::Blank;
            } else {
                board_state[mouse_y][mouse_x] = CellState::Failure;
                failed = true;
            }
        };

        next_frame().await
    }
}
