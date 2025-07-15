<img width="200" height="200" align="left" style="float: left; margin: 0 20px 0 0;" alt="Icon" src="https://github.com/tascord/fluid/blob/main/icon.png?raw=true"> 

<!-- <img width="200" height="200" align="left" style="float: left; margin: 0 20px 0 0;" alt="Icon" src="./banner.png">  -->

# Fluid
## Human Readable Uids

[![GitHub top language](https://img.shields.io/github/languages/top/tascord/fluid?color=0072CE&style=for-the-badge)](#)
[![Crates.io Version](https://img.shields.io/crates/v/fl_uid?style=for-the-badge)](https://crates.io/crates/fluid)
[![docs.rs](https://img.shields.io/docsrs/fl_uid?style=for-the-badge)](https://docs.rs/fl_uid)

### Features
- **Human Readable Uids**: Generate unique identifiers that easy discern.
- **High Uniqueness**: Leverages a large dictionary to provide trillions of possible combinations

### Usage
```rs
use fluid::Fluid;

fn main() {
    // Generate a new Fluid ID
    let id = Fluid::new();

    // Convert it to its human-readable string representation
    let id_string = id.to_string();

    println!("Generated Fluid ID: {}", id_string);
    // Example output: "quick-brown-fox-jumps"
}
```

### Uniqueness and Clash Probability
Fluid uses u128 as its internal random seed, providing a vast space of possible inputs. The number of unique combinations generated is the product of the sizes of your filtered word lists.

With the dictionary sizes:
- `ADJ`: 1,075
- `ADV`: 1,524
- `VRB`: 874
- `  N`: 1,156

For a total of 1,655,246,575,200 (over 1.6 trillion) unique combinations.

**Practical Uniqueness**: For most applications, generating a few million or even billion IDs, the probability of a random clash is extremely, vanishingly small.

**Theoretical Clashes**: The underlying u128 has far more states (2^128) than your total word combinations. The to_string() implementation uses modulo arithmetic to map parts of the u128 to dictionary indices. This means that theoretically, different u128 values can produce the same string, but the immense number of combinations makes such collisions rare in practice unless you approach the limit of the combination space.