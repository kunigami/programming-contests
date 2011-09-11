#include <stdio.h>
#include <algorithm>
using namespace std;
int main (){

  int t, cases = 0;
  scanf("%d", &t);
  while (t--){

    printf("Case #%d: ", ++cases);
    int n, v, x = 0, y = 1231231, s = 0;
    scanf("%d", &n);
    for(int i=0; i<n; i++){
      scanf("%d", &v);
      x ^= v;
      y = min(y, v);
      s += v;
    }
    if (!x) printf("%d\n", s-y);
    else printf("NO\n");
  }

  return 0;
}
