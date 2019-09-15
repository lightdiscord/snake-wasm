use stdweb::traits::*;
use stdweb::web::{ window, document, CanvasRenderingContext2d };
use stdweb::web::html_element::CanvasElement;
use stdweb::web::event::{ MouseMoveEvent, ResizeEvent };
use stdweb::unstable::TryInto;

use failure::Fallible;

fn main() -> Fallible<()> {
    stdweb::initialize();

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

    canvas.add_event_listener({
        let context = context.clone();
        move |event: MouseMoveEvent| {
            context.fill_rect(f64::from(event.client_x() - 5), f64::from(event.client_y() - 5), 10.0, 10.0);
        }
    });

    stdweb::event_loop();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
