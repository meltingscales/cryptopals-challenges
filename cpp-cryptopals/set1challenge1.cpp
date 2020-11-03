#include <iostream>
#include <iomanip>
#include <vector>
#include <string>
#include <fstream>
#include <filesystem>
#include <stdlib.h>//path stuff
#include <bitset>


using namespace std;

/**
 *
 * @param c Character.
 * @return Character if it is printable, space if not.
 */
char printable_ascii_only(char c) {
    if (c >= 32 && c <= 126) {
        return c;
    }
    return ' ';
}


bitset<8> charToBitstring(char c) {
    return bitset<8>(c);
}




//see https://stackoverflow.com/questions/16804251/how-would-i-create-a-hex-dump-utility-in-c
void dump_binary_complex(){
}

/***
 * Print a binary view of a buffer of 1-byte `char`s
 * @param buffer A buffer of `char`
 */
void dump_binary_simple(char* buffer, int length) {

//    cout << "size of buffer passed is "<<length;

    for (int i = 0; i < length; i++) {

        char c = buffer[i];

        cout << "buf[" << i << "] = 0x";

        ios state(nullptr);
        state.copyfmt(cout); // save current formatting

        cout << hex
        << uppercase
        << setw(2)
        << setfill('0')
        << (uint)(u_char) c;

        cout.copyfmt(state); //restore format

        cout << " | char: ";

         cout << "'" << printable_ascii_only(c) << "'" <<
             " | " << "0b" << charToBitstring(c) <<
             endl;
//        printf("buf[%d] = %d, '%c', 0b%s",i,buffer[i],printable_ascii_only(buffer[i]),charToBitstring(buffer[i]))


    }
}


int main() {
    vector<string> msg{"Hello", "C++", "World", "from", "CLion"};

    for (const string &word : msg) {
        cout << word << " ";
    }
    cout << "\n";

    char buffer[]= "\x01\x02\x03\x04\x05\x06\xFFhello\20world";
    dump_binary_simple(buffer, sizeof(buffer));


//    cout << filesystem::

    cout << endl;

//    cout << curren

    ifstream fileBuffer("../../set-01/challenge-01/input", ios::in | ios::binary);
    char input[1024];

//    char path[]="../../set-01/challenge-01/input";
//    cout << realpath(path,NULL);

    if (fileBuffer.is_open()) {
        fileBuffer.seekg(0, ios::beg);
        fileBuffer.getline(input, 1024);
    }

    cout << input;


}


