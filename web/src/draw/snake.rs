use crate::themes::current::*;
use super::{ Draw, DrawContext };

use std::cmp::min;

pub struct Snake;

impl Draw for Snake {
    type Context = DrawContext;

    fn draw<'c>(&self, context: &'c Self::Context) {
        let DrawContext { canvas, context, game } = context;
        let game = game.read().unwrap();

        let tile_size = min(canvas.width(), canvas.height()) as f64 / (game.size() + 2) as f64;
        let grid_size = tile_size * game.size() as f64;
        let start_x = canvas.width() as f64 / 2.0 - grid_size / 2.0;
        let start_y = canvas.height() as f64 / 2.0 - grid_size / 2.0;

        context.set_fill_style_color(PRIMARY);
        for coords in game.snake().body().iter() {
            context.fill_rect(start_x + coords.x() as f64 * tile_size, start_y + coords.y() as f64 * tile_size, tile_size, tile_size);
        }
    }
}
