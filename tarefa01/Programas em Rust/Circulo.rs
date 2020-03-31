struct Circulo{
	raio : f32,
}

impl Circulo{
	pub fn calcula_raio(&self) -> f32{
		return 2.0 * self.raio * 3.14;
	}
}


fn main(){
	let c = Circulo{raio : 4.2};
	println!("{} ",c.calcula_raio());
}

