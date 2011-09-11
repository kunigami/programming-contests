#include <stdio.h>
#include <iostream>

using namespace std;

#define MAXN 123
#define MAXV 10010

typedef long long int i64;

i64 v[MAXN];

i64 mdc(i64 a, i64 b){
  return (b == 0) ? a : mdc(b, a % b); 
}
i64 mmc(i64 a, i64 b){
  i64 p = a*b;
  //printf("%lld mdc(%d, %d) = %d\n", p, a, b, mdc(a, b));
  return p / mdc(a, b);
}
i64 mdc(i64 *v, i64 n){
  i64 m = v[0];
  for(i64 i=1; i<n; i++)
    m = mdc(m, v[i]);
  return m;
}
i64 mmc(i64 *v, i64 n){
  i64 m = v[0];
  for(i64 i=1; i<n; i++){
    m = mmc(m, v[i]);
    if (m >= MAXV) return -1;
    //printf("%lld\n", m);
  }
  return m;
}

int main (){

  i64 t;
  scanf("%lld", &t);
  for(i64 cases=1; cases<=t; cases++){
    i64 n, l, h;
    scanf("%lld %lld %lld", &n, &l, &h);
    for(i64 i=0; i<n; i++)
      scanf("%lld", &v[i]);
    printf("Case #%lld: ", cases);
    bool ok2 = false;
    for(i64 k=l; k<=h; k++){
      bool ok = true;
      for(i64 i=0; i<n; i++){
	if (!((v[i] >= k && v[i] % k == 0) ||
	      (v[i] < k && k % v[i] == 0))){
	  ok = false;
	  break;
	}
      }
      if (ok){
	ok2 = true;
	printf("%lld\n", k);
	break;
      }
    }
    if (!ok2) printf("NO\n");
  }

}
