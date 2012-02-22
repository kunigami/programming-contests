#include <vector>
#include <stdio.h>
#include <string>
#include <iostream>
#include <algorithm>

using namespace std;

int main (){
    
    int n, m;
    cin >> n  >> m;
    vector<string> s(n);
    vector<int> max_grade(m);
    for(int i = 0; i < n; i++){
        cin >> s[i];
        for(int j = 0; j < m; j++){
            max_grade[j] = max(max_grade[j], s[i][j]-'0');
        }
    }
    int cnt = 0;
    for(int i = 0; i < n; i++){
        bool best = false;
        for(int j = 0; j < m; j++)
            if(s[i][j]-'0' == max_grade[j]){
                best = true;
                break;
            }
        if(best) cnt++;
    }
    cout << cnt << "\n";
    return 0;
}
