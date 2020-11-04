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
void dump_binary_complex(char *in, int length) {


    for (unsigned long address = 0; address < length; address += 16) {
        int nread = 0;
        char buf[16] = {0};

        while(nread < 16) {
            if ((address + nread) < length) {
                buf[nread] = in[address + nread];
            } else {
                break;
            }
            nread++;
        }

        if (nread == 0) { break; }

        // Show the address
        printf("%08lx ", address);

        // show the hex codes
        for (int i = 0; i < 16; i++) {
            if (i < nread) {
                printf(" %02x", (unsigned int) (unsigned char) buf[i]);
            } else {
                printf("   ");
            }
        }

        printf("  ");

        // show printable characters
        for (int i = 0; i < 16; i++) {
            if (i < nread) {
                if (buf[i] >= 32) {
                    printf("%c", buf[i]);
                } else {
                    printf(".");
                }
            } else {
                printf(" ");
            }
        }

        printf("\n");

    }


}


/***
 * Print a binary view of a buffer of 1-byte `char`s
 * @param buffer A buffer of `char`
 */
void dump_binary_simple(char *buffer, int length) {

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
             << (uint) (u_char) c;

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

    char buffer[] = "\x01\x02\x03\x04\x05\x06\xFFhello\20world. I'm an array of characters. Woo.";
    buffer[0] = 126;
//    dump_binary_simple(buffer, sizeof(buffer));
    dump_binary_complex(buffer, sizeof(buffer));

    cout << endl;

    FILE *fp;
    int c, i, max;

    char filepath[] = "../../set-01/challenge-01/input";

    fp = fopen(filepath, "rb");
    if (fp == NULL) {
        fprintf(stderr, "Cannot open input file %s!", filepath);
    }

    // todo process file

}


