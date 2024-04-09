#include<bits/stdc++.h>

// На вход — два вектора (это нулевая и единичная остаточные функции по некоторому аргументу),
// номер аргумента, на выход — вектор функции.
int main() {

    /*
    0000
    1100
    0

    res : 0101'0000
    */

    /*

    1101
    1011
    2

    1110
    0111
    1

    res : 11011011

    */

    std::array<std::string, 2> a;
    std::array<int, 2> cur{0,0};
    std::cin>>a[0]>>a[1];

    int n = a[0].size();
    int num;
    std::cin>>num;
    int len = (1LL<<num);

    for(int i = 0; i < 2*n; i++) {
        std::cout << a[i/len & 1][ cur[i/len & 1]++ ];
    }

}
