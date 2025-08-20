use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    IResult, Parser,
};

#[derive(Debug)]
pub struct Reindeer {
    pub name: String,
    pub speed: u32,
    pub flight_time: u32,
    pub rest_time: u32,
}

pub fn parse(input: &str) -> Vec<Reindeer> {
    let (_, reindeer) = separated_list1(newline, parse_reindeer)
        .parse(input)
        .unwrap();
    reindeer
}

fn parse_reindeer(input: &str) -> IResult<&str, Reindeer> {
    let (input, (name, _, speed, _, flight_time, _, rest_time, _)) = (
        alpha1,
        tag(" can fly "),
        complete::u32,
        tag(" km/s for "),
        complete::u32,
        tag(" seconds, but then must rest for "),
        complete::u32,
        tag(" seconds."),
    )
        .parse(input)?;
    Ok((
        input,
        Reindeer {
            name: name.to_string(),
            speed,
            flight_time,
            rest_time,
        },
    ))
}
