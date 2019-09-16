use crate::themes::current::*;
use super::{ Draw, DrawContext };

pub struct Grid;

impl Draw for Grid {
    type Context = DrawContext;

    fn draw<'c>(&self, context: &'c Self::Context) {
        let DrawContext { context, canvas, game } = context;
        let game = game.read().unwrap();

        let cwidth = canvas.width() as f64;
        let cheight = canvas.height() as f64;
        // Can't use the min function because f64 doesn't implement the Ord trait.
        let min = if cwidth < cheight { cwidth } else { cheight };
        let tile_size = min / (game.size() + 2) as f64;
        let grid_size = tile_size * (game.size() as f64);
        let start_x = cwidth / 2.0 - grid_size / 2.0;
        let end_x = cwidth / 2.0 + grid_size / 2.0;
        let start_y = cheight / 2.0 - grid_size / 2.0;
        let end_y = cheight / 2.0 + grid_size / 2.0;

        context.set_stroke_style_color(GRID);

        for y in 0..(game.size() + 1) {
            context.begin_path();
            context.move_to(start_x, (y as f64) * tile_size + start_y);
            context.line_to(end_x, (y as f64) * tile_size + start_y);
            context.stroke();
        }

        for x in 0..(game.size() + 1) {
            context.begin_path();
            context.move_to((x as f64) * tile_size + start_x, start_y);
            context.line_to((x as f64) * tile_size + start_x, end_y);
            context.stroke();
        }
    }
}

