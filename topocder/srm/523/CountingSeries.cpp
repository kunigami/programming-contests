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
#include <algorithm>

using namespace std;

typedef long long i64;

class CountingSeries
{
    public:
    long long countThem(long long a, long long b, long long c, long long d, long long ub)
    {

        i64 cnt;
        i64 xlb, xub;

        if(ub < a){
            cnt = 0;
            xlb = 1;
            xub = -1;
        }
        else {
            xlb = max((b - a)/b, 0LL); 
            xub = max((ub - a)/b, 0LL);
            //cout << "xlb: " << xlb << endl;
            //cout << "xub: " << xub << endl;
            cnt = xub - xlb + 1;
        }

        i64 dpot = 1;
        for(i64 y=0; ; y++){
            
            i64 r = dpot * c;
            if(r > ub) break;
            
            cnt++;
            if ((r - a) % b == 0){
                
                i64 x = (r - a)/b;
                if(x >= xlb && x <= xub) cnt--;
            } 
            dpot *= d;
            if(d == 1) break;
        }
        return cnt;
    }
    
// BEGIN CUT HERE
	public:
	void run_test(int Case) { if ((Case == -1) || (Case == 0)) test_case_0(); if ((Case == -1) || (Case == 1)) test_case_1(); if ((Case == -1) || (Case == 2)) test_case_2(); if ((Case == -1) || (Case == 3)) test_case_3();
            if ((Case == -1) || (Case == 4)) test_case_4(); 
            if ((Case == -1) || (Case == 5)) test_case_5(); 
            if ((Case == -1) || (Case == 6)) test_case_6(); 
        }
	private:
	template <typename T> string print_array(const vector<T> &V) { ostringstream os; os << "{ "; for (typename vector<T>::const_iterator iter = V.begin(); iter != V.end(); ++iter) os << '\"' << *iter << "\","; os << " }"; return os.str(); }
	void verify_case(int Case, const long long &Expected, const long long &Received) { cerr << "Test Case #" << Case << "..."; if (Expected == Received) cerr << "PASSED" << endl; else { cerr << "FAILED" << endl; cerr << "\tExpected: \"" << Expected << '\"' << endl; cerr << "\tReceived: \"" << Received << '\"' << endl; } }
	void test_case_0() { long long Arg0 = 1LL; long long Arg1 = 1LL; long long Arg2 = 1LL; long long Arg3 = 2LL; long long Arg4 = 1000LL; long long Arg5 = 1000LL; verify_case(0, Arg5, countThem(Arg0, Arg1, Arg2, Arg3, Arg4)); }
	void test_case_1() { long long Arg0 = 3LL; long long Arg1 = 3LL; long long Arg2 = 1LL; long long Arg3 = 2LL; long long Arg4 = 1000LL; long long Arg5 = 343LL; verify_case(1, Arg5, countThem(Arg0, Arg1, Arg2, Arg3, Arg4)); }
	void test_case_2() { long long Arg0 = 40LL; long long Arg1 = 77LL; long long Arg2 = 40LL; long long Arg3 = 100000LL; long long Arg4 = 40LL; long long Arg5 = 1LL; verify_case(2, Arg5, countThem(Arg0, Arg1, Arg2, Arg3, Arg4)); }
	void test_case_3() { long long Arg0 = 452LL; long long Arg1 = 24LL; long long Arg2 = 4LL; long long Arg3 = 5LL; long long Arg4 = 600LL; long long Arg5 = 10LL; verify_case(3, Arg5, countThem(Arg0, Arg1, Arg2, Arg3, Arg4)); }
	void test_case_4() { long long Arg0 = 234LL; long long Arg1 = 24LL; long long Arg2 = 377LL; long long Arg3 = 1LL; long long Arg4 = 10000LL; long long Arg5 = 408LL; verify_case(4, Arg5, countThem(Arg0, Arg1, Arg2, Arg3, Arg4)); }


    void test_case_5() { long long Arg0 = 10000LL; long long Arg1 = 1LL; long long Arg2 = 1LL; long long Arg3 = 1LL; long long Arg4 = 1LL; long long Arg5 = 1; verify_case(4, Arg5, countThem(Arg0, Arg1, Arg2, Arg3, Arg4)); }
    void test_case_6() { long long Arg0 = 1LL; long long Arg1 = 1LL; long long Arg2 = 10LL; long long Arg3 = 1LL; long long Arg4 = 1LL; long long Arg5 = 1; verify_case(4, Arg5, countThem(Arg0, Arg1, Arg2, Arg3, Arg4)); }

// END CUT HERE

};

// BEGIN CUT HERE
int main()
{
    CountingSeries ___test;
    ___test.run_test(-1);
}
// END CUT HERE
