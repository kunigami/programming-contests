#include <iostream>
#include <vector>

using namespace std;

typedef long long i64;

bool is_prime[1123123];
vector<i64> primes;     // lista de primos

// Calcula os numeros primos ateh n (inclusive)
void Sieve(int n){
    is_prime[0] = is_prime[1] = false;
    is_prime[2] = true;
    for(int i = 3; i <= n; i++) 
        is_prime[i] = i % 2;
    for(int i = 3; i*i <= n; i+=2){
        if (is_prime[i])
            for(int j = i*i; j <= n; j+=i)
                is_prime[j] = false;
    }
    for(int i = 2; i <= n; i++)
        if (is_prime[i])
            primes.push_back(i);
}
i64 Discrete_log(i64 p, i64 n){
    i64 cnt = 0, prod = 1;
    while (prod <= n){
        prod *= p;
        cnt++;
    }
    return cnt - 1;
}

int main (){

    Sieve(1100000);

    int t;
    cin >> t;
    for(int cases = 1; cases <= t; cases++){

        i64 n, ans = 1;
        cin >> n;

        // Caso especial
        if (n == 1){
            cout << "Case #" << cases << ": "  << 0 << "\n";
            continue;
        }

        // Conta as potencias dos fatores primos p**e <= n, com e > 1
        for(int i = 0; primes[i]*primes[i] <= n; i++){
            ans += Discrete_log(primes[i], n) - 1;
        }

        cout << "Case #" << cases << ": "  << ans << "\n";
    }
    

    return 0;
}
