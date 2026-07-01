# shc Repository Inventory

This document records only the facts directly observed during the current repository scan. It does not fill in missing directory trees or guess at artifacts that have not appeared yet.

## Snapshot
- Project name: `shc`
- Build system: `Makefile.in`
- C file count: 1
- Header file count: 0
- Other file count: 55
- Build files: Makefile, configure, Makefile.am, Makefile.in
- Entry files: none
- Executables observed at the repository root: configure, config.status, shc
- Library files observed at the repository root: none

## Directory Inventory
- `autom4te.cache`: 5 files total (0 C files, 0 header files, 5 other files). Examples: autom4te.cache/output.0, autom4te.cache/output.1, autom4te.cache/requests, autom4te.cache/traces.0, autom4te.cache/traces.1
- `config`: 4 files total (0 C files, 0 header files, 4 other files). Examples: config/compile, config/depcomp, config/install-sh, config/missing
- `root`: 23 files total (0 C files, 0 header files, 23 other files). Examples: .gitattributes, .gitignore, .travis.yml, AUTHORS, COPYING, ChangeLog
- `src`: 6 files total (1 C files, 0 header files, 5 other files). Examples: src/Makefile, src/Makefile.am, src/Makefile.in, src/shc, src/shc.c, src/shc.o
- `test`: 18 files total (0 C files, 0 header files, 18 other files). Examples: test/match, test/pru.sh, test/pru.sh.kk, test/test.bash, test/test.csh, test/test.ksh

## Source File Inventory
- `src/shc.c`

## Header Files by Directory
- No header files were observed.

## README Excerpt
[![build status image](https://travis-ci.org/neurobin/shc.svg?branch=release)](https://travis-ci.org/neurobin/shc)
[![GitHub stars](https://img.shields.io/github/stars/neurobin/shc.svg)](https://github.com/neurobin/shc/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/neurobin/shc.svg)](https://github.com/neurobin/shc/network)
[![GitHub issues](https://img.shields.io/github/issues/neurobin/shc.svg)](https://github.com/neurobin/shc/issues)

# Shell Script Compiler

A generic shell script compiler. Shc takes a script, which is specified on the command line and produces C source code. The generated source code is then compiled and linked to produce a stripped binary executable.

The compiled binary will still be dependent on the shell specified in the first line of the shell code (i.e shebang) (i.e. #!/bin/sh), thus shc does not create completely independent binaries.

shc itself is not a compiler such as cc, it rather encodes and encrypts a shell script and generates C source code with the added expiration capability. It then uses the system compiler to compile a stripped binary which behaves exactly like the original script. Upon execution, the compiled binary will decrypt and execute the code with the shell -c option.

## Install

```bash
./configure
make
sudo make install
```

**Note** If `make` fails due to *automake* version, run `./autogen.sh` before running the above commands.

### Ubuntu-specific

```
sudo add-apt-repository ppa:neurobin/ppa
sudo apt-get update
sudo apt-get install shc
```

If the above installation method seems like too much work, then just download a compiled binary package from [release page](https://github.com/neurobin/shc/releases/latest) and copy the `shc` binary to `/usr/bin` and `shc.1` file to `/usr/share/man/man1`.

## Usage

```
shc [options]
shc -f script.sh -o binary
shc -U -f script.sh -o binary # Untraceable binary (prevent strace, ptrace etc..)
shc -H -f script.sh -o binary # Untraceable binary, does not require
...[truncated]

## Evidence Boundaries
- This inventory comes from a filesystem scan and does not represent the final installation layout.
- Executables, libraries, or generated directories that were not observed will not be inferred to exist.
- File responsibilities and module behavior must be understood together with the later `01_subsystems` / `03_behaviors` documents.
