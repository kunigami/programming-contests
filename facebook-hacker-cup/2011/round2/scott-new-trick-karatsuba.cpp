#include <iostream>
#include <vector>
#include <cassert>

using namespace std;

typedef long long i64;

vector<int> gpow, gpowinv;
void CalcGen(int P)
{
    gpow.resize(P);
    while(1){
        i64 G = rand() % (P - 1) + 1;
        gpow[0] = 1;
        bool ok = true;
        for(int i=1; i<P-1; i++){
            gpow[i] = gpow[i-1]*G % P;
            if(gpow[i] == 1){
                ok = false;
                break;
            }
        }
        gpow[P-1] = 1;
        if(ok) break;
    }
    gpowinv.resize(P);
    for(int i=0; i<P-1; i++) gpowinv[gpow[i]] = i;
}
int ReadInput(int P, i64 &fx0, vector<i64> &fx)
{
    i64 n, x0, x1, a, b, c, r;
    cin >> n >> x0 >> x1 >> a >> b >> c;
    for(int i=0; i<n; i++){
        if(i == 0) r = x0;
        else if(i == 1) r = x1;
        else {
            r = (x0*a + x1*b + c) % P;
            x0 = x1; 
            x1 = r;
        }
        r == 0 ? fx0++ : fx[gpowinv[r]]++;
    }
    return n;
}

typedef vector<i64> Poly;

/**  
 * Karatsuba algorithm. Fast multiplication of the first n
 * positions of fa and fb.
 * PS. Assumes n is power of 2 
 **/
void Karatsuba(int n, i64 *fa, i64 *fb, Poly &fab)
{
    int m = n/2;
    /* Base */ 
    if(n <= 16){
        fab.assign(2*n - 1, 0);
        for(int i=0; i<n; i++)
            for(int j=0; j<n; j++)
                fab[i+j] += fa[i]*fb[j];
        return;
    }
    Poly z0, z1, z2, tmp;
    /* Find z0 and z2 recursively */
    Karatsuba(m, &fa[0], &fb[0], z0);
    Karatsuba(m, &fa[m], &fb[m], z2);
    /* Find z1 recursively */
    Poly fai(m), fbi(m);
    for(int i=0; i<m; i++){
        fai[i] = fa[i] + fa[i+m];
        fbi[i] = fb[i] + fb[i+m];
    }
    Karatsuba(m, &fai[0], &fbi[0], z1);
    for(int i=0; i<z1.size(); i++)
        z1[i] -= (z0[i] + z2[i]);
    /* Merge z0, z1 and z2 in fab */
    fab.assign(2*m + z2.size(), 0);
    for(int i=0; i<z0.size(); i++){
        fab[i] += z0[i];
        fab[i+m] += z1[i];
        fab[i+2*m] += z2[i];
    }
}

i64 Count(int L, int P, Poly &fab)
{
    i64 r = 0;
    for(int i=0; i<fab.size(); i++)
        r += gpow[i % (P-1)] < L ? fab[i] : 0;
    return r;
}
int main ()
{
    int T;
    cin >> T;
    while (T--){
        int P, L;
        cin >> P >> L;
        CalcGen(P);
        /* Make sure fa and fb have size power of 2 */
        int nextP = 1;
        while(nextP < P) nextP *= 2;
        /* Read input and fill in the freq. vectors */
        i64 fa0 = 0, fb0 = 0;
        vector<i64> fa(nextP), fb(nextP);
        i64 nA = ReadInput(P, fa0, fa);
        i64 nB = ReadInput(P, fb0, fb);
        /* Fast multiplication of vectors fa and fb */
        vector<i64> fab;
        Karatsuba(fa.size(), &fa[0], &fb[0], fab);
        /* Trivial case */
        i64 r = fa0*nB + fb0*nA - fa0*fb0;
        /* Actual count */
        r += Count(L, P, fab);
        cout << r << endl;
    }
    return 0;
}
