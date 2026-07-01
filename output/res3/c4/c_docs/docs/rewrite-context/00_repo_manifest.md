# c4 Repository Inventory

This document records only the facts directly observed during the current repository scan. It does not fill in missing directory trees or guess at artifacts that have not appeared yet.

## Snapshot
- Project name: `c4`
- Build system: `Makefile`
- C file count: 4
- Header file count: 0
- Other file count: 11
- Build files: Makefile
- Entry files: none
- Executables observed at the repository root: c4
- Library files observed at the repository root: none

## Directory Inventory
- `root`: 11 files total (2 C files, 0 header files, 9 other files). Examples: LICENSE, Makefile, README.md, base_test.sh, c4, c4.c
- `test`: 4 files total (2 C files, 0 header files, 2 other files). Examples: test/c4-hello.sh, test/c4-self-host.sh, test/c4.c, test/hello.c

## Source File Inventory
- `c4.c`
- `hello.c`
- `test/c4.c`
- `test/hello.c`

## Header Files by Directory
- No header files were observed.

## README Excerpt
c4 - C in four functions
========================

An exercise in minimalism.

Try the following:

    gcc -o c4 c4.c
    ./c4 hello.c
    ./c4 -s hello.c
    
    ./c4 c4.c hello.c
    ./c4 c4.c c4.c hello.c



## Evidence Boundaries
- This inventory comes from a filesystem scan and does not represent the final installation layout.
- Executables, libraries, or generated directories that were not observed will not be inferred to exist.
- File responsibilities and module behavior must be understood together with the later `01_subsystems` / `03_behaviors` documents.
