use std::io;

fn main(){
	let n : u64;
	let mut input = String::new();
	match io::stdin().read_line(&mut input){
		Ok(__) => {},
		Err(_e) => {println!("Erro na leitura de dados\n")},
											}
	n = input.trim().parse::<u64>().unwrap();
	println!("Sequencia:\n");
	for i in 0..n {
		println!("{} ",fibonacci(i));
				   }
	
}

fn fibonacci(n : u64) -> u64{
	if n == 0{
		return 0;
			  }
	else if n == 1{
		return 1;
				   }
	else{
		return fibonacci(n-1) + fibonacci(n-2);
		 }
} 
