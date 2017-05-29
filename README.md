	strace is implemented in c and used in linux. It's  a little bit nasty since used goto.
	toy-trace is our version of strace in c. we did it to have a better sence of how ptrace work.
	rust-ptrace is a ptrace implemented in rust. quite naive and brute-force.
	tracer is the lib for tracing process building on libc::ptrace
	safe-box is the whole point. we focus on I/O safety, especially open() and connect() syscall.
	demo is full with c files used for demo.
