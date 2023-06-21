#include <stdlib.h>
#include <stdio.h>

/* exit statuses */
#define E_SUCCESS  0  /* success */
#define E_USAGE	   1  /* invalid options */

void usage(char *prog, int status) {
    FILE *out = (status == E_SUCCESS) ? stdout : stderr;

	fprintf(out, "Usage: %s [options]\n\nOptions:\n", prog);
	fprintf(out, "  -a, --all                     TBD\n");

	exit(status);
}

int main(int argc, char **argv) {
    printf("%i\n", argc);
    usage(argv[0], E_SUCCESS);
}
