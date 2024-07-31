use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
use framebuffer::Framebuffer;

const WIDTH: usize = 200;
const HEIGHT: usize = 200;
const CELL_SIZE: usize = 5;

const COLOR_FONDO: u32 = 0x000000;
const COLOR_CUADRO: u32 = 0xFFFFFF;
const COLOR_OSCILLATOR: u32 = 0xFFFF00;
const COLOR_SPACESHIP: u32 = 0xFF00FF;
const COLOR_STILL_LIFE: u32 = 0x00FF00;

fn add_pattern(framebuffer: &mut Framebuffer, pattern: &[(usize, usize)], x_offset: usize, y_offset: usize, color: u32) {
    for &(x, y) in pattern {
        framebuffer.point(x + x_offset, y + y_offset, color);
    }
}

fn count_neighbors(framebuffer: &Framebuffer, x: usize, y: usize) -> u8 {
    let mut count = 0;
    for dx in [-1, 0, 1].iter().cloned() {
        for dy in [-1, 0, 1].iter().cloned() {
            if dx != 0 || dy != 0 {
                let nx = (x as isize + dx).rem_euclid(WIDTH as isize) as usize;
                let ny = (y as isize + dy).rem_euclid(HEIGHT as isize) as usize;
                if framebuffer.is_point_set(nx, ny) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn update(framebuffer: &Framebuffer) -> Framebuffer {
    let mut new_framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let neighbors = count_neighbors(framebuffer, x, y);
            if framebuffer.is_point_set(x, y) {
                if neighbors == 2 || neighbors == 3 {
                    new_framebuffer.point(x, y, framebuffer.buffer[y * WIDTH + x]);
                }
            } else {
                if neighbors == 3 {
                    new_framebuffer.point(x, y, COLOR_CUADRO);
                }
            }
        }
    }
    new_framebuffer
}

fn initialize_pattern(framebuffer: &mut Framebuffer) {
    let block = vec![(0, 0), (1, 0), (0, 1), (1, 1)];
    let beehive = vec![(0, 1), (1, 0), (2, 0), (3, 1), (1, 2), (2, 2)];
    let loaf = vec![(2, 0), (3, 1), (3, 2), (1, 1), (0, 2), (1, 3), (2, 3)];
    let boat = vec![(0, 0), (1, 0), (0, 1), (2, 1), (1, 2)];
    let tub = vec![(1, 0), (0, 1), (2, 1), (1, 2)];
    let blinker = vec![(0, 0), (1, 0), (2, 0)];
    let toad = vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)];
    let beacon = vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 2), (3, 2), (2, 3), (3, 3)];
    let pulsar = vec![
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2), (0, 3), (5, 3),
        (7, 3), (12, 3), (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8), (0, 9), (5, 9),
        (7, 9), (12, 9), (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ];
    let pentadecathlon = vec![
        (0, 0), (1, 0), (2, 0), (3, 1), (3, 2), (2, 3), (1, 3),
        (0, 3), (2, 4), (1, 4), (0, 4), (3, 5), (3, 6), (2, 7), 
        (1, 7), (0, 7), (1, 8), (2, 8), (3, 9), (0, 9)
    ];
    let glider = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    let lwss = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 1), (0, 1), (0, 2), (3, 2), (1, 3)];
    let mwss = vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 1), (0, 1), (0, 2), (4, 3), (1, 4)];
    let hwss = vec![
        (0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (5, 1), (0, 1), (0, 2),
        (4, 3), (1, 4), (2, 4), (3, 4), (4, 4), (5, 4), (6, 4), (0, 3)
    ];

    for i in (0..WIDTH).step_by(30) {
        for j in (0..HEIGHT).step_by(30) {
            add_pattern(framebuffer, &block, i, j, COLOR_STILL_LIFE);
            add_pattern(framebuffer, &beehive, i + 7, j, COLOR_STILL_LIFE);
            add_pattern(framebuffer, &loaf, i + 14, j, COLOR_STILL_LIFE);
            add_pattern(framebuffer, &boat, i + 21, j, COLOR_STILL_LIFE);
            add_pattern(framebuffer, &tub, i, j + 7, COLOR_STILL_LIFE);
            add_pattern(framebuffer, &blinker, i + 7, j + 7, COLOR_OSCILLATOR);
            add_pattern(framebuffer, &toad, i + 14, j + 7, COLOR_OSCILLATOR);
            add_pattern(framebuffer, &beacon, i + 21, j + 7, COLOR_OSCILLATOR);
            add_pattern(framebuffer, &pulsar, i, j + 14, COLOR_OSCILLATOR);
            add_pattern(framebuffer, &pentadecathlon, i + 7, j + 14, COLOR_OSCILLATOR);
            add_pattern(framebuffer, &glider, i + 14, j + 14, COLOR_SPACESHIP);
            add_pattern(framebuffer, &lwss, i + 21, j + 14, COLOR_SPACESHIP);
            add_pattern(framebuffer, &mwss, i, j + 21, COLOR_SPACESHIP);
            add_pattern(framebuffer, &hwss, i + 7, j + 21, COLOR_SPACESHIP);
        }
    }
}

fn main() {
    let window_width = WIDTH * CELL_SIZE;
    let window_height = HEIGHT * CELL_SIZE;
    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);
    initialize_pattern(&mut framebuffer);

    let mut window = Window::new(
        "Conway's Game of Life",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer = update(&framebuffer);

        let mut display_buffer = vec![COLOR_FONDO; window_width * window_height];
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if framebuffer.is_point_set(x, y) {
                    for dy in 0..CELL_SIZE {
                        for dx in 0..CELL_SIZE {
                            display_buffer[(y * CELL_SIZE + dy) * window_width + (x * CELL_SIZE + dx)] = framebuffer.buffer[y * WIDTH + x];
                        }
                    }
                }
            }
        }

        window
            .update_with_buffer(&display_buffer, window_width, window_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
