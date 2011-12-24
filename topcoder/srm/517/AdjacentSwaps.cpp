#include <cassert>
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

using namespace std;

#define MAXN 123
#define MOD 1000000007

typedef long long i64;

class AdjacentSwaps
{

    int N;
    vector<int> p;
    int isPmt[MAXN][MAXN][MAXN];
    i64 bin[MAXN][MAXN], t[MAXN][MAXN];

    i64 Binomial(int n, int r){
        if(r == 0 || n == r) return 1;
        if(bin[n][r] != -1) return bin[n][r];
        return bin[n][r] = (Binomial(n - 1, r - 1) + Binomial(n - 1, r)) % MOD;
    }
    void BuildIsPmt(){

        for(size_t i = 0; i < N; i++){

            vector<int> q(N);
            for(size_t j = i + 1; j < N; j++){

                // linear sort
                vector<int> freq(N);
                for(int k = i; k <= j; k++)
                    freq[p[k]]++;              
                int cnt = i;
                for(int k = 0; k < N; k++)
                    if(freq[k]) q[cnt++] = k;

                vector<int> vis(N);
                vis[p[i]]++;
                isPmt[i][j][i] = (--vis[q[i+1]] >= 0);

                for(int k = i + 1; k < j; k++){
                    vis[p[k]]++;  // add p[k]
                    vis[q[k]]++; // remove q[k]
                    isPmt[i][j][k] = (--vis[q[k-1]] >= 0); // add q[k-1]
                    isPmt[i][j][k] &= (--vis[q[k+1]] >= 0); // add q[k+1]
                }
            }
        }
    }
    i64 Solve(int i, int j){

        if(i == j) return 1;

        if(t[i][j] != -1) return t[i][j];
        t[i][j] = 0;

        for(int k = i; k < j; k++){
            if(isPmt[i][j][k]){
                i64 pt = (Solve(i, k) * Solve(k + 1, j)) % MOD * Binomial(j - i - 1, k - i) % MOD;
                t[i][j] = (t[i][j] + pt) % MOD;         
            }
        }
        return t[i][j];
    }

    public:

    int theCount(vector <int> _p)
    {
        N = _p.size();
        p = _p;

        memset(isPmt, -1, sizeof(isPmt));
        memset(bin, -1, sizeof(bin));
        memset(t, -1, sizeof(t));
        BuildIsPmt();
        return(Solve(0, N-1));
    }
    
// BEGIN CUT HERE
	public:
	void run_test(int Case) { if ((Case == -1) || (Case == 0)) test_case_0(); if ((Case == -1) || (Case == 1)) test_case_1(); if ((Case == -1) || (Case == 2)) test_case_2(); if ((Case == -1) || (Case == 3)) test_case_3();
            if ((Case == -1) || (Case == 4)) test_case_4();
            if ((Case == -1) || (Case == 5)) test_case_5();
        }
	private:
	template <typename T> string print_array(const vector<T> &V) { ostringstream os; os << "{ "; for (typename vector<T>::const_iterator iter = V.begin(); iter != V.end(); ++iter) os << '\"' << *iter << "\","; os << " }"; return os.str(); }
	void verify_case(int Case, const int &Expected, const int &Received) { cerr << "Test Case #" << Case << "..."; if (Expected == Received) cerr << "PASSED" << endl; else { cerr << "FAILED" << endl; cerr << "\tExpected: \"" << Expected << '\"' << endl; cerr << "\tReceived: \"" << Received << '\"' << endl; } }
	void test_case_0() { int Arr0[] = {1, 2, 0}; vector <int> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); int Arg1 = 1; verify_case(0, Arg1, theCount(Arg0)); }
	void test_case_1() { int Arr0[] = {0, 1}; vector <int> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); int Arg1 = 0; verify_case(1, Arg1, theCount(Arg0)); }
	void test_case_2() { int Arr0[] = {2, 0, 3, 1}; vector <int> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); int Arg1 = 2; verify_case(2, Arg1, theCount(Arg0)); }
	void test_case_3() { int Arr0[] = {1, 0, 3, 2}; vector <int> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); int Arg1 = 0; verify_case(3, Arg1, theCount(Arg0)); }
	
    void test_case_4() { 
        int Arr0[] = {1, 3, 0, 5, 2, 7, 4, 8, 10, 6, 12, 9, 14, 11, 16, 13, 18, 15, 19, 17}; vector <int> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); int Arg1 = 716743312; verify_case(4, Arg1, theCount(Arg0)); }

    void test_case_5(){
        int Arr0[] = {2, 34, 23, 28, 32, 25, 12, 40, 33, 10, 35, 39, 20, 19, 0, 48, 43, 7, 38, 16, 1, 3, 18, 11, 5, 36, 49, 44, 31, 22, 6, 13, 37, 15, 29, 21, 9, 45, 4, 42, 14, 41, 8, 46, 26, 24, 47, 30, 27, 17};
        vector <int> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); 
        int Arg1 = 0; 
        verify_case(5, Arg1, theCount(Arg0));
    }

// END CUT HERE

};

// BEGIN CUT HERE
int main()
{
    AdjacentSwaps ___test;
    ___test.run_test(-1);
}
// END CUT HERE
