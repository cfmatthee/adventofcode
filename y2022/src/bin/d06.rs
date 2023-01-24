use std::collections::HashSet;

fn main() {
    find_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_owned(), 14);
    find_start("bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned(), 14);
    find_start("nppdvjthqldpwncqszvftbrmjlhg".to_owned(), 14);
    find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned(), 14);
    find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned(), 14);
    find_start(include_str!("../files/d06").to_owned(), 14);
}

fn find_start(data: String, packet_len: usize) {
    let data = data.as_bytes();
    let len = data.len() - packet_len - 1;
    for idx in 0..len {
        let slice = data.iter().skip(idx).take(packet_len);
        let set: HashSet<u8> = slice.map(|b| *b as u8).collect();
        let len = set.len();
        if len == packet_len {
            println!("{}", idx + packet_len);
            break;
        }
    }
}
