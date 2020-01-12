use std::collections::HashSet;
use std::iter::repeat;
use std::str::FromStr;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Camel {
    Red,
    Orange,
    Yellow,
    Green,
    White,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Marker {
    Camel(Camel),
    Divider,
}

impl Marker {
    fn is_a_camel(&self) -> bool {
        match self {
            Marker::Camel(_) => true,
            _ => false,
        }
    }

    fn is_a_divider(&self) -> bool {
        match self {
            Marker::Divider => true,
            _ => false,
        }
    }

    fn to_camel(&self) -> Option<Camel> {
        match self {
            Marker::Camel(camel) => Some(*camel),
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
            _ => Err(NotAMarker::But(input.to_owned())),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum NotAMarker {
    But(String),
}

#[derive(PartialEq, Eq, Debug)]
pub struct Race {
    positions: Vec<Marker>,
}

impl Clone for Race {
    fn clone(&self) -> Self {
        Self {
            positions: self.positions.iter().cloned().collect(),
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
        let positions = positions[min..(max + 1)]
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
            result.push(input[cursor..(cursor + 1)].parse::<Marker>()?);
            cursor += 1;
        }

        Ok(Race::from(result))
    }
}

#[derive(PartialEq, Debug)]
pub enum RaceParseError {
    NotAMarker(NotAMarker),
}

impl From<NotAMarker> for RaceParseError {
    fn from(problem: NotAMarker) -> Self {
        Self::NotAMarker(problem)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Roll {
    camel: Camel,
    face: Face,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Face {
    One,
    Two,
    Three,
}

impl Face {
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
                .chain(repeat(&Marker::Divider).take(3))
                .copied()
                .collect();
            let divider_offset = remaining[(index+1)..].iter().enumerate().filter(|(_, marker)| marker.is_a_divider()).map(|(index, _)| index).skip(roll.face as usize).nth(0).unwrap(/* offset is present because of repeated divider */);
            let result: Vec<Marker> = remaining[0..(index + 1 + divider_offset)]
                .iter()
                .chain(unit.iter())
                .chain(remaining[(index + 1 + divider_offset)..].iter())
                .copied()
                .collect();
            Self::from(result)
        } else {
            let positions: Vec<Marker> = self.positions.iter().cloned().collect();
            Self::from(positions)
        }
    }

    pub fn winner(&self) -> Option<Camel> {
        self.positions
            .iter()
            .filter(|marker| marker.is_a_camel())
            .map(|marker| marker.to_camel().unwrap())
            .last()
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
}
