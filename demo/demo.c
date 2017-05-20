#define _POSIX_SOURCE
#include <sys/ptrace.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <sys/reg.h>
#include <sys/user.h>
#include <sys/wait.h>
#include <signal.h>
#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>

void do_child(int argc, char const ** argv){
	char * args [argc+1];
	memcpy(args, argv, argc * sizeof(char *));
	args[argc] = NULL;
	ptrace(PTRACE_TRACEME, NULL, NULL, NULL);
	kill(getpid(), SIGSTOP);
	execvp(args[0], args);
	fprintf(stderr, "Didn't run\n");
}


void do_trace(pid_t child){
	int status = 0;
	long syscall = -1;
	ptrace(PTRACE_SETOPTIONS, child, 0, PTRACE_O_TRACESYSGOOD);
	while (1) {
		waitpid(child, &status, 0);
		syscall = 
			ptrace( PTRACE_PEEKUSER, child, 
					sizeof(long) * ORIG_RAX);
		if ( syscall == -1 ) { break; }
		fprintf(stderr, "syscall %ld made.\n", syscall);
		if ( syscall == 59 ) {
			long rsi = ptrace(PTRACE_PEEKUSER, child, 
							sizeof(long) * RSI);
			long temp;
			int one_0 = 0, two_0 = 0;
			while (!(one_0 && two_0) && (rsi != 0)) {
				fprintf(stderr, "%ld\n", rsi);
				char c;
				temp = ptrace(PTRACE_PEEKTEXT, child, rsi, 0);
				for (int i=0; i<8; i++){
					c = temp & 0xf;
					temp = temp >> 8;
					printf("%c", c);
					if (c == 0) {
						if (one_0) { two_0 = 1; }
						one_0 = 1;
					} else {
						one_0 = two_0 = 0;
					}
				}
				rsi += 8;
			}
			fprintf(stderr, "\t%ld\n", rsi);
		}
		ptrace(PTRACE_SYSCALL, child, 0, 0);
	}
}

int main(int argc, char const *argv[]) {
	
	if (argc < 2) {
		fprintf(stderr, "Please specify program you want to trace.\n");
		exit(1);
	}

	pid_t child = fork();
	if (child == 0) {
		do_child(argc-1, argv+1);
		// We leave the frist argument behind.
		// So argc(count var) --; 
		// argv(arg lists) ++, points to next arg.
	} else {
		do_trace(child);
	}
	return 0;

}