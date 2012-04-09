#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <deque>
#include <queue>
#include <stack>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <set>

using namespace std;

#define MAXN 123

typedef pair<int,int> pii;

class MagicBoard
{

    struct pt {
        int y, x;
        pt(int y=0, int x=0) : y(y), x(x) {}
        bool operator==(pt q) const { return y == q.y && x == q.x; }
        bool operator< (pt q) const { return y != q.y ? y < q.y : x < q.x; }
    };
    bool vis[MAXN][MAXN];

    vector<string> b;
    vector<int> cn, rn;
    int r, c;           // number of rows and cols
    int n;              // number of diamonds

    bool all_even(){        
        for(int i = 0; i < r; i++)
            if(rn[i] % 2) return false;
        for(int j = 0; j < c; j++)
            if(cn[j] % 2) return false;         
        return true;
    }
    

    bool decide_even(pt s, pt t){
        cn[t.x]--;        
        bool st = all_even();
        cn[t.x]++;
        return st;
    }
    bool decide_odd(pt s, pt t){
        rn[t.y]--;
        bool st = all_even();
        rn[t.y]++;
        return st;
    }
    void dfs(pt v){
        if(vis[v.y][v.x]) return;
        vis[v.y][v.x] = true;

        for(int x=0; x<c; x++)
            if(b[v.y][x] == '#')
                dfs(pt(v.y, x));
        for(int y=0; y<r; y++)
            if(b[y][v.x] == '#')
                dfs(pt(y, v.x));
    }
    bool is_connected(vector<pt> &pts){
        memset(vis, 0, sizeof(vis));
        dfs(pts[0]);
        for(int i=0; i<pts.size(); i++)
            if(vis[pts[i].y][pts[i].x] == 0)
                return false;
        return true;
    }
    bool decide(){
        r = b.size();
        c = b[0].length();
        vector< pt > pts;
        cn = vector<int>(c);
        rn = vector<int>(r);

        for(int i = 0; i < r; i++)
            for(int j = 0; j < c; j++)
                if(b[i][j] == '#'){
                    rn[i]++;
                    cn[j]++;
                    pts.push_back(pt(i,j));
                }

        n = pts.size();

        // Special case
        if(n == 1) return true;

        // Check if graph is connected
        if (!is_connected(pts))
            return false;
        
        for(int k = 0; k < pts.size(); k++) // starting point
            for(int l = 0; l < pts.size(); l++){ // end point
                if(k == l) continue;
                cn[pts[k].x]++;
                if((n % 2 && decide_odd(pts[k], pts[l])) ||
                   (n % 2 == 0 && decide_even(pts[k], pts[l])))
                    return true;
                cn[pts[k].x]--;
            }
        return false;
    }

    public:
    string ableToUnlock(vector <string> bd)
    {
        b = bd;
        return decide() ? "YES" : "NO";
    }
};
