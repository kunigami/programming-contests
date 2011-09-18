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

#define MAXN 1234

int adj[MAXN][MAXN];    // adjacency matrix
int dist[MAXN];         // distance from the source
int chosen[MAXN];       // vertices of the path
int n;                  // number of vertices
int max_threat;
int freq[MAXN];

int bfs(int src, int *dist){

    for(int i=0; i<n; i++)
        dist[i] = -1;

    queue<int> q;

    q.push(src);
    dist[src] = 0;
    while(!q.empty()){

        int v = q.front(); 
        q.pop();

        for(int u = 0; u < n; u++){
            if (adj[v][u] && dist[u] == -1){
                dist[u] = dist[v] + 1;
                q.push(u);
            }
        }
    }
}

void most_threat(int v, int lvl, int sp){
    
    if(lvl == sp && adj[1][v]){
        memset(freq, 0, sizeof(freq));
        for(int i=0; i < sp; i++){
            int v = chosen[i];
            // union of neighborhoods
            for(int u=0; u<n; u++)
                freq[u] += adj[v][u];
            int threat = 0;
            for(int u=1; u<n; u++)
                threat += (freq[u]) ? 1 : 0;
            max_threat = max(max_threat, threat);
        }
        return;
    }
    for(int u = 0; u < n; u++){
        if(adj[v][u] && dist[u] == lvl){
            chosen[lvl] = u;
            most_threat(u, lvl+1, sp);
        }
    }
}

int main (){

    int t;
    cin >> t;
    for(int cases = 1; cases <= t; cases++){
        int w;
        cin >> n >> w;

        for(int i=0; i<n; i++)
            for(int j=0; j<n; j++)
                adj[i][j] = 0;

        for(int j=0; j<w; j++){
            int src, dst;
            scanf("%d,%d", &src, &dst);
            adj[src][dst] = adj[dst][src] = 1;
        }

        bfs(0, dist);

        max_threat = 0;
        chosen[0] = 0;
        int mindist = dist[1];
        most_threat(0, 1, mindist);
        
        printf("Case #%d: %d %d\n", cases, mindist-1, max_threat - mindist + 1);
    }

    return 0;
}
