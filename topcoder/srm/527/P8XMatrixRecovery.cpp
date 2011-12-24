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

#define MAXN 100

class P8XMatrixRecovery
{

    int h, w;
    vector< string > rows;
    vector< string > cols;

    int conj[MAXN], vis[MAXN];
    bool mat[MAXN][MAXN];

    // Found a augmenting path
    bool augment(int u) {
        for (int v = 0; v < w; v++) {
            if(!mat[u][v]) continue;
            if (vis[v]) continue; 
            vis[v] = 1;
            if (conj[v] == -1 || augment(conj[v])) {
                conj[v] = u;
                return true;
            }
        }
        return false;
    }
    // Cardinality of maximum bipartite matching
    int maxbpm() {
        int res = 0;
        memset(conj, -1, sizeof(conj));
        for (int i = 0; i < w; i++) {
            memset(vis, 0, sizeof(vis));
            if (augment(i)) res++;
        }
        return res;
    }
    // Whether two columns matches
    bool matches(int c1, int c2){
        for(int k=0; k<h; k++){
            char a = rows[k][c1];
            char b = cols[c2][k];
            if(a != '?' && b != '?' && a != b) return false;
        }
        return true;
    }
    // Whether the current matrix 'rows' is feasible
    bool feasible(){

        // Build graph
        for(int i = 0; i < w; i++)
            for(int j = 0; j < w; j++)
                mat[i][j] = matches(i, j);

        return maxbpm() == w;
    }

public:
    
    vector <string> solve(vector <string> _rows, vector <string> _cols){

        h = _rows.size();
        w = _cols.size();
        rows = _rows;
        cols = _cols;
         
        for(int i = 0; i < h; i++)
            for(int j = 0; j < w; j++)
                if(rows[i][j] == '?'){
                    rows[i][j] = '0';
                    if (!feasible()) rows[i][j] = '1';
                }       
        return rows;
    }
    
// BEGIN CUT HERE
	public:
	void run_test(int Case) { if ((Case == -1) || (Case == 0)) test_case_0(); if ((Case == -1) || (Case == 1)) test_case_1(); if ((Case == -1) || (Case == 2)) test_case_2(); if ((Case == -1) || (Case == 3)) test_case_3(); }
	private:
	template <typename T> string print_array(const vector<T> &V) { ostringstream os; os << "{ "; for (typename vector<T>::const_iterator iter = V.begin(); iter != V.end(); ++iter) os << '\"' << *iter << "\","; os << " }"; return os.str(); }
	void verify_case(int Case, const vector <string> &Expected, const vector <string> &Received) { cerr << "Test Case #" << Case << "..."; if (Expected == Received) cerr << "PASSED" << endl; else { cerr << "FAILED" << endl; cerr << "\tExpected: " << print_array(Expected) << endl; cerr << "\tReceived: " << print_array(Received) << endl; } }
	void test_case_0() { string Arr0[] = {"10?"
,"?11"}; vector <string> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); string Arr1[] = {"01"
,"10"
,"1?"}
; vector <string> Arg1(Arr1, Arr1 + (sizeof(Arr1) / sizeof(Arr1[0]))); string Arr2[] = {"101", "011" }; vector <string> Arg2(Arr2, Arr2 + (sizeof(Arr2) / sizeof(Arr2[0]))); verify_case(0, Arg2, solve(Arg0, Arg1)); }
	void test_case_1() { string Arr0[] = {"0"
,"?"
,"1"}; vector <string> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); string Arr1[] = {"0?1"}; vector <string> Arg1(Arr1, Arr1 + (sizeof(Arr1) / sizeof(Arr1[0]))); string Arr2[] = {"0", "0", "1" }; vector <string> Arg2(Arr2, Arr2 + (sizeof(Arr2) / sizeof(Arr2[0]))); verify_case(1, Arg2, solve(Arg0, Arg1)); }
	void test_case_2() { string Arr0[] = {"10"
,"01"}; vector <string> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); string Arr1[] = {"10"
,"01"}; vector <string> Arg1(Arr1, Arr1 + (sizeof(Arr1) / sizeof(Arr1[0]))); string Arr2[] = {"10", "01" }; vector <string> Arg2(Arr2, Arr2 + (sizeof(Arr2) / sizeof(Arr2[0]))); verify_case(2, Arg2, solve(Arg0, Arg1)); }
	void test_case_3() { string Arr0[] = {"??0"
,"11?"
,"?01"
,"1?1"}; vector <string> Arg0(Arr0, Arr0 + (sizeof(Arr0) / sizeof(Arr0[0]))); string Arr1[] = {"1???"
,"?111"
,"0?1?"}; vector <string> Arg1(Arr1, Arr1 + (sizeof(Arr1) / sizeof(Arr1[0]))); string Arr2[] = {"010", "110", "101", "101" }; vector <string> Arg2(Arr2, Arr2 + (sizeof(Arr2) / sizeof(Arr2[0]))); verify_case(3, Arg2, solve(Arg0, Arg1)); }

// END CUT HERE

};

// BEGIN CUT HERE
int main()
{
    P8XMatrixRecovery ___test;
    ___test.run_test(-1);
}
// END CUT HERE
