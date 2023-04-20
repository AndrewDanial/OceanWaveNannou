#![allow(unused)]

use std::io::Write;

use itertools::Itertools;
use names::Generator;
use nannou::{
    glam::Vec2,
    image::DynamicImage,
    noise::{NoiseFn, OpenSimplex, Seedable},
    prelude::*,
    wgpu::Texture,
};

// interesting variables
const NOISE_STEP: f64 = 500.;
const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;
const PIXEL_DENSITY: usize = 4;
fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Debug, Clone)]
struct Cell {
    val: f64,
}

struct Gol {
    board: Vec<Vec<Cell>>,
}

impl Gol {
    fn new() -> Self {
        let noise = OpenSimplex::new().set_seed(random());
        let mut board: Vec<Vec<Cell>> = vec![
            vec![Cell { val: 0. }; HEIGHT as usize / PIXEL_DENSITY];
            WIDTH as usize / PIXEL_DENSITY
        ];
        for x in (-((WIDTH / 2) as i32)..(WIDTH as i32 / 2)).step_by(PIXEL_DENSITY) {
            for y in (-((HEIGHT / 2) as i32)..(HEIGHT as i32 / 2)).step_by(PIXEL_DENSITY) {
                let curr_x = map_range(
                    x,
                    -((WIDTH / 2) as i32),
                    (WIDTH as i32 / 2),
                    0,
                    WIDTH / PIXEL_DENSITY as u32,
                ) as usize;
                let curr_y = map_range(
                    y,
                    -((HEIGHT / 2) as i32),
                    (HEIGHT as i32 / 2),
                    0,
                    HEIGHT / PIXEL_DENSITY as u32,
                ) as usize;
                let val = map_range(noise.get([x as f64, y as f64]), -1., 1., 0., 1.);
                board[curr_x as usize][curr_y as usize] = Cell { val }
            }
        }

        Gol { board }
    }

    fn display(&self, draw: &Draw) {
        for x in (-((WIDTH / 2) as i32)..(WIDTH as i32 / 2)).step_by(PIXEL_DENSITY) {
            for y in (-((HEIGHT / 2) as i32)..(HEIGHT as i32 / 2)).step_by(PIXEL_DENSITY) {
                let curr_x = map_range(
                    x,
                    -((WIDTH / 2) as i32),
                    (WIDTH as i32 / 2),
                    0,
                    WIDTH / PIXEL_DENSITY as u32,
                ) as usize;
                let curr_y = map_range(
                    y,
                    -((HEIGHT / 2) as i32),
                    (HEIGHT as i32 / 2),
                    0,
                    HEIGHT / PIXEL_DENSITY as u32,
                ) as usize;
                let mut val = self.board[curr_x][curr_y].val as f32;
                val = val.clamp(0., 1.);
                draw.rect()
                    .x_y(x as f32, y as f32)
                    .w_h(PIXEL_DENSITY as f32, PIXEL_DENSITY as f32)
                    .color(rgb(val * 1.2, val * 1.2, 1.));
            }
        }
    }

    fn generate(&mut self) {
        let mut next: Vec<Vec<Cell>> = self.board.clone();
        for x in 1..self.board.len() - 1 {
            for y in 1..self.board[x].len() - 1 {
                let prop = random_range::<f64>(0., 1.5);
                let r = random::<f64>();
                let split = random_range(0., 1.);
                let subtracted_val = self.board[x][y].val * prop;
                next[x][y].val -= subtracted_val;
                next[x][y + 1].val += subtracted_val * split * 0.932; // top
                next[x - 1][y].val += subtracted_val * (1. - split) * 0.932; // left

                next[x][y].val = next[x][y].val.clamp(0., 1.);
                if r < 0.001 {
                    next[x][y].val = 0.8;
                }
            }
        }

        self.board = next;
    }
}

struct Model {
    gol: Gol,
}

fn model(app: &App) -> Model {
    let _image_window = app
        .new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();
    Model { gol: Gol::new() }
}
fn update(app: &App, model: &mut Model, _update: Update) {
    model.gol.generate();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let board = model.gol.board.clone();
    model.gol.display(&draw);

    draw.to_frame(app, &frame).unwrap();
}
