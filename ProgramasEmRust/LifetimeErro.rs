fn main(){
	let var1 : i32 = 1;
	{
	let var2 : i32 = 2; //escopo diferente
	}
	println!("{}",var1 + var2) //lifetime de var2 chegou ao fim,logo terminará em erro
}
