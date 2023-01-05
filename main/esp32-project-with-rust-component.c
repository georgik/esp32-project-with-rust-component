#include <stdio.h>
#include "example_rust.h"

void app_main(void)
{
    unsigned long x = add(1, 2);
    printf("Computed by Rust component %lu\n", x);
}
