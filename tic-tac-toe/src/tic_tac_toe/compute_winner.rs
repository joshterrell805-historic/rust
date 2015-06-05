use tic_tac_toe::board::*;

pub fn compute_winner(board : &Board) -> Player {
  let diag = compute_diag_winner(board);
  if diag != Player::None {
    return diag;
  }
  let vert = compute_vert_winner(board);
  if vert != Player::None {
    return vert;
  }
  return compute_horiz_winner(board);
}

/******************************************************************************/

fn compute_vert_winner(board : &Board) -> Player {
  let none = (Mark::None, Player::None);
  let mut winner = &none;

  'i: for i in 0..BOARD_SIZE {
    winner = &board.moves[i][0];

    for j in 1..BOARD_SIZE {
      if *winner != board.moves[i][j] {
        winner = &none;
        continue 'i;
      }
    }

    if winner != &none {
      break;
    }
  }

  return winner.1;
}

fn compute_horiz_winner(board : &Board) -> Player {
  let none = (Mark::None, Player::None);
  let mut winner = &none;

  'j: for j in 0..BOARD_SIZE {
    winner = &board.moves[0][j];

    for i in 1..BOARD_SIZE {
      if *winner != board.moves[i][j] {
        winner = &none;
        continue 'j;
      }
    }

    if winner != &none {
      break;
    }
  }

  return winner.1;
}

fn compute_diag_winner(board : &Board) -> Player {
  let none = (Mark::None, Player::None);
  let mut top_to_bottom = &board.moves[0][0];
  let mut bottom_to_top = &board.moves[0][BOARD_SIZE - 1];

  for i in 1..BOARD_SIZE {
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

/******************************************************************************/

#[test]
fn test_compute_winner() {
  let cases = [
    ([(' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),], Player::None),
    // diag
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
    // vert
    ([('x',1),(' ',0),(' ',0),
      ('x',1),(' ',0),(' ',0),
      ('x',1),(' ',0),(' ',0),], Player::One),
    ([(' ',0),('x',1),(' ',0),
      (' ',0),('x',1),(' ',0),
      (' ',0),('x',1),(' ',0),], Player::One),
    ([(' ',0),(' ',0),('x',1),
      (' ',0),(' ',0),('x',1),
      (' ',0),(' ',0),('x',1),], Player::One),
    ([(' ',0),(' ',0),('x',1),
      (' ',0),(' ',0),('x',2),
      (' ',0),(' ',0),('x',1),], Player::None),
    ([(' ',0),(' ',0),('o',1),
      (' ',0),(' ',0),('o',1),
      (' ',0),(' ',0),('o',1),], Player::One),
    ([(' ',0),(' ',0),('o',2),
      (' ',0),(' ',0),('o',2),
      (' ',0),(' ',0),('o',2),], Player::Two),
    // horiz
    ([('x',2),('x',2),('x',2),
      (' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),], Player::Two),
    ([(' ',0),(' ',0),(' ',0),
      ('x',2),('x',2),('x',2),
      (' ',0),(' ',0),(' ',0),], Player::Two),
    ([(' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),
      ('x',2),('x',2),('x',2),], Player::Two),
    ([(' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),
      ('x',2),('x',1),('x',2),], Player::None),
    ([(' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),
      ('x',2),('o',2),('o',2),], Player::None),
    ([(' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),
      ('o',2),('o',2),('o',2),], Player::Two),
    ([(' ',0),(' ',0),(' ',0),
      (' ',0),(' ',0),(' ',0),
      ('o',1),('o',1),('o',1),], Player::One),
  ];

  for case in &cases {
    let board = Board {
      moves: [[get_move_from_tuple(&case.0[0]), get_move_from_tuple(&case.0[3]),
          get_move_from_tuple(&case.0[6]),], [get_move_from_tuple(&case.0[1]),
          get_move_from_tuple(&case.0[4]), get_move_from_tuple(&case.0[7]),],
          [get_move_from_tuple(&case.0[2]), get_move_from_tuple(&case.0[5]),
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
