	demo is a simple implementation of strace in c. It didn't work as expected. 
	rust-ptrace is a ptrace implemented in rust. quite naive and brute-force.
	strace is implemented in c and used in linux. It's  a little bit nasty since used goto.
    tracer is the lib for tracing process building on libc::ptrace
    safe-box is the sandbox demo