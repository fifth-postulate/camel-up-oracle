use std::str::FromStr;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
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

#[derive(PartialEq, Debug)]
pub struct Race {
    positions: Vec<Marker>,
}

impl From<Vec<Marker>> for Race {
    fn from(positions: Vec<Marker>) -> Self {
        let positions = positions
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
        let left = ",,,,,,,r,y".parse::<Race>().expect("to parse");
        let right = Race::from(vec![
            Marker::Camel(Camel::Red),
            Marker::Divider,
            Marker::Camel(Camel::Yellow),
        ]);

        assert_eq!(left, right);
    }

}
