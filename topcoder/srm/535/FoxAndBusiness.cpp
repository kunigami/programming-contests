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

#define MAXT 100123
#define MAXN 55
bool pd[MAXT], aux[MAXT];

class FoxAndBusiness
{
    public:

    double minimumCost(int K, int totalWork, vector <int> a, vector <int> p)
    {
        int n = a.size();
        double lo = 0, hi = 1e20;
        vector<double> cost(n);
        for(int i = 0; i < 1000; i++){
            double c = (hi + lo)/2;

            for(int j = 0; j < n; j++)
                cost[j] = a[j]*(c - p[j]);
            sort(cost.rbegin(), cost.rend());

            double s = 0;
            for(int j = 0; j < K; j++)
                s += cost[j];
            if(s >= K*3600) hi = c;
            else lo = c;
        }
        return hi*totalWork;
    }
};
