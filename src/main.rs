use std::fs::{self};

const ADJ: &str = include_str!("../data/adj.txt");
const ADV: &str = include_str!("../data/adv.txt");
const VRB: &str = include_str!("../data/vrb.txt");
const N: &str = include_str!("../data/n.txt");

const FILTER: &str = include_str!("../data/filter.txt");

trait Ext {
    fn dedup(self) -> Self;
}

impl Ext for String {
    fn dedup(self) -> Self {
        let mut s = self.chars().collect::<Vec<_>>();
        s.dedup();
        String::from_iter(s)
    }
}

fn filter(v: &str) -> Vec<String> {
    v.lines()
        .map(|v| v.trim().to_lowercase())
        .filter(|l| l.len() - l.to_string().dedup().len() < 3) // Remove words with more than three duplicate letters
        .filter(|l| l.len() < 10) // Remove long words
        .filter(|l| l.len() > 2) // Remove short words
        .filter(|l| !FILTER.contains(l))
        .collect::<Vec<_>>()
}

fn main() {
    let dict = fl_uid::Dict {
        adjectives: filter(ADJ),
        adverbs: filter(ADV),
        verbs: filter(VRB),
        nouns: filter(N),
    };

    println!("");
    println!("===== Created Fluid Dictionary");
    println!("=> ADJ: {}", dict.adjectives.len());
    println!("=> ADV: {}", dict.adverbs.len());
    println!("=> VRB: {}", dict.verbs.len());
    println!("=>   N: {}", dict.nouns.len());
    println!(
        "=>   #: {} Combinations",
        dict.unique_combinations()
            .to_string()
            .chars()
            .rev()
            .collect::<Vec<_>>()
            .as_slice()
            .chunks(3)
            .map(|c| c.iter().rev().collect::<String>())
            .rev()
            .collect::<Vec<_>>()
            .join(",")
    );

    bincode::encode_into_std_write(
        dict,
        &mut fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open("./dict.txt")
            .unwrap(),
        bincode::config::standard(),
    )
    .expect("Failed to encode dictionary.");

    println!("===== Wrote to ./dict.txt");
    println!("");

    
}
