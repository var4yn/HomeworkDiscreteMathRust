#include<bits/stdc++.h>

// Игра. Узнать имя функции от 2-х аргументов.
// Система предлагает вектор функции, пользователь выбирает «имя» (одно из 16).
int main() {

    std::array<std::pair<std::string, std::string>, 16> funcs {
        {
            {"0000", "0"},
            {"0001", "x ∧ y"},
            {"0010", "x ↛ y"},
            {"0011", "x"},
            {"0100", "x ↚ y"},
            {"0101", "y"},
            {"0110", "x⊕y"},
            {"0111", "x ∨ y" },
            {"1000", "x ↓ y"},
            {"1001", "x = y"},
            {"1010", "¬y"},
            {"1011", "x ← y"},
            {"1100", "¬x"},
            {"1101", "x → y"},
            {"1110", "x ▽ y"},
            {"1111", "1"},
        }
    };

    int idx = rand()%16;
    std::cout << idx << "\n";

    auto [vfunc, name] = funcs[idx];



    std::cout << vfunc << "\n";
    for(int i = 0; i < 16; i++) {
        std::cout << funcs[i].second << " \n"[i != 0 && i%4==0];
    }
    std::cout << "\n";


    std::string ans;
    std::getline(std::cin, ans);

    bool ok = false;
    for(const auto& [func, name] : funcs) {
        if(func == vfunc) {
            std::cout << name << " " << ans << "\n";
            ok |= name == ans;
        }
    }

    std::cout << (ok ? "Yes\n" : "No\n");


}
