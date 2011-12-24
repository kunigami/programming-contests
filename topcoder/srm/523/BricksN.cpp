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

typedef long long i64;
#define MOD 1000000007            
#define MAXW 123

class BricksN
{
    public:

    // m[w][h]: how many ways to build a tower of length w and height h
    i64 m[MAXW][MAXW];
    // complete[w]: counts in how many ways we can build a complete line of length w (no holes)
    i64 complete[MAXW];
    // sizes of blocks
    int k; 

    void calc_complete(int w)
    {
        memset(complete, 0, sizeof(complete));
        complete[0] = 1;
        for(int j = 0; j <= w; j++)
            for(int i = 1; i <= k; i++)
                if(j-i >= 0) complete[j] = (complete[j] + complete[j-i]) % MOD;
    }

    i64 pd(int w, int h)
    {    
        // Base
        if(w <= 0 || h == 0) return 1;
        // Memo
        if(m[w][h] != -1) return m[w][h];

        m[w][h] = 0;

        i64 rr = 0;
        // Complete in the first i positions,
        // hole in the (i+1)-th position
        for(int i = 0; i <= w; i++){
            i64 r = complete[i] * pd(i, h-1) % MOD;
            r = r * pd(w-i-1, h) % MOD;
            rr = (rr + r) % MOD;
        }
        return m[w][h] = rr;
    }

    int countStructures(int w, int h, int _k)
    {
        memset(m, -1, sizeof(m));
        k = _k;
        calc_complete(w);
        return pd(w, h);
    }
    
// BEGIN CUT HERE
	public:
	void run_test(int Case) { if ((Case == -1) || (Case == 0)) test_case_0(); if ((Case == -1) || (Case == 1)) test_case_1(); if ((Case == -1) || (Case == 2)) test_case_2(); if ((Case == -1) || (Case == 3)) test_case_3(); }
	private:
	template <typename T> string print_array(const vector<T> &V) { ostringstream os; os << "{ "; for (typename vector<T>::const_iterator iter = V.begin(); iter != V.end(); ++iter) os << '\"' << *iter << "\","; os << " }"; return os.str(); }
	void verify_case(int Case, const int &Expected, const int &Received) { cerr << "Test Case #" << Case << "..."; if (Expected == Received) cerr << "PASSED" << endl; else { cerr << "FAILED" << endl; cerr << "\tExpected: \"" << Expected << '\"' << endl; cerr << "\tReceived: \"" << Received << '\"' << endl; } }
	void test_case_0() { int Arg0 = 3; int Arg1 = 1; int Arg2 = 3; int Arg3 = 13; verify_case(0, Arg3, countStructures(Arg0, Arg1, Arg2)); }
	void test_case_1() { int Arg0 = 3; int Arg1 = 2; int Arg2 = 3; int Arg3 = 83; verify_case(1, Arg3, countStructures(Arg0, Arg1, Arg2)); }
	void test_case_2() { int Arg0 = 1; int Arg1 = 5; int Arg2 = 1; int Arg3 = 6; verify_case(2, Arg3, countStructures(Arg0, Arg1, Arg2)); }
	void test_case_3() { int Arg0 = 10; int Arg1 = 10; int Arg2 = 3; int Arg3 = 288535435; verify_case(3, Arg3, countStructures(Arg0, Arg1, Arg2)); }

// END CUT HERE

};

// BEGIN CUT HERE
int main()
{
    BricksN ___test;
    ___test.run_test(-1);
}
// END CUT HERE
