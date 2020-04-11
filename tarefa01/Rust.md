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

<h3>-Referência bibliográfica</h3>
<p>https://www.ibm.com/developerworks/br/library/os-developers-know-rust/index.html<br>
https://pt.wikipedia.org/wiki/Rust_(linguagem_de_programa%C3%A7%C3%A3o)</p>
