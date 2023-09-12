#include <stdio.h>
#include <stdlib.h>
#include <time.h>


int randmy() {
    srand(time(NULL));
    return rand();

}