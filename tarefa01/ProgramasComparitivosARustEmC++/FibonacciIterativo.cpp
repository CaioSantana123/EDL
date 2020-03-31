#include <stdio.h>
int fibonacci(int n);

int main(){
	int n,i;
	scanf("%d",&n);
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
