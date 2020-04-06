
struct Pessoa{
	nome: String,
	idade: u16,
}

fn main(){
	let f = Pessoa{nome : "Fernando".to_string(),idade : 39,};
	let b = Pessoa{nome : "Bianca".to_string(),idade : 45,};
	let maior = verifica_idade(&f,&b);
	println!("{}",maior.idade);
}

fn verifica_idade<'a>(p1: &'a Pessoa,p2 : &'a Pessoa) -> &'a Pessoa{   //Como nesta função não sabemos se será retornado p1 ou p2 é necessario que declarar que tanto os parametros quanto o retorno tem lifetime a
	if p1.idade > p2.idade{											   
		return p1;
						   }
	return p2;
}

