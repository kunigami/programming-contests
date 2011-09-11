#include <iostream>
#include <stdio.h>

using namespace std;

char mat[100][100];
int r, c;

bool check_square(int i, int j){
  if (i >= r-1 || j >= c-1) return false;
  if (mat[i+1][j] != '#' || mat[i][j+1] != '#' || mat[i+1][j+1] != '#')
    return false;
  return true;
}

bool fill (){

  for(int i=0; i<r; i++)
    for(int j=0; j<c; j++)
      if (mat[i][j] == '#'){
	if (!check_square(i, j)) return false;
	mat[i][j] = '/';
	mat[i][j+1] = '\\';
	mat[i+1][j] = '\\';
	mat[i+1][j+1] = '/';
      }
  return true;
}

int main (){

  int t;
  scanf("%d", &t);
  for(int cases=1; cases<=t; cases++){
    scanf("%d %d", &r, &c);
    for(int i=0; i<r; i++)
      scanf("%s", mat[i]);
  
    printf("Case #%d:\n", cases);
    if (!fill())
      printf("Impossible\n");
    else
      for(int i=0; i<r; i++)
	printf("%s\n", mat[i]);
  }
  
}
