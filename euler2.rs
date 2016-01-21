fn fib(n: u64) -> u64{
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    }    
}

fn euler (n: u64) -> u64 {
	let numbers = fib(n).collect::<Vec<i32>>()
	.filter(|i| i % 2);//.find(|x| *x % 2));
}

fn main() {
	println!("{:?}", euler(4000000));
}