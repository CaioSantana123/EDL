<h1>Trabalho Rust</h1>
<p>Integrantes: Caio Luiz Alonso Santana, Leonardo Fridman Bacellar</p>
<h3>1-Introdução</h3><p>Rust é uma linguagem multiparadigma que surgiu como projeto pessoal de Graydon Hoare, funcionário da Mozilla, e depois foi adotada pela empresa. A Mozilla começou a apoiar o projeto em 2009 e anunciou a linguagem em 2010, e seu compilador era escrito em OCaml e depois foi modificado para um compilador auto-hospedado em Rust conhecido como rustc, compilado pela primeira vez em 2011. A versão pré-alfa foi lançada pela primeira vez em 2012 e sua primeira versão estável, a Rust 1.0, foi lançada em 15 de maio de 2015.</p>
<h3>2- Origens e Influências</h3><p>Rust foi influenciado por Cyclone(uma linguagem imperativa derivada de C), com alguns recursos de orientação a objetos de C++ e recursos funcionais de linguagens como Haskell e OCaml. O resultado final é uma linguagem semelhante a C com suporte a programação multiparadigma(imperativa, funcional e orientada a objeto).</p><br>


![](GenealogiaRust.png)


<h3>3-Classificação</h3><p>Rust é uma linguagem orientada a objetos, estruturada, imperativa, concorrente e funcional com tipagem estática, forte e inferida.</p>
<h3>4-Exemplos comparativos</h3><p>-Java vs Rust(Cálculo da área de um círculo)</p>

	public class Circle{
		float raio;
		public Circle(float raio){
			this.raio=raio;
		}
		float area(){
			return (this.raio)*(float)3.14;
		}
		public static void main(String args[]){
			Circle c=new Circle((float)4.3);
			System.out.println("Raio: " + c.area());
		}
	}
-Java<br>



	pub struct Circulo{
		raio : f32,
	}
	impl Circulo{
		pub fn new(raio: f32) -> Circulo{
			Circulo{
				raio: raio,
			}
		}
		pub fn calcula_raio(&self) -> f32{
			return 2.0 * self.raio * 3.14;
		}
	}
	fn main(){
		let c = Circulo::new(4.2);
		print!("Area: {}", c.calcula_raio());
	}
-Rust<br>
<p>As diferenças de rust para java neste código é que em rust os métodos e os campos devem ser implementados em blocos diferentes enquanto em java é possível deixar ambos no mesmo bloco.Em java,e em outras linguagens orientadas ao objeto,é possível escrever um construtor para a sua classe de maneira muito mais simples e concisa do que em rust visto que construtores em rust são apenas metodos estáticos e meramente convencionais.</p>
<p>-C++ vs Rust(Fibonacci Iterativo)</p>

	int main(){
	    int n,i;
	    n = 7;
	    for(i = 0; i < n; i++){
		printf("%d ",fibonacci(i));
	    }
	    return 0;
	}
	int fibonacci(int n)
	{
	    if(n == 0){
		return 0;
	    }
	    int i;
	    int f1 = 0;
	    int f2 = 1;
	    int f3;
	    for (i = 1; i < n; i++){
		f3 = f1 + f2;
		f1 = f2;
		f2 = f3;
	}

	    return f2;

	};

-C++<br>
	
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

-Rust<br>
<p>A diferença entre Rust e C++ nesse exemplo é que você define a quantidade de bytes de cada variável numérica em Rust e em C++ não. Os loops em Rust são bem mais simples do que aqueles em C++, sendo muito mais parecidos com os de Python. Ao declarar variáveis em Rust, elas são fixas por padrão, isto é para garantir a segurança da linguagem. Caso queira uma variável mutável, você deve avisar para o compilador colocando o prefixo mut.</p>

	let mut v:i32;//um exemplo de variável mutável

-C++ vs Rust(Lifetime)
	
	#include<vector>
	#include<iostream>
	using namespace std;

	struct Pessoa{
		int idade;
		string nome;
	};
	Pessoa* verifica_idade(vector<Pessoa> *v);

	int main(){
		vector<Pessoa> v;
		Pessoa p;
		p.idade = 39;
		p.nome = "Fernando";
		v.push_back(p);
		p.idade = 45;
		p.nome = "Bianca";
		v.push_back(p);
		p.idade = 48;
		p.nome = "Joao";
		v.push_back(p);
		Pessoa *e = verifica_idade(&v);
		cout << (*e).idade << endl;
		return 0;
	}
	
	Pessoa* verifica_idade(vector<Pessoa> *v){
		Pessoa *maior_pessoa = &(v->front());
		int maior_idade = (*maior_pessoa).idade;
		vector<Pessoa>::iterator it;
		for(it = v->begin();it != v->end();it++){
			if((*it).idade > maior_idade){
				*maior_pessoa = (*it);
				maior_idade = (*it).idade;
			}
		}
		return maior_pessoa;
	}

-C++

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
		let mut maior_idade = 0;
		for p in v{
			if p.idade > maior_idade{
				maior_idade = p.idade;
				maior_Pessoa = p;
									}
				   }
		return maior_Pessoa;
	}
-Rust

Uma das principais diferenças entre os dois códigos é o fato de que quando se trata de operações usando apontadores em C++ elas são bastante perigosas e complexas quando comparadas às em Rust,visto que é possível retornar uma referência a um espaço de memória que está no escopo da função,em Rust um código assim não compilaria,graças ao conceito de lifetime,tornando operações que usam apontadores muito mais seguras.

<h2>Macros - Rust</h2>

- O que é Macro em Rust?
	Macro é uma especialidade do Rust que lhe oferece uma forma de criar o seu próprio código ou sintaxe, conhecida como metaprogramação. O uso de macros como println! ou print!, que lhe permite imprimir textos na saída do programa, e vec!, que permite o programador inicializar um vetor, são alguns exemplos dessa funcionalidade. A metaprogramação é útil para reduzir o tamanho do código que o programador escreve, papel semelhante às funções, contudo a metaprogramação com as macros em Rust tem suas especialidades.
	Uma assinatura de função deve declarar o número e o tipo de parâmetros que a função tem, macros, por outro lado, pode aceitar um número variável de parâmetros. Entretanto, o lado negativo de usar macro ao invés de função é que as macros são mais complexas do que declarar funções, porque o programador "está escrevendo Rust que sobrescreve código de Rust". Portanto, macros, em sua grande maioria, são mais difíceis de entender e ler seus códigos.
-As influências e origem da Macro
	As implementações mais amplamente utilizadas de macros sintáticas são encontradas em linguagens semelhantes a LISP. Linguagens como C e Assembly usam macros simples, implementados como pré-processadores no compilador ou montador. Em C, por exemplo, as macros funcionam pela simples busca e substituição de texto no código fonte.
	Em dialetos da linguagem LISP(List Processing, ou processamento de lista, em inglês), como Common Lisp e Scheme, são um pouco mais elaborados e complexos. As macros nesses dialetos agem como funções que transformam o texto do programa, usando a própria linguagem para expressar estas transformações.


<h2>-Lifetime</h2>

Rust é uma linguagem que se propõe a ter foco em segurança em relação à memória e a ser veloz,ela consegue isto através do sistema de ownership,cujo engloba lifetime, e que garante que não haverá mais de uma referência apontando para o mesmo local na memória e que assim que uma variável deixar de ser usada ao decorrer do programa a memória consumida por ela será liberada automaticamente.À primeira vista o conceito de lifetime em rust pode parecer bastante com o de garbage collector.

<h4>-Garbage Collector</h4>

Garbage Collector surgiu em 1959 e começou a ser implementada em Lisp com o propósito de aumentar a produtividade dos desenvolvedores,visto que eles não necessitariam gerenciar memória manualmente,apesar de este conceito se assemelhar com o de lifetime eles diferem em alguns pontos.Primeiramente pelo fato de que o compilador em rust ser quem analisa o código,portanto não interfere na velocidade da execução do código diferentemente do garbage collector que decide se deve desalocar memória ou não em tempo de execução.Difere também pelo fato do garbage collector não desalocar memória que não será mais usada ao decorrer do programa,ele desaloca apenas espaços de memória para os quais não exista mais uma referência para eles.

<h3>-Referência bibliográfica</h3>
<p>https://www.ibm.com/developerworks/br/library/os-developers-know-rust/index.html<br>
https://pt.wikipedia.org/wiki/Rust_(linguagem_de_programa%C3%A7%C3%A3o)</p>
https://doc.rust-lang.org/1.9.0/book/lifetimes.html
https://en.wikipedia.org/wiki/Garbage_collection_(computer_science)
