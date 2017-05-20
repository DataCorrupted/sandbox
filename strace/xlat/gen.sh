#!/bin/sh
#
# Copyright (c) 2014-2015 Mike Frysinger <vapier@gentoo.org>
# Copyright (c) 2014-2015 Dmitry V. Levin <ldv@altlinux.org>
# All rights reserved.
#
# Redistribution and use in source and binary forms, with or without
# modification, are permitted provided that the following conditions
# are met:
# 1. Redistributions of source code must retain the above copyright
#    notice, this list of conditions and the following disclaimer.
# 2. Redistributions in binary form must reproduce the above copyright
#    notice, this list of conditions and the following disclaimer in the
#    documentation and/or other materials provided with the distribution.
# 3. The name of the author may not be used to endorse or promote products
#    derived from this software without specific prior written permission.
#
# THIS SOFTWARE IS PROVIDED BY THE AUTHOR ``AS IS'' AND ANY EXPRESS OR
# IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES
# OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED.
# IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY DIRECT, INDIRECT,
# INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT
# NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
# DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
# THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
# (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
# THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

usage()
{
	cat <<EOF
Usage: $0 <input> <output>

Generate xlat header files from <input> (a file or dir of files) and write
the generated headers to <output>.
EOF
	exit 1
}

cond_def()
{
	local line
	line="$1"; shift

	local val
	val="$(printf %s "$line" |
		sed -r -n 's/^([^[:space:]]+).*$/\1/p')"

	local def
	def="$(printf %s "${line}" |
		sed -r -n 's/^[^[:space:]]+[[:space:]]+([^[:space:]].*)$/\1/p')"

	if [ -n "$def" ]; then
		cat <<-EOF
		#if !(defined($val) || (defined(HAVE_DECL_$val) && HAVE_DECL_$val))
		# define $val $def
		#endif
		EOF
	fi
}

print_xlat()
{
	local val
	val="$1"; shift

	if [ -z "${val_type-}" ]; then
		echo " XLAT(${val}),"
	else
		echo " XLAT_TYPE(${val_type}, ${val}),"
	fi
}

print_xlat_pair()
{
	local val str
	val="$1"; shift
	str="$1"; shift

	if [ -z "${val_type-}" ]; then
		echo " XLAT_PAIR(${val}, \"${str}\"),"
	else
		echo " XLAT_TYPE_PAIR(${val_type}, ${val}, \"${str}\"),"
	fi
}

cond_xlat()
{
	local line val m def xlat
	line="$1"; shift

	val="$(printf %s "${line}" | sed -r -n 's/^([^[:space:]]+).*$/\1/p')"
	m="${val%%|*}"
	def="$(printf %s "${line}" |
	       sed -r -n 's/^[^[:space:]]+[[:space:]]+([^[:space:]].*)$/\1/p')"

	if [ "${m}" = "${m#1<<}" ]; then
		xlat="$(print_xlat "${val}")"
	else
		xlat="$(print_xlat_pair "1ULL<<${val#1<<}" "${val}")"
		m="${m#1<<}"
	fi

	if [ -z "${def}" ]; then
		cat <<-EOF
		#if defined(${m}) || (defined(HAVE_DECL_${m}) && HAVE_DECL_${m})
		 ${xlat}
		#endif
		EOF
	else
		echo "$xlat"
	fi
}

gen_header()
{
	local input="$1" output="$2" name="$3"
	echo "generating ${output}"
	(
	local defs="${0%/*}/../defs.h"
	local mpers="${0%/*}/../mpers_xlat.h"
	local decl="extern const struct xlat ${name}[];"
	local in_defs= in_mpers=

	if grep -F -x "$decl" "$defs" > /dev/null; then
		in_defs=1
	elif grep -F -x "$decl" "$mpers" > /dev/null; then
		in_mpers=1
	fi

	echo "/* Generated by $0 from $1; do not edit. */"

	local unconditional= unterminated= line
	# 1st pass: output directives.
	while read line; do
		LC_COLLATE=C
		case $line in
		'#stop')
			exit 0
			;;
		'#conditional')
			unconditional=
			;;
		'#unconditional')
			unconditional=1
			;;
		'#unterminated')
			unterminated=1
			;;
		'#val_type '*)
			# to be processed during 2nd pass
			;;
		'#'*)
			echo "${line}"
			;;
		[A-Z_]*)
			[ -n "$unconditional" ] ||
				cond_def "$line"
			;;
		esac
	done < "$input"

	echo
	if [ -n "$in_defs" ]; then
		cat <<-EOF
			#ifndef IN_MPERS

		EOF
	elif [ -n "$in_mpers" ]; then
		cat <<-EOF
			#ifdef IN_MPERS

			${decl}

			#else

			# if !(defined HAVE_M32_MPERS || defined HAVE_MX32_MPERS)
			static
			# endif
		EOF
	else
		cat <<-EOF
			#ifdef IN_MPERS

			# error static const struct xlat ${name} in mpers mode

			#else

			static
		EOF
	fi
	echo "const struct xlat ${name}[] = {"

	unconditional= val_type=
	# 2nd pass: output everything.
	while read line; do
		LC_COLLATE=C
		case ${line} in
		'#conditional')
			unconditional=
			;;
		'#unconditional')
			unconditional=1
			;;
		'#unterminated')
			# processed during 1st pass
			;;
		'#val_type '*)
			val_type="${line#\#val_type }"
			;;
		[A-Z_]*)	# symbolic constants
			if [ -n "${unconditional}" ]; then
				print_xlat "${line}"
			else
				cond_xlat "${line}"
			fi
			;;
		'1<<'[A-Z_]*)	# symbolic constants with shift
			if [ -n "${unconditional}" ]; then
				print_xlat_pair "1ULL<<${line#1<<}" "${line}"
			else
				cond_xlat "${line}"
			fi
			;;
		[0-9]*)	# numeric constants
			print_xlat "${line}"
			;;
		*)	# verbatim lines
			echo "${line}"
			;;
		esac
	done < "${input}"
	if [ -n "${unterminated}" ]; then
		echo " /* this array should remain not NULL-terminated */"
	else
		echo " XLAT_END"
	fi

	cat <<-EOF
		};

		#endif /* !IN_MPERS */
	EOF
	) >"${output}"
}

gen_make()
{
	local output="$1"
	local name
	shift
	echo "generating ${output}"
	(
		printf "XLAT_INPUT_FILES = "
		printf 'xlat/%s.in ' "$@"
		echo
		printf "XLAT_HEADER_FILES = "
		printf 'xlat/%s.h ' "$@"
		echo
		for name; do
			printf '$(top_srcdir)/xlat/%s.h: $(top_srcdir)/xlat/%s.in $(top_srcdir)/xlat/gen.sh\n' \
				"${name}" "${name}"
			echo '	$(AM_V_GEN)$(top_srcdir)/xlat/gen.sh $< $@'
		done
	) >"${output}"
}

gen_git()
{
	local output="$1"
	shift
	echo "generating ${output}"
	(
		printf '/%s\n' .gitignore Makemodule.am
		printf '/%s.h\n' "$@"
	) >"${output}"
}

main()
{
	case $# in
	0) set -- "${0%/*}" "${0%/*}" ;;
	2) ;;
	*) usage ;;
	esac

	local input="$1"
	local output="$2"
	local name
	local jobs=0
	local ncpus="$(getconf _NPROCESSORS_ONLN)"
	[ "${ncpus}" -ge 1 ] ||
		ncpus=1

	if [ -d "${input}" ]; then
		local f names=
		for f in "${input}"/*.in; do
			[ -f "${f}" ] || continue
			name=${f##*/}
			name=${name%.in}
			gen_header "${f}" "${output}/${name}.h" "${name}" &
			names="${names} ${name}"
			: $(( jobs += 1 ))
			if [ ${jobs} -ge ${ncpus} ]; then
				jobs=0
				wait
			fi
		done
		gen_git "${output}/.gitignore" ${names}
		gen_make "${output}/Makemodule.am" ${names}
		wait
	else
		name=${input##*/}
		name=${name%.in}
		gen_header "${input}" "${output}" "${name}"
	fi
}

main "$@"
