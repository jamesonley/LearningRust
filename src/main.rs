/* fn main() {
    let a = 38;
    let b = 4; 
    let mut sum = a;

    sum += b;

    println!("{} + {} = {}", a, b, sum);
}
*/

struct Accumulator {
    numbers: Vec<i32>,
}

impl Accumulator {
    fn new() -> Accumulator {
        return Accumulator { numbers: Vec::new() }
    }

    fn get(&self) -> i32 {
        self.numbers.iter().sum()
    }
}


fn main() {
    let mut acc = Accumulator::new();

    for n in 3..10 {
        acc.numbers.push(n);
    }

    println!("acc = {}", acc.get());
}
 