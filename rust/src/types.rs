use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Piece::King => "King",
                Piece::Queen => "Queen",
                Piece::Rook => "Rook",
                Piece::Bishop => "Bishop",
                Piece::Knight => "Knight",
                Piece::Pawn => "Pawn",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Black => "Black",
                Color::White => "White",
            }
        )
    }
}

#[derive(Clone, Copy)]
pub struct Troop {
    pub color: Color,
    pub piece: Piece,
}
impl std::fmt::Debug for Troop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut piece = match self.piece {
            Piece::King => 'k',
            Piece::Queen => 'q',
            Piece::Rook => 'r',
            Piece::Bishop => 'b',
            Piece::Knight => 'n',
            Piece::Pawn => 'p',
        };
        if self.color == Color::White {
            piece = piece.to_ascii_uppercase();
        }
        write!(f, "{}", piece)
    }
}
impl Display for Troop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.color)?;
        write!(f, "{}", self.piece)
    }
}

pub struct Board {
    pub troops: [Option<Troop>; 64],
    pub turn: Color,
    pub castling_rights: CastlingRights,
}

pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl Board {
    pub fn starting() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let fields = fen.split(' ').collect::<Vec<&str>>();
        if fields.len() != 6 {
            return Err("Invalid number of fields".to_string());
        }

        let turn_char = fields[1];
        let turn = match turn_char {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err("Invalid turn character".to_string()),
        };

        let castling_rights = CastlingRights {
            white_king_side: fields[2].contains('K'),
            white_queen_side: fields[2].contains('Q'),
            black_king_side: fields[2].contains('k'),
            black_queen_side: fields[2].contains('q'),
        };

        let mut board = Self {
            troops: [None; 64],
            turn,
            castling_rights,
        };

        let rows = fields[0].split('/').collect::<Vec<&str>>();
        if rows.len() != 8 {
            return Err("Not enough rows".to_string());
        }
        for (i, row) in rows.iter().enumerate() {
            let mut troops = Vec::new();

            for c in row.chars() {
                let color = if c.is_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };

                match c.to_lowercase().next().unwrap() {
                    'k' => troops.push(Some(Troop {
                        color,
                        piece: Piece::King,
                    })),
                    'q' => troops.push(Some(Troop {
                        color,
                        piece: Piece::Queen,
                    })),
                    'r' => troops.push(Some(Troop {
                        color,
                        piece: Piece::Rook,
                    })),
                    'b' => troops.push(Some(Troop {
                        color,
                        piece: Piece::Bishop,
                    })),
                    'n' => troops.push(Some(Troop {
                        color,
                        piece: Piece::Knight,
                    })),
                    'p' => troops.push(Some(Troop {
                        color,
                        piece: Piece::Pawn,
                    })),
                    '1'..='8' => {
                        let n = c.to_digit(10).unwrap() as usize;
                        for _ in 0..n {
                            troops.push(None);
                        }
                    }
                    _ => return Err(format!("Invalid character {}", c)),
                }
            }

            if troops.len() != 8 {
                return Err(format!("Not enough troops in row {}", i));
            }

            board.troops[i * 8..(i + 1) * 8].copy_from_slice(&troops);
        }

        Ok(board)
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.troops.chunks(8) {
            write!(f, "|")?;
            for troop in row {
                let char = troop.map(|t| format!("{:?}", t)).unwrap_or(" ".into());
                write!(f, "{}|", char)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        writeln!(f, "{} to move", self.turn)?;
        if self.castling_rights.white_king_side {
            write!(f, " (K)")?;
        }
        if self.castling_rights.white_queen_side {
            write!(f, " (Q)")?;
        }
        if self.castling_rights.black_king_side {
            write!(f, " (k)")?;
        }
        if self.castling_rights.black_queen_side {
            write!(f, " (q)")?;
        }
        Ok(())
    }
}
