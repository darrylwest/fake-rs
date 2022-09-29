

use fake::Fake;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    use fake::locales::{EN, FR_FR};
    use fake::faker::name::raw::*;
    use fake::faker::internet::raw::*;
    use fake::faker::lorem::raw::*;

    for _n in 1..=100 {
        let first_name: String = FirstName(EN).fake();
        let last_name: String = LastName(EN).fake();
        let middle: char = rng.gen_range('A'..'Z');

        let dom1: String = LastName(FR_FR).fake();
        let dom2: String = Word(EN).fake();
        let suffix: String = DomainSuffix(EN).fake();

        let email = format!("{}.{}.{}@{}{}.{}", first_name.to_lowercase(), middle.to_lowercase(), last_name.to_lowercase(), dom1.to_ascii_lowercase(), dom2.to_ascii_lowercase(), suffix);

        println!("name: {} {}, email: {}", first_name, last_name, email);
    }
}