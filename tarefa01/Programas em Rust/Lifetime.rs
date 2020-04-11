
struct Pessoa{
	nome: String,
	idade: u16,
}

fn main(){
	let  mut v : Vec<Pessoa> = Vec::new();
	v.push(Pessoa{nome : "Fernando".to_string(),idade : 39,});
	v.push(Pessoa{nome : "Bianca".to_string(),idade : 45,});
	v.push(Pessoa{nome : "Joao".to_string(),idade : 48,});
	let maior = verifica_idade(&v);
	println!("{}",maior.idade);
}

fn verifica_idade<'a>(v: &'a Vec<Pessoa>) -> &'a Pessoa{ 
	let mut maior_Pessoa: &Pessoa = v.first().unwrap();
	let mut maior_idade = maior_Pessoa.idade;
	for p in v{
		if p.idade > maior_idade{
			maior_idade = p.idade;
			maior_Pessoa = p;
								 }
			   }
	return maior_Pessoa;
}
