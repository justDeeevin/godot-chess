use crate::classes::ChessBoard2D;
use godot::{
    engine::{global::MouseButton, ISprite2D, InputEvent, InputEventMouseButton, Sprite2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base = Sprite2D)]
pub struct ChessPiece {
    is_held: bool,
    pub index: usize,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for ChessPiece {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            is_held: false,
            index: 0,
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseButton>() {
            if event.get_button_index() == MouseButton::LEFT
                && self
                    .base()
                    .get_rect()
                    .has_point(self.base().to_local(event.get_position()))
            {
                if event.is_pressed() {
                    self.is_held = true;
                    self.base()
                        .get_parent()
                        .unwrap()
                        .cast::<ChessBoard2D>()
                        .bind_mut()
                        .pick(self);
                }
                if event.is_released() && self.is_held {
                    self.is_held = false;
                    self.base()
                        .get_parent()
                        .unwrap()
                        .cast::<ChessBoard2D>()
                        .bind_mut()
                        .place(self, event.get_position());
                }
            }
        }
    }

    fn process(&mut self, delta: f64) {
        if self.is_held {
            let mouse_position = self.base().get_viewport().unwrap().get_mouse_position();
            self.base_mut().set_position(mouse_position);
        }
    }
}
