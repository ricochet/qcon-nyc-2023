#include "critter.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void exports_component_example_greeting_hello(critter_string_t *greeting, critter_string_t *ret) {
    critter_string_t a;
    critter_string_set(&a, "\nGreetings from C! 🐱");

    // append critter to greeting and return
    strcpy(ret->ptr, a.ptr);
    critter_string_free(&a);
}
