//! The camel module models Camel Up.
//!
//! It identifies all the relevant parts of the Camel Up game. Starting from a `Race` it allows you to perform a `Roll`, which will return the resulting race.
//!
//! ```
//! # use camel_up::camel::{Race, Marker, Camel, Roll, Face};
//! let race = Race::from(vec![Marker::Camel(Camel::Red), Marker::Divider, Marker::Camel(Camel::Yellow)]);
//! let roll = Roll::from((Camel::Red, Face::One));
//!
//! let actual = race.perform(roll);
//!
//! let expected = Race::from(vec![Marker::Camel(Camel::Yellow), Marker::Camel(Camel::Red)]);
//! assert_eq!(actual, expected);
//! ```
//!
//! One can cut down on the verbosity by using the various `parse` functions and other convenience functions. The above code example can be widdled down to
//!
//! ```
//! # use camel_up::camel::{Race, Marker, Camel, Roll, Face};
//! let race = "r,y".parse::<Race>().expect("to parse");
//!
//! let actual = race.perform((Camel::Red, Face::One));
//!
//! let expected = "yr".parse::<Race>().expect("to parse");
//! assert_eq!(actual, expected);
//! ```

use std::collections::HashSet;
use std::iter::repeat;
use std::str::FromStr;

/// The various camels that race in the game.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Camel {
    /// The red camel, Rachel for friends.
    Red,
    /// The orange camel, also known as Olleta.
    Orange,
    /// The yellow camel. They are a little shy. They go by the name of Yenn.
    Yellow,
    /// The green camel. The mysterious one, calls itself G.
    Green,
    /// The white camel. Responds to Whitney. Suspected to be a foreign spy.
    White,
}

/// A marker is used to describe a race.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Marker {
    /// Signals that a camel is present at this position. Its argument tells you which camel.
    Camel(Camel),
    /// Divider between positions.
    Divider,
    /// When camels land on an oasis they advance one position.
    Oasis,
    /// When camels land on a fata morgana, they fallback one position.
    FataMorgana,
}

impl Marker {
    fn is_a_camel(self) -> bool {
        match self {
            Marker::Camel(_) => true,
            _ => false,
        }
    }

    fn is_a_divider(self) -> bool {
        match self {
            Marker::Divider => true,
            _ => false,
        }
    }

    fn is_an_oasis(self) -> bool {
        match self {
            Marker::Oasis => true,
            _ => false,
        }
    }

    fn is_a_fata_morgana(self) -> bool {
        match self {
            Marker::FataMorgana => true,
            _ => false,
        }
    }

    fn is_an_adjustment(self) -> bool {
        self.is_an_oasis() || self.is_a_fata_morgana()
    }

    fn to_camel(self) -> Option<Camel> {
        match self {
            Marker::Camel(camel) => Some(camel),
            _ => None,
        }
    }
}

impl FromStr for Marker {
    type Err = NotAMarker;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "r" => Ok(Marker::Camel(Camel::Red)),
            "o" => Ok(Marker::Camel(Camel::Orange)),
            "y" => Ok(Marker::Camel(Camel::Yellow)),
            "g" => Ok(Marker::Camel(Camel::Green)),
            "w" => Ok(Marker::Camel(Camel::White)),
            "," => Ok(Marker::Divider),
            "+" => Ok(Marker::Oasis),
            "-" => Ok(Marker::FataMorgana),
            _ => Err(NotAMarker::But(input.to_owned())),
        }
    }
}

/// When parsing of Marker goes wrong, this enumeration tells you precisely what went down.
#[derive(PartialEq, Debug)]
pub enum NotAMarker {
    /// It was not a marker, but something else. The argument tells you what it was.
    But(String),
}

/// Models a race as a sequence of markers.
///
/// Note that a race is normalized, i.e. leading and trailing dividers are stripped.
///
/// ```
/// # use camel_up::camel::{Race, Marker, Camel};
/// let race_with_superfluous_dividers = ",,,,,,r,y,,,,,,,".parse::<Race>().expect("to parse");
/// let minimal_race = "r,y".parse::<Race>().expect("to parse");
///
/// assert_eq!(race_with_superfluous_dividers, minimal_race);
/// ```
#[derive(PartialEq, Eq, Debug)]
pub struct Race {
    positions: Vec<Marker>,
}

impl Clone for Race {
    fn clone(&self) -> Self {
        Self {
            positions: self.positions.to_vec(),
        }
    }
}

impl From<Vec<Marker>> for Race {
    fn from(positions: Vec<Marker>) -> Self {
        let (min, max) = positions
            .iter()
            .zip(0..)
            .filter(|(marker, _)| marker.is_a_camel())
            .map(|(_, index)| index)
            .fold(
                (core::usize::MAX, core::usize::MIN),
                |(minimum, maximum), index| (minimum.min(index), maximum.max(index)),
            );
        let positions = positions[min..=max]
            .iter()
            .skip_while(|marker| **marker == Marker::Divider)
            .cloned()
            .collect();
        Self { positions }
    }
}

impl FromStr for Race {
    type Err = RaceParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = vec![];
        let mut cursor = 0;
        while cursor < input.len() {
            result.push(input[cursor..=cursor].parse::<Marker>()?);
            cursor += 1;
        }
        if result
            .iter()
            .zip(result.iter().skip(1))
            .filter(|(l, r)| l.is_a_camel() && r.is_an_oasis() || l.is_an_oasis() && r.is_a_camel())
            .count()
            > 0
        {
            return Err(RaceParseError::CamelInOasis);
        }

        if result
            .iter()
            .zip(result.iter().skip(1))
            .filter(|(l, r)| l.is_a_camel() && r.is_a_fata_morgana() || l.is_a_fata_morgana() && r.is_a_camel())
            .count()
            > 0
        {
            return Err(RaceParseError::CamelInFataMorgana);
        }

        if result
            .iter()
            .zip(result.iter().skip(1))
            .filter(|(l, r)| l.is_an_adjustment() && r.is_an_adjustment())
            .count()
            > 0
        {
            return Err(RaceParseError::ToManyAdjustmentsInOnePosition);
        }

        if result
            .iter()
            .zip(result.iter().skip(2))
            .filter(|(l, r)| l.is_an_adjustment() && r.is_an_adjustment())
            .count()
            > 0
        {
            return Err(RaceParseError::ConsecutiveAdjustments);
        }

        Ok(Race::from(result))
    }
}

/// When parsing of Race goes wrong, this enumeration tells you precisely what went down.
#[derive(PartialEq, Debug)]
pub enum RaceParseError {
    /// a race consists solely of markers, and this isn't a marker.
    NotAMarker(NotAMarker),
    /// a camel can't be in an oasis.
    CamelInOasis,
    /// a camel can't be in a fata morgana.
    CamelInFataMorgana,
    /// adjustments can't be in the same position.
    ToManyAdjustmentsInOnePosition,
    /// adjustments can't be consecutive.
    ConsecutiveAdjustments,
}

impl From<NotAMarker> for RaceParseError {
    fn from(problem: NotAMarker) -> Self {
        Self::NotAMarker(problem)
    }
}

/// A roll of the dice
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Roll {
    /// The camel that is allowed to move.
    camel: Camel,
    /// The number of steps they are allowed to take.
    face: Face,
}

/// The faces of the Camel dice.
///
/// It corresponds with the number of steps to take.
///
/// ```
/// # use camel_up::camel::{Face};
/// assert_eq!(usize::from(Face::One), 1 as usize);
/// assert_eq!(usize::from(Face::Two), 2 as usize);
/// assert_eq!(usize::from(Face::Three), 3 as usize);
/// ```
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Face {
    /// represents one step
    One,
    /// represents two steps
    Two,
    /// represents three steps
    Three,
}

impl Face {
    /// Convenience function that retuns all the possible face values.
    pub fn values() -> HashSet<Self> {
        vec![Face::One, Face::Two, Face::Three]
            .iter()
            .copied()
            .collect()
    }
}

impl From<(Camel, Face)> for Roll {
    fn from((camel, face): (Camel, Face)) -> Self {
        Self { camel, face }
    }
}

impl From<Face> for usize {
    fn from(face: Face) -> Self {
        match face {
            Face::One => 1,
            Face::Two => 2,
            Face::Three => 3,
        }
    }
}

impl Race {
    /// perform a roll on a race, returns the race with all the camels in their correct positions.
    pub fn perform<R>(&self, roll: R) -> Self
    where
        R: Into<Roll>,
    {
        let roll: Roll = roll.into();
        if self.positions.contains(&Marker::Camel(roll.camel)) {
            let index = self.positions.iter().position(|marker| *marker == Marker::Camel(roll.camel)).unwrap(/* camel is present because of contains check */);
            let offset = self.positions[index..]
                .iter()
                .take_while(|marker| marker.is_a_camel())
                .count();

            let unit = &self.positions[index..(index + offset)];
            let remaining: Vec<Marker> = self.positions[0..index]
                .iter()
                .chain(self.positions[(index + offset)..].iter())
                .chain(repeat(&Marker::Divider).take(4))
                .copied()
                .collect();

            let original_divider_offset = remaining[index..].iter().enumerate().filter(|(_, marker)| marker.is_a_divider()).map(|(index, _)| index).skip(roll.face as usize + 1).nth(0).unwrap(/* offset is present because of repeated divider */);
            let delta: usize = match remaining[index + original_divider_offset - 1] {
                Marker::Oasis => 2,
                Marker::FataMorgana => 0,
                _ => 1,
            };
            let divider_offset = remaining[index..].iter().enumerate().filter(|(_, marker)| marker.is_a_divider()).map(|(index, _)| index).skip(roll.face as usize + delta).nth(0).unwrap(/* offset is present because of repeated divider */);
            let result: Vec<Marker> = remaining[0..(index + divider_offset)]
                .iter()
                .chain(unit.iter())
                .chain(remaining[(index + divider_offset)..].iter())
                .copied()
                .collect();
            Self::from(result)
        } else {
            let positions: Vec<Marker> = self.positions.to_vec();
            Self::from(positions)
        }
    }

    /// Determines which camel is the winner, i.e. is at the front.
    pub fn winner(&self) -> Option<Camel> {
        self.positions
            .iter()
            .filter(|marker| marker.is_a_camel())
            .map(|marker| marker.to_camel().unwrap(/* camel is present because of filter on camel */))
            .last()
    }

    /// Determines which camel is the loser, i.e. is at the back.
    pub fn loser(&self) -> Option<Camel> {
        self.positions
            .iter()
            .filter(|marker| marker.is_a_camel())
            .map(|marker| marker.to_camel().unwrap(/* camel is present because of filter on camel */))
            .nth(0)
    }

    /// Determines which camel is the runner up, i.e. is behind the winner.
    pub fn runner_up(&self) -> Option<Camel> {
        self.positions
            .iter()
            .filter(|marker| marker.is_a_camel())
            .map(|marker| marker.to_camel().unwrap(/* camel is present because of filter on camel */))
            .rev()
            .nth(1)
    }
}

/// Represents the dice that still can be rolled.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Dice(HashSet<Camel>); // TODO model the fact that not all dice could be rolled.

impl Dice {
    /// Remove a dice from the pyramid, i.e. the options to throw are reduced.
    pub fn remove(&self, camel: Camel) -> Self {
        let mut dice = self.0.clone();
        dice.remove(&camel);
        Self::from(dice)
    }
}

impl Default for Dice {
    fn default() -> Self {
        let mut dice = HashSet::new();
        dice.insert(Camel::Red);
        dice.insert(Camel::Orange);
        dice.insert(Camel::Yellow);
        dice.insert(Camel::Green);
        dice.insert(Camel::White);
        Self::from(dice)
    }
}

impl From<HashSet<Camel>> for Dice {
    fn from(dice: HashSet<Camel>) -> Self {
        Self(dice)
    }
}

impl FromStr for Dice {
    type Err = NoDice;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut dice = HashSet::new();
        let mut index = 0;
        while index < input.len() {
            let marker = input[index..=index].parse::<Marker>()?;
            index += 1;
            match marker.to_camel() {
                Some(camel) => {
                    dice.insert(camel);
                }
                None => {
                    return Err(NoDice::NotACamel);
                }
            }
        }
        Ok(Dice::from(dice))
    }
}

impl IntoIterator for Dice {
    type Item = Camel;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// When parsing of Dice goes wrong, this enumeration tells you precisely what went down.
#[derive(PartialEq, Debug)]
pub enum NoDice {
    /// What is encountered isn't even a marker.
    NotAMarker(NotAMarker),
    /// It is a marker, but not a camel.
    NotACamel,
}

impl From<NotAMarker> for NoDice {
    fn from(error: NotAMarker) -> Self {
        NoDice::NotAMarker(error)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn races_can_be_equated() {
        let left = Race::from(vec![
            Marker::Camel(Camel::Red),
            Marker::Divider,
            Marker::Camel(Camel::Yellow),
        ]);
        let right = Race::from(vec![
            Marker::Camel(Camel::Red),
            Marker::Divider,
            Marker::Camel(Camel::Yellow),
        ]);

        assert_eq!(left, right);
    }

    #[test]
    fn races_can_be_parsed() {
        let left = "r,y".parse::<Race>().expect("to parse");
        let right = Race::from(vec![
            Marker::Camel(Camel::Red),
            Marker::Divider,
            Marker::Camel(Camel::Yellow),
        ]);

        assert_eq!(left, right);
    }

    #[test]
    fn camel_can_not_be_in_an_oasis() {
        let left = "r+,y".parse::<Race>();
        let right = Err(RaceParseError::CamelInOasis);

        assert_eq!(left, right);
    }

    #[test]
    fn camel_can_not_be_in_a_fata_morgana() {
        let left = "r-,y".parse::<Race>();
        let right = Err(RaceParseError::CamelInFataMorgana);

        assert_eq!(left, right);
    }

    #[test]
    fn adjustments_can_not_be_in_same_position() {
        let left = "r,+-,y".parse::<Race>();
        let right = Err(RaceParseError::ToManyAdjustmentsInOnePosition);

        assert_eq!(left, right);
    }

    #[test]
    fn adjustments_can_not_be_consecutive() {
        let left = "r,+,-,y".parse::<Race>();
        let right = Err(RaceParseError::ConsecutiveAdjustments);

        assert_eq!(left, right);
    }

    #[test]
    fn races_are_normalized() {
        let left = ",,,,,,,r,y,,,,,,,,,,,".parse::<Race>().expect("to parse");
        let right = Race::from(vec![
            Marker::Camel(Camel::Red),
            Marker::Divider,
            Marker::Camel(Camel::Yellow),
        ]);

        assert_eq!(left, right);
    }

    #[test]
    fn races_can_perform_a_roll_one() {
        let race = "ro,y".parse::<Race>().expect("to parse");
        let result = race.perform((Camel::Red, Face::One));
        let expected = "yro".parse::<Race>().expect("to parse");

        assert_eq!(result, expected);
    }

    #[test]
    fn races_can_perform_a_roll_two() {
        let race = "ro,y".parse::<Race>().expect("to parse");
        let result = race.perform((Camel::Red, Face::Two));
        let expected = "y,ro".parse::<Race>().expect("to parse");

        assert_eq!(result, expected);
    }

    #[test]
    fn races_can_perform_a_roll_three() {
        let race = "ro,y".parse::<Race>().expect("to parse");
        let result = race.perform((Camel::Red, Face::Three));
        let expected = "y,,ro".parse::<Race>().expect("to parse");

        assert_eq!(result, expected);
    }

    #[test]
    fn oasis_advance_a_camel_when_it_lands() {
        let race = "r,+,y".parse::<Race>().expect("to parse");
        let result = race.perform((Camel::Red, Face::One));
        let expected = "+,yr".parse::<Race>().expect("to parse");

        assert_eq!(result, expected);
    }

    #[test]
    fn fata_morgana_retreats_a_camel_when_it_lands() {
        let race = "r,-,y".parse::<Race>().expect("to parse");
        let result = race.perform((Camel::Red, Face::One));
        let expected = "r,-,y".parse::<Race>().expect("to parse");

        assert_eq!(result, expected);
    }

    #[test]
    fn dice_can_be_parsed() {
        let actual = "ryg".parse::<Dice>().expect("to parse");
        let mut dice = HashSet::new();
        dice.insert(Camel::Red);
        dice.insert(Camel::Yellow);
        dice.insert(Camel::Green);

        assert_eq!(actual, Dice::from(dice));
    }

    #[test]
    fn races_have_winners_runner_ups_and_losers() {
        let race = "r,y,g".parse::<Race>().expect("to parse");
        let winner = race.winner();
        let runner_up = race.runner_up();
        let loser = race.loser();

        assert_eq!(winner, Some(Camel::Green));
        assert_eq!(runner_up, Some(Camel::Yellow));
        assert_eq!(loser, Some(Camel::Red));
    }
}
