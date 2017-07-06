all:
	gcc -Wpedantic -Wall -Wextra -Werror -std=c89 factorize.c -pthread -o factorize