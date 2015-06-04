const BOARD_SIZE : usize = 3;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Mark {
  None,
  O,
  X,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Player {
  None,
  One,
  Two,
}

#[derive(Debug)]
struct Board {
  moves : [[(Mark, Player) ; 3] ; 3],
}

fn main() {
  // let mut board = Board {
  //   moves: [(Mark::None, Player::One) ; 9],
  // };
  // board.moves[0] = (Mark::None, Player::Two);
}

fn compute_winner(board : &Board) -> Player {
  compute_diag_winner(board)
}

fn compute_diag_winner(board : &Board) -> Player {
  let none = (Mark::None, Player::None);
  let mut top_to_bottom = &board.moves[0][0];
  let mut bottom_to_top = &board.moves[0][BOARD_SIZE - 1];

  for i in 1..3 {
    if *top_to_bottom != board.moves[i][i] {
      top_to_bottom = &none;
    }
    if *bottom_to_top != board.moves[i][BOARD_SIZE - 1 - i] {
      bottom_to_top = &none;
    }
  }

  if top_to_bottom.1 != Player::None {
    top_to_bottom.1
  } else {
    bottom_to_top.1
  }
}

#[test]
fn test_compute_winner() {
  let cases = [
    ([(' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),], Player::None),
    ([('x',1),(' ',0),(' ',0),
      (' ',0),('x',1),(' ',0),
      (' ',0),(' ',0),('x',1),], Player::One),
    ([('x',2),(' ',0),(' ',0),
      (' ',0),('x',1),(' ',0),
      (' ',0),(' ',0),('x',1),], Player::None),
    ([('x',1),(' ',0),(' ',0),
      (' ',0),('x',2),(' ',0),
      (' ',0),(' ',0),('x',1),], Player::None),
    ([('x',1),(' ',0),(' ',0),
      (' ',0),('x',1),(' ',0),
      (' ',0),(' ',0),('x',2),], Player::None),
    ([('x',1),(' ',0),(' ',0),
      (' ',0),('x',1),(' ',0),
      (' ',0),(' ',0),('o',1),], Player::None),
    ([('o',1),(' ',0),(' ',0),
      (' ',0),('x',1),(' ',0),
      (' ',0),(' ',0),('x',1),], Player::None),
    ([('x',2),(' ',0),(' ',0),
      (' ',0),('x',2),(' ',0),
      (' ',0),(' ',0),('x',2),], Player::Two),
    ([(' ',0),(' ',0),('x',2),
      (' ',0),('x',2),(' ',0),
      ('x',2),(' ',0),(' ',0),], Player::Two),
  ];

  for case in &cases {
    let board = Board {
      moves: [[get_move_from_tuple(&case.0[0]), get_move_from_tuple(&case.0[1]),
          get_move_from_tuple(&case.0[2]),], [get_move_from_tuple(&case.0[3]),
          get_move_from_tuple(&case.0[4]), get_move_from_tuple(&case.0[7]),],
          [get_move_from_tuple(&case.0[6]), get_move_from_tuple(&case.0[8]),
          get_move_from_tuple(&case.0[8]),],],
    };
    assert_eq!(compute_winner(&board), case.1);
  }

  fn get_move_from_tuple(tup : &(char, usize)) -> (Mark, Player) {
    let mark = match tup.0 {
      ' ' => Mark::None,
      'x' => Mark::X,
      'o' => Mark::O,
      _ => panic!("invalid mark!"),
    };
    let player = match tup.1 {
      0 => Player::None,
      1 => Player::One,
      2 => Player::Two,
      _ => panic!("invalid player!"),
    };

    (mark, player)
  }
}
