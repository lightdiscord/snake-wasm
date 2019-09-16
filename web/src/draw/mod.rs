macro_rules! mod_use {
    (
        $mod:ident::$use:ident
    ) => {
        mod $mod;
        use self::$mod::$use;
    }
}

use ::snake::Game;

mod_use!(background::Background);
mod_use!(clear::Clear);
mod_use!(grid::Grid);
mod_use!(snake::Snake);
mod_use!(goal::Goal);

use stdweb::web::{ window, CanvasRenderingContext2d };
use stdweb::web::html_element::CanvasElement;

use std::sync::{ Arc, RwLock };


pub trait Draw {
    type Context;

    fn draw<'c>(&self, context: &'c Self::Context);
}

pub struct DrawContext {
    pub canvas: CanvasElement,
    pub context: CanvasRenderingContext2d,
    pub game: Arc<RwLock<Game>>
}

impl Draw for DrawContext {
    type Context = ();

    fn draw(&self, _: &Self::Context) {
        Clear.draw(self);
        Background.draw(self);
        Grid.draw(self);
        Snake.draw(self);
        Goal.draw(self);

    }
}

pub struct DrawLoop {
    context: DrawContext
}

impl DrawLoop {
    pub fn new(context: DrawContext) -> Self {
        DrawLoop {
            context
        }
    }
    
    fn draw(self) {
        self.context.draw(&());
        window().request_animation_frame(|_| self.draw());
    }

    pub fn start(self) {
        window().request_animation_frame(|_| self.draw());
    }
}
