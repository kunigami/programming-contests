/* A.I. WAR
 *
 * Solution for the small case
 *
 */
#include <algorithm>
#include <iostream>
#include <queue>
#include <stdio.h>
using namespace std;

#define MAXN 512
#define MAXM 5123

int adj[MAXN][MAXN];        // adjacency matrix

int dist[MAXN];             // distance from source (vertex 0)

int n;                      // number of vertices
int m;                      // number of edges

pair<int, int> edges[MAXM]; // list of edges

int map_edge[MAXN][MAXN];   // map a pair of vertices (a, b) into the
                            // index of the corresponding edge in edges

int G[MAXN][MAXM];          // G[i][j] counts the number of vertices
                            // that are adjacent to i but not to any
                            // of the endpoints of edge j

int pd[MAXN][MAXN];         

int bfs(int src, int *dist){

    for(int i=0; i<n; i++)
        dist[i] = -1;

    queue<int> q;

    q.push(src);
    dist[src] = 0;
    while(!q.empty()){

        int v = q.front(); 
        q.pop();

        for(int u=0; u<n; u++){
            if(!adj[v][u]) continue;
            if (dist[u] == -1){
                dist[u] = dist[v] + 1;
                q.push(u);
            }
        }
    }
}

/* most_threat computes the maximum number of threats in a path
   finished in (b -> c) */
int most_threat(int b, int c){

    if(b == 0){
        int threat = 0;
        for(int i=1; i<n; i++)
            if(adj[0][i] || adj[c][i]) 
                threat++;
        return threat;
    }
    if(pd[b][c] != -1) return pd[b][c];

    int best = 0;
    for(int a = 0; a < n; a++){
        if(adj[a][b] && dist[a] + 1 == dist[b])
            best = max(best, most_threat(a, b) + G[c][map_edge[a][b]]);
    }
    return pd[b][c] = best;
}
/* Pre-computes the matrix G[a][(b,c)], which represents the number of
 * neighbors of 'a' such that they are not neighbors of 'b' or
 * 'c'. The second parameter is indexed by edge to use O(NM) space
 * instead of O(N^3)
 */
void pre_compute(){

    for(int i = 0; i < n; i++)
        for(int j = 0; j < 2*m; j++)
            G[i][j] = 0;

    for(int i=0; i < 2*m; i++)
        for(int j=0; j < 2*m; j++){
            if(i == j) continue;
            int a = edges[i].first;
            int v = edges[i].second;
            int b = edges[j].first;
            int c = edges[j].second;
            if(!adj[v][b] && !adj[v][c])
                G[a][j]++;
        }
}

int main (){

    int t;
    cin >> t;
    for(int cases = 1; cases <= t; cases++){

        cin >> n >> m;

        for(int i=0; i<n; i++)
            for(int j=0; j<n; j++)
                adj[i][j] = 0;

        for(int j=0; j<m; j++){
            int src, dst;
            scanf("%d,%d", &src, &dst);
            adj[src][dst] = adj[dst][src] = 1;
            map_edge[src][dst] = 2*j;
            edges[2*j] = make_pair(src, dst);
            map_edge[dst][src] = 2*j + 1;
            edges[2*j + 1] = make_pair(dst, src);
        }

        // Special case, adj[0][1] = 1
        if(adj[0][1]){
            int threat = 0;
            for(int i = 0; i < n; i++)
                threat += (adj[0][i]) ? 1 : 0;
            printf("Case #%d: 0 %d\n", cases, threat);
            continue;
        }

        bfs(0, dist);
        int mindist = dist[1];

        pre_compute();

        for(int i = 0; i < n; i++)
            for(int j = 0; j < n; j++)
                pd[i][j] = -1;

        int max_threat = 0;
        for(int j = 0; j < 2*m; j++){
            int a = edges[j].first;
            int b = edges[j].second;
            
            if(dist[a] + 1 == dist[b] && dist[b] == mindist - 1 && adj[b][1]){
                max_threat = max(max_threat, most_threat(a, b));
            }
        }
        
        printf("Case #%d: %d %d\n", cases, mindist-1, max_threat - mindist + 1);
    }

    return 0;
}
