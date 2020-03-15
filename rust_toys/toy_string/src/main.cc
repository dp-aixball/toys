//
// Created by zhyi on 2020-03-08.
//

#include <iostream>
#include <vector>
#include "qsplit.h"

using namespace std;

int main() {

    std::string s = "一二三。四五六！";
    for( int lll = 0; lll < 1000; lll++)
    {
        auto sents = long_text_split_ffi( s.c_str(), "。?？!！", ",，", 32 );
    }

    cout<< "hhh" <<"\n";

    return 0;
}