#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <syslog.h>

/* exit statuses */
#define E_SUCCESS  0  /* success */
#define E_USAGE	   1  /* invalid options */

void usage(char *prog, int status) {
    FILE *out = (status == E_SUCCESS) ? stdout : stderr;

	fprintf(out, "EUMaster4HPC\n");
	fprintf(out, "\n");
	fprintf(out, "  This program allows configuring power capping.\n");
	fprintf(out, "  Its main purpose is to allow non-sudoer users\n");
	fprintf(out, "  to set power capping with the use of setuid bit.\n");
	fprintf(out, "\n");
	fprintf(out, "Usage: %s <file> <value>\n", prog);
	fprintf(out, "\n");
	fprintf(out, "file may be one of:\n");
	fprintf(out, "  /sys/class/powercap/intel-rapl/intel-rapl:0/\n");
	fprintf(out, "\n");
	fprintf(out, "Author: Kamil Jarosz\n");
	fprintf(out, "Website: https://github.com/kjarosh/eumaster4hpc-tools\n");

	exit(status);
}

int main(int argc, char **argv) {
    syslog(LOG_NOTICE, "Test");

    char *prog = argc >= 1 ? argv[0] : "program";

    if (argc == 2 && (strcmp(argv[1], "--help") == 0 || strcmp(argv[1], "-h") == 0)) {
        usage(prog, E_SUCCESS);
    }

    if (argc != 3) {
        usage(prog, E_USAGE);
    }

    printf("%i\n", argc);
    usage(argv[0], E_SUCCESS);
}
