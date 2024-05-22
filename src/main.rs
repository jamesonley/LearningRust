/* fn main() {
    let a = 38;
    let b = 4; 
    let mut sum = a;

    sum += b;

    println!("{} + {} = {}", a, b, sum);
}
*/

impl Accumulator(){
    fn get(self) -> i32 {
        self.sum;
    }
}


fn main() {
    let mut acc = Accumulator::new(0);

    for n in 3..10 {
        acc = acc.add(n);
    }

    println!("acc = {}", acc.get());
}
 