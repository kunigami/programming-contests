#include <iostream>
#include <cmath>
#include <algorithm>

using namespace std;

int main (){

  int t, cases = 0;
  scanf("%d", &t);
  while (t--){

    int n, pos;
    char type[10];
    scanf("%d", &n);

    int pa = 1, ta = 0;
    int pb = 1, tb = 0;
    int prev = -1, d;
    for(int i = 0; i < n; i++){
      scanf("%s %d", type, &pos);
      if (type[0] == 'O'){//A
	d = abs(pos - pa);
	if (prev == 0) // was A 
	  ta = ta + d + 1;
	else // was B
	  ta = max(ta + d, tb) + 1;
	prev = 0;
	pa = pos;
      }
      else {//B
	d = abs(pos - pb);
	if (prev == 1) // was B
	  tb = tb + d + 1;
	else // was A
	  tb = max(tb + d, ta) + 1;
	prev = 1;
	pb = pos;
      }
    }

    printf("Case #%d: %d\n", ++cases, max(ta, tb));

  }

  return 0;
}
