#include <stdio.h>
#include <string.h>
#include <algorithm>
#include <vector>
using namespace std;

#define MAXN 1234

int v[MAXN], len[MAXN];

int main (){

  int t;
  scanf("%d", &t);
  for(int cases=1; cases <= t; cases++){
  
    int X, S, R, T, N;
    scanf("%d %d %d %d %d", &X, &S, &R, &T, &N);

    int maxv = 0;
    memset(len, 0, sizeof(len));
    int rem = X;
    for(int i=0; i<N; i++){
      int b, e;
      scanf("%d %d %d", &b, &e, &v[i]);
      len[v[i]] += e - b;
      rem -= (e - b);
      maxv = max(maxv, v[i]);
    }

    //printf("lens: %d %d\n", len[0], len[1]);
    len[0] += rem;

    double rt = T, tt = 0;
    for(int i = 0; i <= maxv; i++){
      if (len[i] == 0) continue;
      double t2;
      if(len[i] < rt*(R + i)){
	t2 = len[i]*1.0/(R + i);
	rt -= t2;
      }
      else {
	t2 = rt + (len[i] - rt*(R + i))*1.0/(S + i);
	//printf("len:%d R+i:%d %.2lf\n", len[i], R+i, (len[i] - rt*(R + i))*1.0/(S + i));
	rt = 0;
      }
      //printf("t2: %.2lf\n", t2);
      tt += t2;
    }
    printf("Case #%d: %.9f\n", cases, tt);

  }

  return 0;
}
