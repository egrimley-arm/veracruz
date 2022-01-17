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

#ifndef _STDIO_H
#define _STDIO_H

#pragma once

#include <stddef.h>
#include <icecap-utils.h>

#define printf icecap_utils_debug_printf

int snprintf(char *str, size_t size, const char *format, ...);

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

typedef void *FILE;
typedef long int fpos_t;

void clearerr(FILE *stream);
int fclose(FILE *stream);
FILE *fdopen(int fd, const char *mode);
int feof(FILE *stream);
int ferror(FILE *stream);
int fflush(FILE *stream);
int fgetc(FILE *stream);
int fgetpos(FILE *stream, fpos_t *pos);
char *fgets(char *s, int size, FILE *stream);
int fileno(FILE *stream);
FILE *fopen(const char *pathname, const char *mode);
size_t fread(void *ptr, size_t size, size_t nmemb, FILE *stream);
FILE *freopen(const char *pathname, const char *mode, FILE *stream);
int fscanf(FILE *stream, const char *format, ...);
int fseek(FILE *stream, long offset, int whence);
int fsetpos(FILE *stream, const fpos_t *pos);
long ftell(FILE *stream);
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream);
int getc(FILE *stream);
int getchar(void);
void perror(const char *s);
int remove(const char *pathname);
int rename(const char *oldpath, const char *newpath);
void rewind(FILE *stream);
int scanf(const char *format, ...);
int sscanf(const char *str, const char *format, ...);
int ungetc(int c, FILE *stream);

extern FILE *stdin;
extern FILE *stdout;
extern FILE *stderr;

int fprintf(FILE *stream, const char *format, ...);
int dprintf(int fd, const char *format, ...);
int sprintf(char *str, const char *format, ...);
int snprintf(char *str, size_t size, const char *format, ...);

#endif // _STDIO_H
