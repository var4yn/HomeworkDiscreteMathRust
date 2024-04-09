#include<bits/stdc++.h>

// На вход — вектор функции, 0 или 1, номер аргумента, на выход — соответствующая остаточная
int main() {
    std::string s;

    std::cin>>s;

    int c, num; // num : n, n-1, ... , 1, 0
    std::cin>>c>>num;

    //num = std::__lg(s.size()) - 1 - num;

    int len = (1LL<<num);
    for(int i = 0; i < s.size(); i++) {
        if( (i/len & 1) == c ) {
            std::cout << s[i];
        }

    }



}
