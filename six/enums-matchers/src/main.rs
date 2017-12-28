enum FreedomDollar {
    One,
    Five,
    Ten,
    Twenty(TwentyVariant),
    Fifty,
    Hundred
}

#[derive(Debug)]
enum TwentyVariant {
    Tubman,
    Jackson
}

impl FreedomDollar {
    fn value(&self) -> u32 {
        match *self {
            FreedomDollar::One => 1,
            _ => 0
        }
    }
}

fn value_in_dollars(dollar : FreedomDollar) -> u32 {
    match dollar {
        FreedomDollar::One => 1,
        FreedomDollar::Five => 5,
        FreedomDollar::Ten => 10,
        FreedomDollar::Twenty(variant) => {
            match variant {
                TwentyVariant::Tubman => {
                    println!("When the Tubman's drop I need 20 G's");
                },
                TwentyVariant::Jackson => {
                    println!("Meh");
                }
            }
            20
        },
        FreedomDollar::Fifty => 50,
        FreedomDollar::Hundred => {
            println!("HUNNIDS BABY");
            100
        }
    }
}

fn main() {
    let twenty_dolla = FreedomDollar::Twenty(TwentyVariant::Tubman);
    println!("{:?}", value_in_dollars(twenty_dolla));

    // Ownership of twenty dolla abandoned in value_in_dollars
    let hunnid = FreedomDollar::Hundred;
    println!("{:?}", hunnid.value());
}
