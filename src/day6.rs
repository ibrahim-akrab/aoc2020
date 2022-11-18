pub fn day6a() -> String {
    let groups = read_data();
    groups
        .iter()
        .map(|s| {
            s.split_whitespace()
                // convert the whole group's answers to continous bytes stream
                .flat_map(|s| s.bytes())
                // create the bitmap representation of group's answers
                .fold(0u32, |val, question| val | 1 << (question - b'a'))
        })
        .map(count_set_bits)
        .sum::<u32>()
        .to_string()
}

pub fn day6b() -> String {
    let groups = read_data();
    groups
        .iter()
        .map(|s| {
            s.split_whitespace()
                // create an iterator of the bitmap representation of
                // each person's answers within the same group
                .map(|s| {
                    s.bytes()
                        .fold(0u32, |val, question| val | 1 << (question - b'a'))
                })
                // create bitmap of answers everyone answered with "yes"
                .fold(0xFFFFFFFFu32, |val, bitmap| val & bitmap)
        })
        .map(count_set_bits)
        .sum::<u32>()
        .to_string()
}

fn count_set_bits(mut bitmap: u32) -> u32 {
    let mut count = 0;
    while bitmap != 0 {
        count += 1;
        bitmap &= bitmap - 1;
    }
    count
}

// struct Group(u32);
//
// impl Group {
//     fn new(s: &str) -> Self {
//         // bitmap of each question answered with yes
//         let value = s
//             .split_whitespace()
//             .map(|s| s.bytes())
//             .flatten()
//             .fold(0u32, |val, question| val | 1 << (question - b'a'));
//         Self(value)
//     }
//
//     fn count_questions(mut self) -> u32 {
//         let mut count = 0;
//         while self.0 != 0 {
//             count += 1;
//             self.0 &= self.0 - 1;
//         }
//         count
//     }
//
//     fn count_and
// }
//
fn read_data() -> Vec<String> {
    use std::fs;
    fs::read_to_string("inputs/day6.txt")
        .expect("Couldn't read file")
        .split("\n\n")
        .map(String::from)
        .collect()
}
