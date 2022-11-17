pub fn day3a() -> String {
    let forest = read_data();
    track_path(&forest, 3, 1).to_string()
}

pub fn day3b() -> String {
    let forest = read_data();
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .fold(1usize, |total, (n_right, n_down)| {
            total * track_path(&forest, *n_right, *n_down)
        })
        .to_string()
}

fn read_data() -> Vec<String> {
    use std::fs;
    let values = fs::read_to_string("inputs/day3.txt").expect("Couldn't read file");
    values
        .split('\n')
        .filter(|&s| !s.is_empty())
        .map(String::from)
        .collect()
}

fn track_path(rows: &[String], n_right: usize, n_down: usize) -> usize {
    rows.iter()
        .step_by(n_down)
        .fold((0usize, 0usize), |(total, pos), s| {
            let (new_pos, has_tree) = move_right(s, pos, n_right);
            if has_tree {
                (total + 1, new_pos)
            } else {
                (total, new_pos)
            }
        })
        .0
}

fn move_right(s: &str, pos: usize, n_right: usize) -> (usize, bool) {
    let new_pos = (pos + n_right) % s.len();
    let is_tree = s.as_bytes()[pos] == b'#';
    (new_pos, is_tree)
}
