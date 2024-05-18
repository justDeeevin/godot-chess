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
    pieces: Vec<Option<Gd<ChessPiece>>>,
    last_picked: usize,
    last_placed: usize,
    current_picked: usize,
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
            pieces: vec![None; 64],
            last_picked: 0,
            last_placed: 0,
            current_picked: 0,
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
                piece.bind_mut().index = i;
                self.pieces[i] = Some(piece.clone());
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
        for (i, square) in self.squares.iter().enumerate() {
            if square.get_rect().has_point(square.to_local(position)) {
                piece.base_mut().set_position(
                    square.get_position()
                        + Vector2::new(self.square_size / 2.0, self.square_size / 2.0),
                );
                piece.index = i;

                let last_picked_color = if is_index_dark(self.last_picked) {
                    self.dark_color
                } else {
                    Color::WHITE
                };
                self.squares[self.last_picked].set_modulate(last_picked_color);

                let last_placed_color = if is_index_dark(self.last_placed) {
                    self.dark_color
                } else {
                    Color::WHITE
                };
                self.squares[self.last_placed].set_modulate(last_placed_color);

                let mut picked_square = self.squares[self.current_picked].clone();
                let new_picked_color = picked_square
                    .get_modulate()
                    .lerp(Color::YELLOW.darkened(0.5), 0.5);
                picked_square.set_modulate(new_picked_color);

                let mut placed_square = self.squares[i].clone();
                let new_color = placed_square.get_modulate().lerp(Color::YELLOW, 0.5);
                placed_square.set_modulate(new_color);
                self.last_placed = i;

                self.pieces[self.current_picked] = None;
                if let Some(ref mut piece) = &mut self.pieces[i] {
                    piece.queue_free();
                }
                self.pieces[i] = Some(piece.base().clone().cast());

                break;
            }
        }
    }

    pub fn pick(&mut self, piece: &ChessPiece) {
        let mut square = self.squares[piece.index].clone();
        let new_color = square.get_modulate().lerp(Color::YELLOW, 0.5);
        square.set_modulate(new_color);
        self.base_mut()
            .move_child(piece.base().clone().upcast(), -1);
        self.last_picked = self.current_picked;
        self.current_picked = piece.index
    }
}

fn is_index_dark(index: usize) -> bool {
    let rank = index / 8;
    let file = index % 8;
    rank % 2 != file % 2
}
