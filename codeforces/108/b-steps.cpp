#include <vector>
#include <stdio.h>
#include <string>
#include <iostream>
#include <algorithm>

using namespace std;

typedef long long i64;

int main (){

    i64 w, h, x0, y0, dx, dy, sx, sy, ans;
    int k;

    cin >> w >> h;
    cin >> x0 >> y0;
    cin >> k;
    ans = 0;
    for(int i = 0; i < k; i++){
        cin >> dx >> dy;
        
        if(dx > 0)
            sx = (w - x0)/dx;
        else if(dx < 0)
            sx = (1 - x0)/dx;
        else
            sx = -1;

        if(dy > 0)
            sy = (h - y0)/dy;
        else if(dy < 0)
            sy = (1 - y0)/dy;
        else 
            sy = -1;

        if(sy == -1) sy = sx;
        if(sx == -1) sx = sy;
        i64 s = min(sx, sy);
        ans += s;
        x0 = x0 + dx*s;
        y0 = y0 + dy*s;
    }
    cout << ans << "\n";

    return 0;
} 
