#include <stdio.h>

char * vuoto() {
    return "FUNZIONA?";
}

int hello_world(char * str) {
    printf("%s\n", str);
}

int main(int argc, char ** argv) {
    hello_world("Hello, world!");

    return 0;
}