use snake::Game;
use snake::Direction;

use stdweb::traits::*;
use stdweb::web::{ window, document, interval_buffered, CanvasRenderingContext2d };
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::ResizeEvent;
use stdweb::unstable::TryInto;

use futures_util::stream::StreamExt;
use failure::Fallible;

pub mod themes;
pub mod draw;

use draw::{ DrawContext, DrawLoop };

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
            let _ = game.write().unwrap().tick();
            future::ready(())
        });

    stdweb::spawn_local(tick_loop);

    let draw_context = DrawContext {
        canvas,
        context,
        game: render_game
    };

    DrawLoop::new(draw_context).start();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
