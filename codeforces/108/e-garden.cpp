#include <stdio.h>
#include <algorithm>
#include <iostream>
#include <vector>
#include <string.h>

using namespace std;

#define MAXN 300
#define MASK 200
#define INF 0x3f3f3f3f

// n x m board with costs
int b[MAXN][MAXN];
// serialized b
int B[MAXN];
// id of the point in the bitmask or -1 if its not a fixed point
int mask_id[MAXN];
// all pairs shortest paths costs
int adj[MAXN][MAXN];
// dynamic programming to avoid recomputation
int dp[MASK][MAXN];
// save the steps we took on dp to recover the path
int path[MASK][MAXN];
// output board 
char painted[MAXN][MAXN];
// fixed points
vector<int> pts;
// k: number of fixed points
// N: number of points (= n x m)
// n: lines of the board
// m: columns of the board
int k, N, n, m;

int floydWarshall(int n){
    for(int k=0; k<n; k++)
        for(int i=0; i<n; i++)
            for(int j=0; j<n; j++)
                adj[i][j] = min(adj[i][j], adj[i][k] + adj[k][j]);
}

int solve(int mask, int k1){

    // If k1 belongs to the fixed set, it must belong to the mask
    if((mask_id[k1] != -1 && !((1 << mask_id[k1]) & mask)) || dp[mask][k1] == -2 || mask == 0)
        return INF;

    if(dp[mask][k1] != -1)
        return dp[mask][k1];

    // Mark as processing this case
    dp[mask][k1] = -2;

    // Number of setted bits
    int bits = __builtin_popcount(mask);
    // Base: exactly one bit set and it's the pivot 
    if(bits == 1 && mask_id[k1] != -1){
        return dp[mask][k1] = B[k1];
    }

    int best = INF;

    // Iterate over subsets of mask
    int submask = mask;
    while(1) {
        submask = (submask - 1) & mask;      
        if(submask == 0) break;
        int complement = submask ^ mask; 
        // Counting the k1 weight twice. Remove one
        int ans = solve(submask, k1) + solve(complement, k1) - B[k1];
        if(ans < best){
            best = ans;
            path[mask][k1] = submask + N;
        }
    }
    
    // Pivot changing. If k1 belongs to the fixed points, remove it
    // from the mask
    int new_mask = mask;
    if(mask_id[k1] != -1)
        new_mask &= ~(1 << mask_id[k1]);

    // Choose a new pivot
    for(int k2 = 0; k2 < N; k2++){
        int ans = solve(new_mask, k2);
        ans += adj[k1][k2] - B[k2];
        if(ans < best){
            best = ans;
            path[mask][k1] = k2;
        }
    }
    return dp[mask][k1] = best;
}

void paint_point(int pivot){
    int pi = pivot / m;
    int pj = pivot % m;
    painted[pi][pj] = 'X';
}

void repaint_shortest_path(int src, int dst){
    if(src == dst) 
        return;

    for(int i = 0; i < N; i++){
        if(i == src || i == dst) continue;
 
        if(adj[src][i] + adj[i][dst] - B[i] == adj[src][dst]){
            
            paint_point(i);

            repaint_shortest_path(src, i);
            repaint_shortest_path(i, dst);
            return;
        }
    }
}

void repaint(int mask, int pivot){

    paint_point(pivot);

    if(path[mask][pivot] == -1) return;
        
    if(path[mask][pivot] < N){
        int new_pivot = path[mask][pivot];
        repaint_shortest_path(pivot, new_pivot);
        if(mask_id[pivot] != -1)
            mask &= ~(1 << mask_id[pivot]);
        repaint(mask, new_pivot);
    }
    else {
        int submask = path[mask][pivot] - N;
        repaint(submask, pivot);
        repaint(mask ^ submask, pivot);
    }
}


int main (){

    cin >> n >> m >> k;

    N = 0;
    for(int i = 0; i < n; i++)
        for(int j = 0; j < m; j++){
            cin >> b[i][j];
            B[N++] = b[i][j];
        }

    // Fixed points
    memset(mask_id, -1, sizeof(mask_id));
    pts = vector<int>(k);
    for(int l = 0; l < k; l++){
        int i, j;
        cin >> i >> j;
        i--; j--;
        pts[l] = i*m + j;
        mask_id[pts[l]] = l;        
    }
    
    memset(adj, INF, sizeof(adj));
    // Build wighted adjacency matrix
    // Note: nm <= 200
    int di[] = {-1, 0, 1, 0};
    int dj[] = {0, -1, 0, 1};
    for(int i1 = 0; i1 < n; i1++)
        for(int j1 = 0; j1 < m; j1++){
            int k1 = i1*m + j1;
            adj[k1][k1] = b[i1][j1];
            for(int r = 0; r < 4; r++){
                int i2 = i1 + di[r];
                int j2 = j1 + dj[r];
                if(i2 >= 0 && i2 < n && j2 >= 0 && j2 < m){
                    int k2 = i2*m + j2;
                    // Going from k1 to k2
                    adj[k1][k2] = b[i2][j2];                
                }
            }
        }

    // all pairs shortest path
    floydWarshall(n*m);

    // weights are on vertices. add the initial weight
    for(int i1 = 0; i1 < n; i1++)
        for(int j1 = 0; j1 < m; j1++)
            for(int k2 = 0; k2 < N; k2++){
                int k1 = i1*m + j1;
                if(k1 != k2){
                    adj[k1][k2] += b[i1][j1];
                }
            }

    memset(dp, -1, sizeof(dp));
    memset(path, -1, sizeof(path));

    int size = (1<<k);
    int best_pivot = -1;
    int best = INF;
    for(int k1 = 0; k1 < N; k1++){
        int ans = solve(size-1, k1);
        if(ans < best){
            best = ans;
            best_pivot = k1;
        }
    }

    cout << best << endl;

    int mask = size-1;
    int pivot = best_pivot;

    for(int i = 0; i < n; i++)
        for(int j = 0; j < m; j++)
            painted[i][j] = '.';

    repaint(size-1, best_pivot);

    for(int i = 0; i < n; i++){
        for(int j = 0; j < m; j++)
            printf("%c", painted[i][j]);
        printf("\n");
    }

    return 0;
}
