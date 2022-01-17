/*
 * AUTHORS
 *
 * The Veracruz Development Team.
 *
 * COPYRIGHT
 *
 * See the `LICENSE_MIT.markdown` file in the Veracruz root directory for licensing
 * and copyright information.
 *
 */

#ifndef _STDLIB_H
#define _STDLIB_H

#pragma once

#include <stddef.h>

#define EXIT_SUCCESS 0
#define EXIT_FAILURE 1

int atoi(const char *nptr);
long atol(const char *nptr);
long long atoll(const char *nptr);
void *calloc(size_t nmemb, size_t size);
void exit(int status);
void free(void *ptr);
void *malloc(size_t size);
int rand(void);
int rand_r(unsigned int *seedp);
void *realloc(void *ptr, size_t size);
void srand(unsigned int seed);

#endif // _STDLIB_H
