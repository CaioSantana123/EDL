#include<stdio.h>
class Circulo{
	public : 
		float raio;
		float calcula_raio(){
			return 2.0 * raio * 3.14;
		}
};


int main(){
	Circulo c;
	c.raio = 4.2;
	printf("%f\n",c.calcula_raio());
	return 0;
}
