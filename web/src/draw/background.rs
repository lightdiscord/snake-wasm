use crate::themes::current::*;
use super::{ Draw, DrawContext };

pub struct Background;

impl Draw for Background {
    type Context = DrawContext;

    fn draw<'c>(&self, context: &'c Self::Context) {
        let DrawContext { context, canvas, .. } = context;

        context.set_fill_style_color(SECONDARY);
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    }
}
