//! Rendering routines
#![allow(missing_docs)]
use ansi_term::ANSIString;
use crate::prelude::Camel;
use crate::vis::types::BOARD_SIZE;
use crate::vis::types::Board;
use crate::vis::types::Tile;
use crate::vis::types::Trap;
use crate::vis::types::TrapType;
use crate::vis::types::Player;

use ansi_term::Color;

const CAMEL_COUNT: usize = 5;

pub fn render_board(board: &Board) -> Vec<String> {
  let mut screen: [[String; BOARD_SIZE + 1]; CAMEL_COUNT] = Default::default();

  for (i, tile) in board.tiles.iter().enumerate() {
    match tile {
      Tile::Nothing => (),
      Tile::Trap(t) => screen[0][i] = format!("{}", render_trap(t)),
      Tile::Camels(camels) => {
        for (j, camel) in camels.iter().enumerate() {
          screen[j][i] = format!("{}", render_camel(camel));
        }
      }
    }
  }

  for i in 0..CAMEL_COUNT {
    screen[i][BOARD_SIZE] = format!("{}", Color::White.paint("┇"));
  }

  let mut ret = Vec::with_capacity(CAMEL_COUNT + 1);

  for i in (0..CAMEL_COUNT).rev() {
    ret.push(screen[i].iter().map(|s| format!("  {} ", s)).collect::<String>());
  }

  ret.push((0..16).map(|i| format!(" {:2} ", i + 1)).collect::<String>());

  ret
}

pub fn print_board(board: &Board) {
  for line in render_board(board) {
    println!("{}", line);
  }
}


fn render_trap(trap: &Trap) -> ANSIString {
  player_color(trap.player).bold().paint(match trap.trap_type {
    TrapType::Oasis => "+",
    TrapType::FataMorgana => "-",
  })
}

fn render_camel(camel: &Camel) -> ANSIString {
  camel_color(*camel).bold().paint("m")
}

fn player_color(player: Player) -> Color {
  // https://robotmoon.com/256-colors/
  match player {
    Player::SaddamHussain => Color::Fixed(253),
    Player::BobbyTheBooky => Color::Fixed(94),
    Player::Eyebrows => Color::Fixed(160),
    Player::PaulSpencer => Color::Fixed(208),
    Player::PrinceAli => Color::Fixed(221),
    Player::Prophet => Color::Fixed(90),
    Player::StuckUpLady => Color::Fixed(199),
    Player::TheScientist => Color::Fixed(57),
  }
}

fn camel_color(camel: Camel) -> Color {
  match camel {
    Camel::Green => Color::Green,
    Camel::Orange => Color::Fixed(208),
    Camel::Red => Color::Red,
    Camel::White => Color::White,
    Camel::Yellow => Color::Yellow,
  }
}