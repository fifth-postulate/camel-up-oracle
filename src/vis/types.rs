//! Types
#![allow(missing_docs)]
use crate::prelude::Race;
use crate::prelude::Camel;
use crate::camel::Marker;

/// The name is rather self explanatory innit
pub const BOARD_SIZE: usize = 16;

pub type CamelUnit = Vec<Camel>;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Player {
  SaddamHussain,
  StuckUpLady,
  TheScientist,
  Prophet,
  Eyebrows,
  PaulSpencer,
  PrinceAli,
  BobbyTheBooky,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TrapType {
  /// When camels land on an oasis they advance one position.
  Oasis,
  /// When camels land on a fata morgana, they fallback one position.
  FataMorgana,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Trap {
  trap_type: TrapType,
  player: Player,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Tile {
  Camels(CamelUnit),
  Trap(Trap),
  Nothing,
}

impl Default for Tile {
  fn default() -> Self { Tile::Nothing }
}

/// A complete board
pub struct Board {
  pub tiles: [Tile; BOARD_SIZE]
}

impl Board {
  pub fn new() -> Self {
    Board { tiles: Default::default() }
  }
}

impl From<&Race> for Board {
  fn from(race: &Race) -> Self {
    match race.positions.last() {
      None => panic!("Race must have at least one element"),
      Some(&x) => if x != Marker::Finish {
        panic!("Last element of Race must be a finish");
      }
    }

    let mut tiles: [Tile; BOARD_SIZE] = Default::default();
    let mut i = BOARD_SIZE - 1;
    for marker in race.positions.iter().rev() {
      let mutation = match marker {
        Marker::Camel(camel) => Some(prepend_camel_to_tile(*camel, &tiles[i])),
        Marker::Divider => {
          i -= 1;
          None
        },
        Marker::Oasis => Some(Tile::Trap(Trap { trap_type: TrapType::Oasis, player: Player::SaddamHussain })),
        Marker::FataMorgana => Some(Tile::Trap(Trap { trap_type: TrapType::FataMorgana, player: Player::SaddamHussain })),
        Marker::Finish => None,
      };
      match mutation {
        None => (),
        Some(new_tile) => tiles[i] = new_tile,
      }
    }

    Board { tiles }
  }
}

/// Prepend a camel unit to a tile
///
/// If the tile does not already contain a CamelUnit,
/// return a single-sized camel unit
fn prepend_camel_to_tile(camel: Camel, tile: &Tile) -> Tile {
  match tile {
    Tile::Camels(ref unit) => Tile::Camels(prepend_camel_to_unit(camel, unit.clone())),
    _ => Tile::Camels(vec!(camel)),
  }
}

/// Prepend a camel to a camel unit
fn prepend_camel_to_unit(camel: Camel, mut unit: CamelUnit) -> CamelUnit {
  unit.insert(0, camel);
  unit
}
