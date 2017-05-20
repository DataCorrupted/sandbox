/*
 * Check decoding of sockaddr related arguments of recvfrom syscall.
 *
 * Copyright (c) 2016 Dmitry V. Levin <ldv@altlinux.org>
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. The name of the author may not be used to endorse or promote products
 *    derived from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
 * IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
 * OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
 * IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
 * NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#define TEST_SYSCALL_NAME recvfrom
#define TEST_SYSCALL_PREPARE send_un()
#define PREFIX_S_ARGS	, recv_buf, 1, 0
#define PREFIX_S_STR	", \"A\", 1, 0"
#define PREFIX_F_ARGS	, 0, 1, 0
#define PREFIX_F_STR	", NULL, 1, 0"
static void send_un(void);
static char recv_buf[1];
#include "sockname.c"

static int cfd;

static void
send_un(void)
{
	if (send(cfd, "A", 1, 0) != 1)
		perror_msg_and_skip("send");
}

int
main(void)
{
	cfd = socket(AF_UNIX, SOCK_STREAM, 0);
	int lfd = socket(AF_UNIX, SOCK_STREAM, 0);
	if (cfd < 0 || lfd < 0)
		perror_msg_and_skip("socket");

	struct sockaddr_un un = {
		.sun_family = AF_UNIX,
		.sun_path = TEST_SOCKET ".send"
	};

	(void) unlink(un.sun_path);
	if (bind(cfd, (const void *) &un, sizeof(un)))
		perror_msg_and_skip("bind");
	(void) unlink(un.sun_path);

	un.sun_path[sizeof(TEST_SOCKET) - 1] = '\0';
	(void) unlink(un.sun_path);

	if (bind(lfd, (const void *) &un, sizeof(un)))
		perror_msg_and_skip("bind");

	if (listen(lfd, 1))
		perror_msg_and_skip("listen");

	if (connect(cfd, (const void *) &un, sizeof(un)))
		perror_msg_and_skip("connect");

	int afd = accept(lfd, 0, 0);
	if (afd < 0)
		perror_msg_and_skip("accept");

	(void) unlink(un.sun_path);

	test_sockname_syscall(afd);

	puts("+++ exited with 0 +++");
	return 0;
}
