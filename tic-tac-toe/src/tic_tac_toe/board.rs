pub const BOARD_SIZE : usize = 3;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Mark {
  None,
  O,
  X,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
  None,
  One,
  Two,
}

#[derive(Debug)]
pub struct Board {
  pub moves : [[(Mark, Player) ; BOARD_SIZE] ; BOARD_SIZE],
}
