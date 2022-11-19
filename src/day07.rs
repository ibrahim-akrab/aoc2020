use regex::Regex;
use std::collections::HashMap;

pub fn day7a() -> String {
    let bags = read_data();
    let my_bag = "shiny gold";
    bags.values()
        .filter(|b| b.can_ultimately_hold(my_bag, &bags))
        .count()
        .to_string()
}

pub fn day7b() -> String {
    let bags = read_data();
    let my_bag = "shiny gold";

    count_bags(bags.get(my_bag).unwrap(), &bags).to_string()
}

fn count_bags(bag: &Bag, set: &HashMap<String, Bag>) -> usize {
    if bag.contains.is_empty() {
        return 0;
    }
    bag.contains.iter().fold(0, |total, (n, b)| {
        let inner_count = count_bags(set.get(b).unwrap(), set);
        total + n * (inner_count + 1)
    })
}

fn read_data() -> HashMap<String, Bag> {
    use std::fs;
    let factory = BagFactory::default();
    let mut set = HashMap::new();
    fs::read_to_string("inputs/day7.txt")
        .expect("Couldn't read file")
        .split('\n')
        .filter_map(|s| factory.creata_bag(s))
        .for_each(|b| {
            set.insert(b.color.to_string(), b);
        });
    set
}

#[derive(Debug)]
struct Bag {
    color: String,
    contains: Vec<(usize, String)>,
}

impl Bag {
    fn new(color: &str) -> Self {
        Self {
            color: color.to_string(),
            contains: Vec::new(),
        }
    }

    fn can_contain(&mut self, n: usize, bag: &str) {
        self.contains.push((n, bag.to_string()));
    }

    fn can_ultimately_hold(&self, color: &str, set: &HashMap<String, Bag>) -> bool {
        if self.contains.is_empty() {
            return false;
        }
        self.contains.iter().any(|(_, b)| {
            b.as_str() == color || set.get(b.as_str()).unwrap().can_ultimately_hold(color, set)
        })
    }
}

struct BagFactory {
    outer_bag_re: Regex,
    inner_bags_re: Regex,
}

impl Default for BagFactory {
    fn default() -> Self {
        let outer_bag_re = Regex::new(r"^(.*) bags contain (.*)\.$").unwrap();
        let inner_bags_re = Regex::new(r"(\d+) (.*) bags?").unwrap();
        Self {
            outer_bag_re,
            inner_bags_re,
        }
    }
}
impl BagFactory {
    fn creata_bag(&self, s: &str) -> Option<Bag> {
        let caps = self.outer_bag_re.captures(s)?;
        let outer_bag_color = caps.get(1)?.as_str();
        let mut bag = Bag::new(outer_bag_color);
        let contains = caps.get(2)?.as_str();
        if contains == "no other bags" {
            Some(bag)
        } else {
            for inner_bag in contains.split(',') {
                let caps = self.inner_bags_re.captures(inner_bag)?;
                bag.can_contain(caps.get(1)?.as_str().parse().ok()?, caps.get(2)?.as_str());
            }
            Some(bag)
        }
    }
}
