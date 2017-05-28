#[derive(Debug)]
pub enum Register{
	R15 = 0,
	R14 = 1,
	R13 = 2,
	R12 = 3,
	RBP = 4,
	RBX = 5,
	R11 = 6,
	R10 = 7,
	R9 = 8,
	R8 = 9,
	RAX = 10,
	RCX = 11,
	RDX = 12,
	RSI = 13,
	RDI = 14,
	ORIGRAX = 15,
	RIP = 16,
	CS = 17,
	EFLAGS = 18,
	RSP = 19,
	SS = 20,
	FSBASE = 21,
	GSBASE = 22,
	DS = 23,
	ES = 24,
	FS = 25,
	GS = 26,
}
#[derive(Debug)]
pub enum Request{
	TRACEME = 0,
	PEEKTEXT= 1,
	PEEKDATA= 2,
	PEEKUSER= 3,
	POKETEXT= 4,
	POKEDATA= 5,
	POKEUSER= 6,
	CONT= 7,
	KILL= 8,
	SINGLESTEP= 9,
	GETREGS= 12,
	SETREGS= 13,
	ATTACH= 16,
	DETACH= 17,
	SYSCALL= 24,
	SETOPTIONS= 0x4200,
	GETEVENTMSG= 0x4201,
	GETSIGINFO= 0x4202,
	SETSIGINFO= 0x4203,
	GETREGSET= 0x4204,
	SETREGSET= 0x4205,
	SEIZE= 0x4206,
	INTERRUPT= 0x4207,
	LISTEN= 0x4208,
	PEEKSIGINFO= 0x4209,
}