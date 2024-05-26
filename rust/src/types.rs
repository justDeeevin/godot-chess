use std::{
    fmt::{Debug, Display},
    ops::Not,
};

#[derive(Clone, Copy, PartialEq, Eq)]
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

/// Precalculated.
/// For each square, `[north, south, west, east, northwest, southeast, northeast, southwest]`
const NUM_SQUARES_TO_EDGE: [[i8; 8]; 64] = [
    [0, 7, 0, 7, 0, 7, 0, 0],
    [0, 7, 1, 6, 0, 6, 0, 1],
    [0, 7, 2, 5, 0, 5, 0, 2],
    [0, 7, 3, 4, 0, 4, 0, 3],
    [0, 7, 4, 3, 0, 3, 0, 4],
    [0, 7, 5, 2, 0, 2, 0, 5],
    [0, 7, 6, 1, 0, 1, 0, 6],
    [0, 7, 7, 0, 0, 0, 0, 7],
    [1, 6, 0, 7, 0, 6, 1, 0],
    [1, 6, 1, 6, 1, 6, 1, 1],
    [1, 6, 2, 5, 1, 5, 1, 2],
    [1, 6, 3, 4, 1, 4, 1, 3],
    [1, 6, 4, 3, 1, 3, 1, 4],
    [1, 6, 5, 2, 1, 2, 1, 5],
    [1, 6, 6, 1, 1, 1, 1, 6],
    [1, 6, 7, 0, 1, 0, 0, 6],
    [2, 5, 0, 7, 0, 5, 2, 0],
    [2, 5, 1, 6, 1, 5, 2, 1],
    [2, 5, 2, 5, 2, 5, 2, 2],
    [2, 5, 3, 4, 2, 4, 2, 3],
    [2, 5, 4, 3, 2, 3, 2, 4],
    [2, 5, 5, 2, 2, 2, 2, 5],
    [2, 5, 6, 1, 2, 1, 1, 5],
    [2, 5, 7, 0, 2, 0, 0, 5],
    [3, 4, 0, 7, 0, 4, 3, 0],
    [3, 4, 1, 6, 1, 4, 3, 1],
    [3, 4, 2, 5, 2, 4, 3, 2],
    [3, 4, 3, 4, 3, 4, 3, 3],
    [3, 4, 4, 3, 3, 3, 3, 4],
    [3, 4, 5, 2, 3, 2, 2, 4],
    [3, 4, 6, 1, 3, 1, 1, 4],
    [3, 4, 7, 0, 3, 0, 0, 4],
    [4, 3, 0, 7, 0, 3, 4, 0],
    [4, 3, 1, 6, 1, 3, 4, 1],
    [4, 3, 2, 5, 2, 3, 4, 2],
    [4, 3, 3, 4, 3, 3, 4, 3],
    [4, 3, 4, 3, 4, 3, 3, 3],
    [4, 3, 5, 2, 4, 2, 2, 3],
    [4, 3, 6, 1, 4, 1, 1, 3],
    [4, 3, 7, 0, 4, 0, 0, 3],
    [5, 2, 0, 7, 0, 2, 5, 0],
    [5, 2, 1, 6, 1, 2, 5, 1],
    [5, 2, 2, 5, 2, 2, 5, 2],
    [5, 2, 3, 4, 3, 2, 4, 2],
    [5, 2, 4, 3, 4, 2, 3, 2],
    [5, 2, 5, 2, 5, 2, 2, 2],
    [5, 2, 6, 1, 5, 1, 1, 2],
    [5, 2, 7, 0, 5, 0, 0, 2],
    [6, 1, 0, 7, 0, 1, 6, 0],
    [6, 1, 1, 6, 1, 1, 6, 1],
    [6, 1, 2, 5, 2, 1, 5, 1],
    [6, 1, 3, 4, 3, 1, 4, 1],
    [6, 1, 4, 3, 4, 1, 3, 1],
    [6, 1, 5, 2, 5, 1, 2, 1],
    [6, 1, 6, 1, 6, 1, 1, 1],
    [6, 1, 7, 0, 6, 0, 0, 1],
    [7, 0, 0, 7, 0, 0, 7, 0],
    [7, 0, 1, 6, 1, 0, 6, 0],
    [7, 0, 2, 5, 2, 0, 5, 0],
    [7, 0, 3, 4, 3, 0, 4, 0],
    [7, 0, 4, 3, 4, 0, 3, 0],
    [7, 0, 5, 2, 5, 0, 2, 0],
    [7, 0, 6, 1, 6, 0, 1, 0],
    [7, 0, 7, 0, 7, 0, 0, 0],
];

/// `[north, south, west, east, northwest, southeast, northeast, southwest]`
const DIRECTION_OFFSETS: [i8; 8] = [-8, 8, -1, 1, -9, 9, -7, 7];

fn square_to_index(square: (char, u8)) -> Result<usize, String> {
    let rank;
    if square.1 > 8 {
        return Err(format!("Rank {} is too large", square.1));
    } else if square.1 < 1 {
        return Err(format!("Rank {} is too small", square.1));
    } else {
        rank = 8 - square.1;
    }

    let file = match square.0 {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return Err(format!("Invalid file {}", square.0)),
    };

    Ok(((8 * rank) + file) as usize)
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

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
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
    pub en_passant_target: Option<usize>,
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

        let mut en_passant_target = None;
        if fields[3] != "-" {
            let mut churn = fields[3].chars().enumerate();
            while let Some((i, c)) = churn.next() {
                let file = c;
                let rank = churn
                    .next()
                    .ok_or(format!("Invalid square at {}", i))?
                    .1
                    .to_digit(10)
                    .ok_or(format!("Invalid rank at {}", i + 1))? as u8;
                en_passant_target = Some(square_to_index((file, rank))?);
            }
        }

        let mut board = Self {
            troops: [None; 64],
            turn,
            castling_rights,
            en_passant_target,
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

    pub fn moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();

        for (i, troop) in self.troops.iter().enumerate() {
            if let Some(troop) = troop {
                if troop.color == self.turn {
                    match troop.piece {
                        Piece::Rook | Piece::Bishop | Piece::Queen => {
                            moves.extend(self.generate_sliding_moves(i, troop.piece));
                        }
                        Piece::Pawn => {
                            moves.extend(self.generate_pawn_moves(i));
                        }
                        Piece::Knight => {
                            moves.extend(self.generate_knight_moves(i));
                        }
                        _ => {}
                    }
                }
            }
        }

        moves
            .into_iter()
            .filter(|m| {
                !(self.troops[m.end].is_some() && self.troops[m.end].unwrap().color == self.turn)
            })
            .collect()
    }

    fn generate_sliding_moves(&self, start: usize, piece: Piece) -> Vec<Move> {
        let mut moves = Vec::new();

        let start_direction = if piece == Piece::Bishop { 4 } else { 0 };
        let end_direction = if piece == Piece::Rook { 4 } else { 8 };

        for (direction, direction_offset) in DIRECTION_OFFSETS
            .iter()
            .enumerate()
            .take(end_direction)
            .skip(start_direction)
        {
            for n in 0..NUM_SQUARES_TO_EDGE[start][direction] {
                let target = (start as i8 + (direction_offset * (n + 1))) as usize;
                let troop = self.troops[target];

                moves.push(Move { start, end: target });

                if troop.is_some() {
                    break;
                }
            }
        }

        moves
    }

    fn generate_pawn_moves(&self, start: usize) -> Vec<Move> {
        let mut moves = Vec::new();

        let direction = if self.turn == Color::White { 0 } else { 1 };

        if self.troops[(start as i8 + DIRECTION_OFFSETS[direction]) as usize].is_none() {
            moves.push(Move {
                start,
                end: (start as i8 + DIRECTION_OFFSETS[direction]) as usize,
            });

            if (start / 8 == 1 && self.turn == Color::Black)
                || (start / 8 == 6 && self.turn == Color::White)
                    && self.troops[(start as i8 + DIRECTION_OFFSETS[direction] * 2) as usize]
                        .is_none()
            {
                moves.push(Move {
                    start,
                    end: (start as i8 + (DIRECTION_OFFSETS[direction] * 2)) as usize,
                });
            }
        }

        let diagonals = [
            (start as i8 + DIRECTION_OFFSETS[direction + 4]) as usize,
            (start as i8 + DIRECTION_OFFSETS[direction + 6]) as usize,
        ];

        for diagonal in diagonals {
            if self.troops[diagonal].is_some()
                || (self.troops[diagonal].is_none() && self.en_passant_target == Some(diagonal))
            {
                moves.push(Move {
                    start,
                    end: diagonal,
                });
            }
        }

        moves.into_iter().filter(|m| m.end < 64).collect()
    }

    fn generate_knight_moves(&self, start: usize) -> Vec<Move> {
        let mut moves = Vec::new();

        for (i, cardinal) in DIRECTION_OFFSETS.into_iter().enumerate().take(4) {
            let first_move = cardinal * 2;
            if i < 2 {
                let y_delta = first_move / 8;
                if (y_delta < 0 && y_delta.abs() <= NUM_SQUARES_TO_EDGE[start][0])
                    || (y_delta > 0 && y_delta.abs() <= NUM_SQUARES_TO_EDGE[start][1])
                {
                    for second_move in DIRECTION_OFFSETS.into_iter().take(4).skip(2) {
                        let x_delta = second_move % 8;
                        if (x_delta < 0 && x_delta.abs() <= NUM_SQUARES_TO_EDGE[start][2])
                            || (x_delta > 0 && x_delta.abs() <= NUM_SQUARES_TO_EDGE[start][3])
                        {
                            moves.push(Move {
                                start,
                                end: (start as i8 + first_move + second_move) as usize,
                            });
                        }
                    }
                }
            } else {
                let x_delta = first_move % 8;
                if (x_delta < 0 && x_delta.abs() <= NUM_SQUARES_TO_EDGE[start][2])
                    || (x_delta > 0 && x_delta.abs() <= NUM_SQUARES_TO_EDGE[start][3])
                {
                    for second_move in DIRECTION_OFFSETS.into_iter().take(2) {
                        let y_delta = second_move / 8;
                        if (y_delta < 0 && y_delta.abs() <= NUM_SQUARES_TO_EDGE[start][0])
                            || (y_delta > 0 && y_delta.abs() <= NUM_SQUARES_TO_EDGE[start][1])
                        {
                            moves.push(Move {
                                start,
                                end: (start as i8 + first_move + second_move) as usize,
                            });
                        }
                    }
                }
            }
        }

        moves
            .into_iter()
            .filter(|m| {
                m.end < 64
                    && !(self.troops[m.end].is_some()
                        && self.troops[m.end].unwrap().color == self.turn)
            })
            .collect()
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
impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub start: usize,
    pub end: usize,
}
