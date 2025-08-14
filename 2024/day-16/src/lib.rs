use std::collections::HashSet;

use glam::IVec2;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult, Parser,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug)]
pub struct Map {
    pub start_position: IVec2,
    pub stop_position: IVec2,
    pub walls: HashSet<IVec2>,
}

pub fn parse(input: &str) -> Map {
    let mut parser = separated_list1(newline, many1(token));
    let (_, items) = parser.parse(Span::new(input)).unwrap();

    let (start_position, _) = items
        .iter()
        .flatten()
        .find(|(_, item)| item == &'S')
        .cloned()
        .expect("must have start position");
    let (stop_position, _) = items
        .iter()
        .flatten()
        .find(|(_, item)| item == &'E')
        .cloned()
        .expect("must have start position");
    let walls = items
        .into_iter()
        .flatten()
        .filter_map(|(pos, item)| (item == '#').then_some(pos))
        .collect::<HashSet<IVec2>>();

    Map {
        start_position,
        stop_position,
        walls,
    }
}

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line();
    let x = input.get_column();
    let (input, token) = one_of(".SE#")(input)?;
    Ok((input, (IVec2::new(x as i32 - 1, y as i32 - 1), token)))
}
