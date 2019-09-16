use super::{ Draw, DrawContext };

pub struct Clear;

impl Draw for Clear {
    type Context = DrawContext;

    fn draw<'c>(&self, context: &'c Self::Context) {
        let DrawContext { context, canvas, .. } = context;

        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    }
}
