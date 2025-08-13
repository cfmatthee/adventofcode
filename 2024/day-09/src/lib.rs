use std::iter;

use itertools::enumerate;

pub fn parse(input: &str) -> Vec<u16> {
    let mut disk: Vec<u16> = Vec::new();
    let mut id: u16 = 1;
    for (idx, c) in enumerate(input.chars()) {
        if c == '\n' {
            break;
        }
        let len = c.to_digit(10).unwrap() as usize;
        match idx % 2 {
            0 => {
                let new = iter::repeat_n(id, len).collect::<Vec<_>>();
                disk.extend(new);
                id += 1;
            }
            1 => {
                let new = iter::repeat_n(0_u16, len).collect::<Vec<_>>();
                disk.extend(new);
            }
            _ => panic!("not possible"),
        }
    }
    disk
}
