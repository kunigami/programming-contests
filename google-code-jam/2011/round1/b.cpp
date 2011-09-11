#include <stdio.h>
#include <iostream>
#include <set>

using namespace std;

typedef long long int i64;

#define MAXN 1234123
i64 v[MAXN], dist[MAXN];

int main (){

  i64 t;
  i64 best;

  scanf("%lld", &t);
  for(i64 cases=1; cases<=t; cases++){
    i64 L, T, n, c;
    scanf("%lld %lld %lld %lld", &L, &T, &n, &c);

    for(i64 i=0; i<c; i++)
      scanf("%lld", &v[i]);
    for(i64 i=c; i<n; i++)
      v[i] = v[i % c];

    /*
    cout << "v[ ]: ";
    for(i64 i=0; i<n; i++)
      cout << v[i] << " ";
    cout << endl;
    */

    i64 mindist = T/2;

    memset(dist, 0, sizeof(dist));

    for(i64 i=1; i<=n; i++){
      dist[i] = dist[i-1] + v[i-1];
    }

    printf("Case #%lld: ", cases);
    if (L == 0){
      printf("%lld\n", dist[n]*2);
      continue;
    }
    /*
    cout << "dist[ ]: ";
    for(i64 i=0; i<=n; i++)
      cout << dist[i] << " ";
    cout << endl;
    */

    i64 gain = 0;
    i64 best = 0;
    set< pair<i64,i64> > s;
    for(i64 i=n-1; i>=0; i--){
      
      if (dist[i+1] < mindist) break;
      if (dist[i] < mindist)
	best = max(best, gain + v[i] - (mindist - dist[i]));
      else 
	best = max(best, gain + v[i]);

      //printf("gain: %lld\n", gain);
      
      if (dist[i] < mindist) break;

      // insert v[i]
      if (s.size() < L - 1){
	s.insert(make_pair(v[i], i));
	gain += v[i];
      }
      else if (s.size() > 0) {
	set<pair<i64, i64> >::iterator it = s.begin();
	pair<i64, i64>  mini = *it;
	if (mini.first < v[i]){
	  s.erase(s.begin());
	  gain -= mini.first;
	  s.insert(make_pair(v[i], i));
	  gain += v[i];
	}
      }
    }
    printf("%lld\n", dist[n]*2 - best);
  }
}
