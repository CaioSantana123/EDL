fn main(){
	let n : u64 = 7;		
	println!("Sequencia:\n");
	for i in 0..n {
		println!("{} ",fibonacci(i));
				   }
	}

fn fibonacci(n : u64) -> u64{
	let mut f1 : u64 = 0;
	let mut f2 : u64 = 1;
	let mut f3 : u64;
	if n == 0{
		return 0;
			  }
	for i in 1..n{
		f3 = f1 + f2;
		f1 = f2;
		f2 = f3;
					}
	return f2;
} 
