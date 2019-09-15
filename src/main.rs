use stdweb::traits::*;
use stdweb::web::{ window, document, CanvasRenderingContext2d };
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::ResizeEvent;
use stdweb::unstable::TryInto;

use failure::Fallible;

use std::cmp::min;

const PRIMARY: &'static str = "#ffffff";
const SECONDARY: &'static str = "#000000";
const GRID: &'static str = "#7f7f7f";

pub struct Game {
    size: usize
}

impl Game {
    pub fn new() -> Self {
        Game {
            size: 50
        }
    }
}

fn draw(canvas: CanvasElement, context: CanvasRenderingContext2d, game: Game) {
    context.set_fill_style_color(SECONDARY);
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    let tile_size = min(canvas.width(), canvas.height()) as f64 / (game.size + 2) as f64;

    context.set_stroke_style_color(GRID);

    let grid_size = tile_size * game.size as f64;

    let start_x = canvas.width() as f64 / 2.0 - grid_size / 2.0;
    let end_x = canvas.width() as f64 / 2.0 + grid_size / 2.0;

    let start_y = canvas.height() as f64 / 2.0 - grid_size / 2.0;
    let end_y = canvas.height() as f64 / 2.0 + grid_size / 2.0;

    for y in 0..(game.size + 1) {
        context.begin_path();
        context.move_to(start_x, (y as f64) * tile_size + start_y);
        context.line_to(end_x, (y as f64) * tile_size + start_y);
        context.stroke();
    }

    for x in 0..(game.size + 1) {
        context.begin_path();
        context.move_to((x as f64) * tile_size + start_x, start_y);
        context.line_to((x as f64) * tile_size + start_x, end_y);
        context.stroke();
    }


    window().request_animation_frame(|_| draw(canvas, context, game));
}

fn main() -> Fallible<()> {
    let game = Game::new();
    let canvas: CanvasElement = document()
        .query_selector("#game")?
        .unwrap()
        .try_into()?;

    let context: CanvasRenderingContext2d = canvas.get_context()?;

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);

    window().add_event_listener({
        let canvas = canvas.clone();
        move |_: ResizeEvent| {
            canvas.set_width(canvas.offset_width() as u32);
            canvas.set_height(canvas.offset_height() as u32);
        }
    });

    window().request_animation_frame(|_| draw(canvas, context, game));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
