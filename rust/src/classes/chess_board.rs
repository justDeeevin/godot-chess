use crate::{
    classes::ChessPiece,
    types::{self, Board, Move, Piece},
};
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
    pub board: Board,
    #[export]
    /// If blank, the normal starting position is used.
    starting_fen: GString,
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
            starting_fen: "".into(),
            pieces: vec![None; 64],
            last_picked: 0,
            last_placed: 0,
            current_picked: 0,
            base,
        }
    }

    fn ready(&mut self) {
        let fen = if self.starting_fen.is_empty() {
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()
        } else {
            self.starting_fen.clone().to_string()
        };

        self.board = Board::from_fen(&fen).unwrap();

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
                if self.board.moves().contains(&Move {
                    start: self.current_picked,
                    end: i,
                }) || i == self.current_picked
                    || (!matches!(
                        self.board.troops[self.current_picked].unwrap().piece,
                        Piece::Rook | Piece::Bishop | Piece::Queen
                    ) && self.board.troops[self.current_picked].unwrap().piece != Piece::Pawn)
                {
                    // Drop piece on the square
                    piece.base_mut().set_position(
                        square.get_position()
                            + Vector2::new(self.square_size / 2.0, self.square_size / 2.0),
                    );
                    piece.index = i;

                    // Clear the last move's highlights
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

                    // Highlight the new placed square
                    let mut picked_square = self.squares[self.current_picked].clone();
                    let new_picked_color = picked_square
                        .get_modulate()
                        .lerp(Color::YELLOW.darkened(0.5), 0.5);
                    picked_square.set_modulate(new_picked_color);

                    // Make sure to clear any red before lerping
                    let red = if is_index_dark(i) {
                        self.squares[i].get_modulate() != self.dark_color
                    } else {
                        self.squares[i].get_modulate() != Color::WHITE
                    };
                    if red {
                        if is_index_dark(i) {
                            self.squares[i].set_modulate(self.dark_color);
                        } else {
                            self.squares[i].set_modulate(Color::WHITE);
                        }
                    }

                    let mut placed_square = self.squares[i].clone();
                    let new_color = placed_square.get_modulate().lerp(Color::YELLOW, 0.5);
                    placed_square.set_modulate(new_color);
                    self.last_placed = i;

                    // Capture, if necessary, and update the board's state
                    self.pieces[self.current_picked] = None;
                    let picked_troop = self.board.troops[self.current_picked].unwrap();
                    self.board.troops[self.current_picked] = None;
                    if let Some(ref mut piece) = &mut self.pieces[i] {
                        piece.queue_free();
                    }
                    self.pieces[i] = Some(piece.base().clone().cast());
                    self.board.troops[i] = Some(picked_troop);
                    if i != self.current_picked {
                        self.board.turn = !self.board.turn;
                    }

                    // En passant target check
                    if self.board.troops[i].unwrap().piece == Piece::Pawn
                        && (i as i8 - self.current_picked as i8).abs() == 16
                    {
                        let target = (i as i8
                            + match self.board.turn {
                                types::Color::White => -8,
                                types::Color::Black => 8,
                            }) as usize;
                        self.board.en_passant_target = Some(target);
                    }

                    // En passant capture check
                    if self.board.en_passant_target == Some(i) {
                        let piece_index = (i as i8
                            + match self.board.turn {
                                types::Color::White => -8,
                                types::Color::Black => 8,
                            }) as usize;
                        let mut piece = self.pieces[piece_index].clone().unwrap();
                        self.pieces[piece_index] = None;
                        piece.queue_free();
                    }

                    break;
                } else {
                    self.place(piece, self.squares[self.current_picked].get_position());
                    break;
                }
            }
        }
        self.squares.iter_mut().enumerate().for_each(|(i, s)| {
            let color = if is_index_dark(i) {
                self.dark_color
            } else {
                Color::WHITE
            };
            if i != self.current_picked && i != self.last_placed && i != self.last_picked {
                s.set_modulate(color);
            }
        });
    }

    pub fn pick(&mut self, piece: &ChessPiece) {
        let mut square = self.squares[piece.index].clone();
        let new_color = square.get_modulate().lerp(Color::YELLOW, 0.5);
        square.set_modulate(new_color);
        self.base_mut()
            .move_child(piece.base().clone().upcast(), -1);
        self.last_picked = self.current_picked;
        self.current_picked = piece.index;
        for i in self.board.moves().iter_mut().filter_map(|m| {
            if m.start == self.current_picked {
                Some(m.end)
            } else {
                None
            }
        }) {
            let mut square = self.squares[i].clone();
            let dark = is_index_dark(i);
            let yellow = if dark {
                square.get_modulate() != self.dark_color
            } else {
                square.get_modulate() != Color::WHITE
            };
            if yellow {
                if dark {
                    square.set_modulate(self.dark_color);
                } else {
                    square.set_modulate(Color::WHITE);
                }
            }
            let new_color = square.get_modulate().lerp(Color::RED, 0.5);
            square.set_modulate(new_color);
        }
    }
}

fn is_index_dark(index: usize) -> bool {
    let rank = index / 8;
    let file = index % 8;
    rank % 2 != file % 2
}
