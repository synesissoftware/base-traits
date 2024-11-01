
use base_traits::ToF64;

/// Trivial implementation of a price type using exact types (integers)
#[derive(Debug)]
struct Price {
    dollars : u32,
    cents : u8,
}

impl ToF64 for Price {
    fn to_f64(&self) -> f64 {
        self.dollars as f64 + (self.cents as f64 / 100.0)
    }
}


/// Trivial implementation for calculation of mean and std-deviation
fn calc_mean_and_stddev<'a, F, I>(
    i : I
) -> Option<(
    f64, // mean
    f64, // stddev
)>
where
    F : ToF64 + 'a,
    I : Iterator<Item = &'a F>,
{
    let values = i.map(|v| v.to_f64()).collect::<Vec::<_>>();

    if values.is_empty() {
        None
    } else {
        let n = values.len() as f64;
        let sum : f64 = values.iter().sum();
        let mean = sum / n;
        let ss : f64 = values.iter().map(|v| (v - mean)).map(|v| v*v).sum();
        let var = ss / n;
        let stddev = var.sqrt();

        Some((mean, stddev))
    }
}


fn main() {
    {
        let raw_values = vec![
            // insert list
            10.0,
            10.1,
            10.2,
            10.3,
        ];

        let (raw_mean, raw_stddev) = calc_mean_and_stddev(raw_values.iter()).unwrap();

        println!("for {raw_values:?}, mean={raw_mean}, std-dev={raw_stddev}");
    }

    {
        let prices = vec![
            // insert list
            Price { dollars: 10, cents : 0 },
            Price { dollars: 10, cents : 1 },
            Price { dollars: 10, cents : 2 },
            Price { dollars: 10, cents : 3 },
        ];

        let (price_mean, price_stddev) = calc_mean_and_stddev(prices.iter()).unwrap();

        println!("for {prices:?}, mean={price_mean}, std-dev={price_stddev}");
    }
}

