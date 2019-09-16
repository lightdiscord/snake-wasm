use snake::Game;
use snake::Direction;

use stdweb::console;

use stdweb::traits::*;
use stdweb::web::{ window, document, interval_buffered, CanvasRenderingContext2d };
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::ResizeEvent;
use stdweb::unstable::TryInto;

use futures_util::stream::StreamExt;
use failure::Fallible;

use std::cmp::min;

const PRIMARY: &'static str = "#ffffff";
const SECONDARY: &'static str = "#000000";
const GRID: &'static str = "#7f7f7f";
const GOAL: &'static str = "#d50000";

fn draw(canvas: CanvasElement, context: CanvasRenderingContext2d, game: Arc<RwLock<Game>>) {
    {
        let game = game.read().unwrap();
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        context.set_fill_style_color(SECONDARY);
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        let tile_size = min(canvas.width(), canvas.height()) as f64 / (game.size() + 2) as f64;

        context.set_stroke_style_color(GRID);

        let grid_size = tile_size * game.size() as f64;

        let start_x = canvas.width() as f64 / 2.0 - grid_size / 2.0;
        let end_x = canvas.width() as f64 / 2.0 + grid_size / 2.0;

        let start_y = canvas.height() as f64 / 2.0 - grid_size / 2.0;
        let end_y = canvas.height() as f64 / 2.0 + grid_size / 2.0;

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

        context.set_fill_style_color(PRIMARY);
        for coords in game.snake().body().iter() {
            context.fill_rect(start_x + coords.x() as f64 * tile_size, start_y + coords.y() as f64 * tile_size, tile_size, tile_size);
        }

        context.set_fill_style_color(GOAL);
        context.fill_rect(start_x + game.goal().x() as f64 * tile_size, start_y + game.goal().y() as f64 * tile_size, tile_size, tile_size);
    }

    window().request_animation_frame(|_| draw(canvas, context, game));
}

use futures_util::future;
use std::sync::{ Arc, RwLock };

fn main() -> Fallible<()> {
    let game = Arc::new(RwLock::new(Game::new()));
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

    window().add_event_listener({
        use stdweb::web::event::KeyDownEvent;

        let game = game.clone();
        move |event: KeyDownEvent| {
            match event.key().as_str() {
                "ArrowRight" => (game.write().unwrap().set_direction(Direction::Right)),
                "ArrowLeft" => (game.write().unwrap().set_direction(Direction::Left)),
                "ArrowUp" => (game.write().unwrap().set_direction(Direction::Up)),
                "ArrowDown" => (game.write().unwrap().set_direction(Direction::Down)),
                _ => ()
            }
        }
    });

    let render_game = game.clone();

    let tick_loop = interval_buffered(100)
        .for_each(move |_| {
            game.write().unwrap().tick();
            future::ready(())
        });

    stdweb::spawn_local(tick_loop);

    window().request_animation_frame(|_| draw(canvas, context, render_game));

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
