#include "critter.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void exports_component_example_greeting_hello(critter_string_t *greeting, critter_string_t *ret) {
    critter_string_t a;
    critter_string_set(&a, "\nGreetings From 🐱 (C)!");

    ret->len = greeting->len + a.len;
    ret->ptr = malloc(ret->len * 2);

    // append critter to greeting and return
    strcpy(ret->ptr, greeting->ptr);
    strcat(ret->ptr, a.ptr);
    critter_string_free(&a);
    critter_string_free(greeting);
}
