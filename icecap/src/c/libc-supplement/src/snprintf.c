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

#include <stdio.h>

int snprintf(char *str, size_t size, const char *format, ...) {
    // HACK instead of implementing sprintf, just:
    //  - log when it's used (so far it isn't), and
    //  - copy the format string directly
    printf("snprintf(_, _, \"%s\")\n", format);
    strcpy(str, format);
    return 0;
}

FILE *stdin = 0;
FILE *stdout = 0;
FILE *stderr = 0;

void __assert_fail(const char  *str, const char *file, int line, const char *function)
{
}
