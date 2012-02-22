#include <vector>
#include <stdio.h>
#include <string>
#include <iostream>
#include <algorithm>

using namespace std;

typedef long long i64;

i64 mod = 1000000007LL;

int main (){

    int n, m;
    cin >> n >> m;

    vector<string>s(n);
    for(int i = 0; i < n; i++)
        cin >> s[i];

    i64 res = 1;
    for(int j = 0; j < m; j++){
        vector<int> v(100);
        for(int i = 0; i < n; i++)
            v[s[i][j]-'A'] = 1;
        int cnt = 0;
        for(int i = 0; i < 100; i++)
            if(v[i]) cnt++;
        
        res = (res * cnt) % mod;        
    }
    cout << res << "\n";
            
    return 0;
}
