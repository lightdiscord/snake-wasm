pub mod themes;
pub mod draw;

use snake::{ Game, Direction };

use stdweb::traits::*;
use stdweb::web::{ window, document, interval_buffered, CanvasRenderingContext2d };
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::{ ResizeEvent, KeyDownEvent };
use stdweb::unstable::TryInto;

use std::sync::{ Arc, RwLock };

use futures_util::stream::StreamExt;
use futures_util::future;

use failure::Fallible;

use draw::{ DrawContext, DrawLoop };

fn keydown_event(game: Arc<RwLock<Game>>) {
    window().add_event_listener(move |event: KeyDownEvent| {
        match event.key().as_str() {
            "ArrowRight" => (game.write().unwrap().snake.set_direction(Direction::Right)),
            "ArrowLeft" => (game.write().unwrap().snake.set_direction(Direction::Left)),
            "ArrowUp" => (game.write().unwrap().snake.set_direction(Direction::Up)),
            "ArrowDown" => (game.write().unwrap().snake.set_direction(Direction::Down)),
            _ => ()
        }
    });
}

fn resize_canvas(canvas: &CanvasElement) {
    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);
}

fn resize_event(canvas: CanvasElement) {
    window().add_event_listener(move |_: ResizeEvent| resize_canvas(&canvas));
}

fn main() -> Fallible<()> {
    let game = Arc::new(RwLock::new(Game::new()));
    let canvas: CanvasElement = document().query_selector("#game")?.unwrap().try_into()?;
    let context: CanvasRenderingContext2d = canvas.get_context()?;

    resize_canvas(&canvas);
    resize_event(canvas.clone());
    keydown_event(game.clone());

    let tick_loop = interval_buffered(100).for_each({
        let game = game.clone();
        move |_| {
            let _ = game.write().unwrap().tick();
            future::ready(())
        }
    });

    stdweb::spawn_local(tick_loop);
    DrawLoop::new(DrawContext { canvas, context, game }).start();

    Ok(())
}
