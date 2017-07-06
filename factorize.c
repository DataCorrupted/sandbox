#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <fcntl.h>
#include <unistd.h>

#define ITERCNT 10000
/* Use u64 to state unsigned long */
/* It's rust style and more intuitive. */
#define u64 unsigned long
/* I preloaded 255 primes from 2 to 1619 */
#define PRIMECNT 255
int prime_table[] = {2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499,503,509,521,523,541,547,557,563,569,571,577,587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,673,677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,787,797,809,811,821,823,827,829,839,853,857,859,863,877,881,883,887,907,911,919,929,937,941,947,953,967,971,977,983,991,997,1009,1013,1019,1021,1031,1033,1039,1049,1051,1061,1063,1069,1087,1091,1093,1097,1103,1109,1117,1123,1129,1151,1153,1163,1171,1181,1187,1193,1201,1213,1217,1223,1229,1231,1237,1249,1259,1277,1279,1283,1289,1291,1297,1301,1303,1307,1319,1321,1327,1361,1367,1373,1381,1399,1409,1423,1427,1429,1433,1439,1447,1451,1453,1459,1471,1481,1483,1487,1489,1493,1499,1511,1523,1531,1543,1549,1553,1559,1567,1571,1579,1583,1597,1601,1607,1609,1613};

void err_report(int type, int arg_cnt){
	if (type == 3) {
		fprintf(stderr, "MemoryError: Realloc failed.\n");
	} else if (type == 2) {
		fprintf(stderr, "ThreadError: Failed to spawn a thread\n");
	} else if (type == 1){
		fprintf(stderr, "ArgError: Argument %d shouldn't be 1.\n", arg_cnt);
	} else if (type == 0){
		fprintf(stderr, "ArgsNumError: 2 args expected, %d given.\n", arg_cnt);
	} else if (type == -1) {
		fprintf(stderr, 
			"ArgError: Argument %d couldn't be converted to an integer.\n", arg_cnt);
	} else if (type == -2) {
		fprintf(stderr, 
			"ArgError: Argument %d should be positive.\n", arg_cnt);
	} else if (type == -3) {
		fprintf(stderr, 
			"ArgError: Argument %d too large.\n", arg_cnt);
	}
	exit(13);
}

typedef struct{
	int prime;
	int cnt;
} Ctnt;

/* cmp defined to do comparasion */
int cmp(const void* a, const void* b){ 
	Ctnt * A = (Ctnt*) a;
	Ctnt * B = (Ctnt*) b;
	return A->prime - B->prime;
}

u64 n, t;
u64 len = 0;
Ctnt vec[100];
pthread_mutex_t lock;

/* check_prime*/
/* check if a number is a prime  */
int check_prime(int num){
	int i=0;
	/* Use prime_table to do the first check. */
	for (i=0; i<PRIMECNT; i++){
		if (num % prime_table[i] == 0){
			return (num == prime_table[i]);
		}
	}
	/* If unable to determine if this is a prime */
	/* Use each and every possible number to check */
	for (i= 1619; i*i<num+1; i++){
		if (num % i ==0){ return 0;	}
	}
	return 1;
}

/* fact_num*/
void* fact_num(void* t){
	int num = *(int*) t;
	int iter = (num == 2) ? 1 : ITERCNT;
	int i=0;
	Ctnt new_ctnt;
	for (i = 0; i<iter; i++, num+=2){
		/* Determine if num is a prime */
		/* But doing so is expensive, we check if this number is a factor of n first. */
		/* If it's not a prime, then don't waste time doing things Below.*/
		if ((n % num !=0) || (check_prime(num) == 0)) { 
			continue; 
		}
	
		new_ctnt.prime = num;
		new_ctnt.cnt = 0;
		/* Exhaust this number with its copy.*/
		pthread_mutex_lock(&lock);
		while (n % num == 0) {
			n /= num;
			new_ctnt.cnt ++;
		}
		vec[len] = new_ctnt;
		len ++;
		pthread_mutex_unlock(&lock);
	}
	return NULL;
}

/* Convert a string to a number. */
/* I didn't use stroul so I can do error handling more conveniently. */
u64 str2num(const char* argv, u64 digits, int arg_cnt){
	u64 ans = 0, cnt = 0;
	/* The number is a negative number. */
	if (*argv == '-') { err_report(-2, arg_cnt); }
	while ((*argv !='\0') && (*argv <= '9') && (*argv>='0')) {
		ans = ans * 10 + (*argv - '0');
		argv ++; cnt ++;
	}
	/* A mixture of number and chars*/
	if (*argv != '\0') { err_report(-1, arg_cnt); }
	/* The number is a zero. */
	if (!ans) { err_report(-2, arg_cnt); }
	/* The number is too large. */
	if (cnt > digits) { err_report(-3, arg_cnt); }
	return ans;
}

int main(int argc, char const *argv[]){

	void* ptr;
	int arg[100];
	u64 i=2, j;
	pthread_t thread_id[100];
	int a = open("/home/peter/Desktop/install.sh", O_RDWR);
	printf("%d\n", a);
	close(a);
	if (argc != 3) { err_report(0, argc-1); }
	t = str2num(argv[1], 2, 0);
	n = str2num(argv[2], 19, 1);
	if (n == 1) { err_report(1, 1); }

	if (pthread_mutex_init(&lock, NULL) != 0) { err_report(2, 0); }

	/* Try 2 first. So that we can only work on odd number. */
	fact_num(&i); i++;

	while (n != 1){
		/* Spwan t thread to do the check */
		for (j=0; j<t; j++){
			arg[j] = i;
			pthread_create(&(thread_id[j]), NULL, &fact_num, (void*) &arg[j]);
			/* i can add 2 * ITERCNT every time to skip even number. */
			i += 2 * ITERCNT;
		}
		/* And wait for them to return to start next round */
		for (j=0; j<t; j++){
			pthread_join(thread_id[j], &ptr);
		}
		if ((unsigned long) i*i > n) break;
	}
	/* Sort the prime since threads */
	/* run in different order.*/
	qsort(vec+0, len, sizeof(Ctnt), cmp);
	/* Output. */
	for (i=0; i<len; i++){
		printf("%d^%d", vec[i].prime, vec[i].cnt);
		/* If the element printed just now */
		/* is not the last one*/
		/* Then we add a space after it.*/
		if (i!=len-1) { printf(" ");}
	}
	if (n != 1) {
	/* If n isn't updated to 1, then */
	/* the last prime is itself.*/
		if (len > 0){ printf(" ");}
		printf("%lu^1", n);
	}
	printf("\n");
	return 0;
}