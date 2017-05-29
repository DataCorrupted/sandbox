#include <stdio.h>
#include <fcntl.h>

int main(int argc, char const *argv[]) {
	int fd1 = open("/home/peter/Desktop/OS/sandbox/demo/foo.txt", O_RDONLY, 0600);
	int fd2 = open("/home/peter/Desktop/foo.txt", O_RDONLY, 0600);
	int fd3 = open("/usr/include/stdio.h", O_RDONLY, 0600);
	return 0;
}