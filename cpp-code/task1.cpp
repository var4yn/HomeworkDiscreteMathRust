#include<bits/stdc++.h>

// На вход — число n, на выход — булева функция от n аргументов
int main() {
    int n;
    std::cin>>n;

    for(int i = 0; i < (1LL<<n); i++) {
        // random 0 or 1
        std::cout << (rand()%2);
    }

}
