use crate::{classes::ChessPiece, types::Board};
use godot::{
    engine::{CanvasGroup, ICanvasGroup, Sprite2D, Texture2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base = CanvasGroup)]
pub struct ChessBoard2D {
    #[export]
    dark_color: Color,
    #[export]
    square_size: f32,
    board: Board,
    squares: Vec<Gd<Sprite2D>>,
    base: Base<CanvasGroup>,
}

#[godot_api]
impl ICanvasGroup for ChessBoard2D {
    fn init(base: Base<CanvasGroup>) -> Self {
        Self {
            dark_color: Color::from_html("#1A4F42").unwrap(),
            square_size: 70.0,
            board: Board::starting(),
            squares: Vec::new(),
            base,
        }
    }

    fn ready(&mut self) {
        for rank in 0..8 {
            for file in 0..8 {
                let dark = file % 2 != rank % 2;
                self.add_square(
                    dark,
                    file as f32 * self.square_size,
                    rank as f32 * self.square_size,
                );
            }
        }

        let troops = self.board.troops;
        for (i, troop) in troops.iter().enumerate() {
            if let Some(troop) = troop {
                let square = &self.squares[i];
                let texture = load::<Texture2D>(format!("res://art/Chess Pieces/{}.svg", troop));
                let mut piece = ChessPiece::new_alloc();
                piece.set_texture(texture);
                piece.set_scale(Vector2::new(
                    self.square_size / 50.0,
                    self.square_size / 50.0,
                ));
                piece.set_position(
                    square.get_position()
                        + Vector2::new(self.square_size / 2.0, self.square_size / 2.0),
                );
                self.base_mut().add_child(piece.upcast());
            }
        }
    }
}

#[godot_api]
impl ChessBoard2D {
    fn add_square(&mut self, dark: bool, x: f32, y: f32) {
        let texture = load::<Texture2D>("res://art/White Square.png");

        let mut sprite = Sprite2D::new_alloc();
        sprite.set_texture(texture);
        sprite.set_scale(Vector2::new(self.square_size, self.square_size));
        sprite.set_modulate(if dark { self.dark_color } else { Color::WHITE });
        sprite.set_position(Vector2::new(x, y));
        sprite.set_centered(false);

        self.squares.push(sprite.clone());
        self.base_mut().add_child(sprite.upcast());
    }

    pub fn place(&mut self, piece: &mut ChessPiece, position: Vector2) {
        for square in &self.squares {
            if square.get_rect().has_point(square.to_local(position)) {
                piece.base_mut().set_position(
                    square.get_position()
                        + Vector2::new(self.square_size / 2.0, self.square_size / 2.0),
                );
                break;
            }
        }
    }
}
