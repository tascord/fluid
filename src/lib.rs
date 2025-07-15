use std::cell::LazyCell;

const DICT_FILE: &[u8] = include_bytes!("../dict.bin");
const FLUID_DICT: LazyCell<Dict> = LazyCell::new(|| {
    bincode::borrow_decode_from_slice(DICT_FILE, bincode::config::standard())
        .expect("Fluid dictionary file is corrupt.")
        .0
});

#[derive(bincode::Decode, bincode::Encode, Debug)]
pub struct Dict {
    pub adjectives: Vec<String>,
    pub adverbs: Vec<String>,
    pub verbs: Vec<String>,
    pub nouns: Vec<String>,
}

impl Dict {
    pub fn unique_combinations(&self) -> u128 {
        self.adjectives.len() as u128
            * self.nouns.len() as u128
            * self.adverbs.len() as u128
            * self.verbs.len() as u128
    }
}

#[derive(Copy, Clone)]
pub struct Fluid(u128);

impl Fluid {
    pub fn new() -> Self {
        Fluid(rand::random::<u128>() & 0xFFFFFFFFFFFF4FFFBFFFFFFFFFFFFFFF | 0x40008000000000000000)
    }
}

impl std::fmt::Debug for Fluid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Fluid").field(&self.0.to_string()).finish()
    }
}

impl std::fmt::Display for Fluid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let adjective_index = ((self.0 >> 96) & 0xFFFFFFFF) as usize; // Most significant 32 bits
        let noun_index = ((self.0 >> 64) & 0xFFFFFFFF) as usize; // Next 32 bits
        let adverb_index = ((self.0 >> 32) & 0xFFFFFFFF) as usize; // Next 32 bits
        let verb_index = (self.0 & 0xFFFFFFFF) as usize; // Least significant 32 bits

        let adjective = &FLUID_DICT.adjectives[adjective_index % FLUID_DICT.adjectives.len()];
        let noun = &FLUID_DICT.nouns[noun_index % FLUID_DICT.nouns.len()];
        let adverb = &FLUID_DICT.adverbs[adverb_index % FLUID_DICT.adverbs.len()];
        let verb = &FLUID_DICT.verbs[verb_index % FLUID_DICT.verbs.len()];

        write!(f, "{}-{}-{}-{}", adjective, noun, adverb, verb)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::Fluid;

    #[test]
    fn generate() {
        let uids = std::iter::repeat_n((), 100)
            .map(|_| Fluid::new())
            .collect::<Vec<_>>();

        println!("");
        println!("===== DEBUG 50 Fluids:");
        uids.iter().enumerate().for_each(|(idx, uid)| {
            println!(
                "=> {}: {:?}",
                {
                    let mut s = idx.to_string();
                    while s.len() < 3 {
                        s = format!(" {s}");
                    }

                    s
                },
                uid
            );
        });

        println!("");

        println!("===== PRINT 50 Fluids:");
        uids.iter().enumerate().for_each(|(idx, uid)| {
            println!(
                "=> {}: {}",
                {
                    let mut s = idx.to_string();
                    while s.len() < 3 {
                        s = format!(" {s}");
                    }

                    s
                },
                uid
            );
        });
        println!("");
    }

    #[test]
    fn stress() {
        let count = 10_000;
        let mut generated_uids: HashSet<String> = HashSet::with_capacity(count);
        let mut clashes = 0;

        println!(
            "\nStarting UID stress test: Generating {} UIDs...",
            count
        );

        for i in 0..count {
            let fluid_id = Fluid::new();
            let uid_string = fluid_id.to_string();

            if !generated_uids.insert(uid_string) {
                // If insert returns false, the UID already existed in the HashSet, meaning a clash.
                clashes += 1;
            }

            if (i + 1) % (count / 10).max(1) == 0 || i == count - 1 {
                // Provide progress updates
                println!(
                    "  Generated {}/{} UIDs. Clashes so far: {}",
                    i + 1,
                    count,
                    clashes
                );
            }
        }

        println!("\nUID stress test finished.");
        println!("Total UIDs generated: {}", count);
        println!("Total clashes found: {}", clashes);

        assert_eq!(clashes, 0);
    }
}
