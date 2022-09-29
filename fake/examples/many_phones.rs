use fake::faker::phone_number::raw::CellNumber;
use fake::locales::EN;
use fake::Fake;
// use fake::faker::phone_number::raw::*;

fn main() {

    for n in 1..=1_000 {
        let val: String = CellNumber(EN).fake();

        print!("{}, ", val);

        if n % 10 == 0 {
            println!();
        }
    }
}