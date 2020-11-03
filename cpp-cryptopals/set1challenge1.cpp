#include <iostream>
#include <vector>
#include <string>
#include <fstream>
#include <filesystem>
#include <stdlib.h>//path stuff

using namespace std;


int main() {
    vector<string> msg{"Hello", "C++", "World", "from", "CLion"};

    for (const string &word : msg) {
        cout << word << " ";
    }

//    cout << filesystem::

    cout << endl;

//    cout << curren

    ifstream fileBuffer("../../set-01/challenge-01/input", ios::in | ios::binary);
    char input[1024];

//    char path[]="../../set-01/challenge-01/input";
//    cout << realpath(path,NULL);

    if(fileBuffer.is_open()){
        fileBuffer.seekg(0, ios::beg);
        fileBuffer.getline(input, 1024);
    }

    cout << input;


}


