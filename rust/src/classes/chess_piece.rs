use crate::classes::ChessBoard2D;
use godot::{
    engine::{
        display_server::CursorShape, global::MouseButton, Area2D, CollisionShape2D, DisplayServer,
        ISprite2D, InputEvent, InputEventMouseButton, RectangleShape2D, Sprite2D,
    },
    prelude::*,
};

#[derive(GodotClass)]
#[class(base = Sprite2D)]
pub struct ChessPiece {
    is_held: bool,
    pub index: usize,
    hovered: bool,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for ChessPiece {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            is_held: false,
            index: 0,
            hovered: false,
            base,
        }
    }

    fn ready(&mut self) {
        let mut shape = RectangleShape2D::new_gd();
        shape.set_size(Vector2::new(45.0, 45.0));
        let mut collision_shape = CollisionShape2D::new_alloc();
        collision_shape.set_shape(shape.upcast());
        let mut area = Area2D::new_alloc();
        area.add_child(collision_shape.upcast());
        area.connect(
            "mouse_entered".into(),
            self.base().callable("on_mouse_entered"),
        );
        area.connect(
            "mouse_exited".into(),
            self.base().callable("on_mouse_exited"),
        );

        self.base_mut().add_child(area.upcast());
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(event) = event.try_cast::<InputEventMouseButton>() {
            if event.get_button_index() == MouseButton::LEFT
                && self
                    .base()
                    .get_rect()
                    .has_point(self.base().to_local(event.get_position()))
            {
                let mut board_2d = self.base().get_parent().unwrap().cast::<ChessBoard2D>();
                let mut board_2d = board_2d.bind_mut();
                if event.is_pressed()
                    && board_2d.board.turn == board_2d.board.troops[self.index].unwrap().color
                {
                    self.is_held = true;
                    board_2d.pick(self);
                }
                if event.is_released() && self.is_held {
                    self.is_held = false;
                    board_2d.place(self, event.get_position());
                }
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.is_held {
            let mouse_position = self.base().get_viewport().unwrap().get_mouse_position();
            self.base_mut().set_position(mouse_position);
        }
        if self.hovered {
            let mut server = DisplayServer::singleton();
            server.cursor_set_shape(CursorShape::POINTING_HAND);
        }
    }
}

#[godot_api]
impl ChessPiece {
    #[func]
    fn on_mouse_entered(&mut self) {
        let board = self.base().get_parent().unwrap().cast::<ChessBoard2D>();
        let board = &board.bind().board;
        if board.turn == board.troops[self.index].unwrap().color {
            self.hovered = true;
        }
    }

    #[func]
    fn on_mouse_exited(&mut self) {
        self.hovered = false;
    }
}
