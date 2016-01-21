fn euler() -> u64 {
	//use std::iter::AdditiveIterator;
	let numbers = (1..1000).filter(|i| i %3 == 0 || i % 5 == 0);
	numbers.fold(0, |acc, x| acc + x)
}

fn main() {
	println!("{:?}", euler());
}