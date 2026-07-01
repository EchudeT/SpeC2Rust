# sds Repository Inventory

This document records only the facts directly observed during the current repository scan. It does not fill in missing directory trees or guess at artifacts that have not appeared yet.

## Snapshot
- Project name: `sds`
- Build system: `Makefile`
- C file count: 1
- Header file count: 3
- Other file count: 37
- Build files: Makefile
- Entry files: none
- Executables observed at the repository root: sds
- Library files observed at the repository root: none

## Directory Inventory
- `root`: 16 files total (1 C files, 3 header files, 12 other files). Examples: .gitignore, Changelog, LICENSE, Makefile, README.md, base_test.sh
- `test`: 25 files total (0 C files, 0 header files, 25 other files). Examples: test/_lib.bash, test/sds-01-create-and-length.sh, test/sds-02-create-with-specified-length.sh, test/sds-03-string-concatenation.sh, test/sds-04-sdscpy-against-longer.sh, test/sds-05-sdscpy-against-shorter.sh

## Source File Inventory
- `sds.c`

## Header Files by Directory
### `root`
- `sds.h`
- `sdsalloc.h`
- `testhelp.h`

## README Excerpt
Simple Dynamic Strings
===

**Notes about version 2**: this is an updated version of SDS in an attempt
to finally unify Redis, Disque, Hiredis, and the stand alone SDS versions.
This version is **NOT* binary compatible** with SDS verison 1, but the API
is 99% compatible so switching to the new lib should be trivial.

Note that this version of SDS may be a slower with certain workloads, but
uses less memory compared to V1 since header size is dynamic and depends to
the string to alloc.

Moreover it includes a few more API functions, notably `sdscatfmt` which
is a faster version of `sdscatprintf` that can be used for the simpler
cases in order to avoid the libc `printf` family functions performance
penalty.

How SDS strings work
===

SDS is a string library for C designed to augment the limited libc string
handling functionalities by adding heap allocated strings that are:

* Simpler to use.
* Binary safe.
* Computationally more efficient.
* But yet... Compatible with normal C string functions.

This is achieved using an alternative design in which instead of using a C
structure to represent a string, we use a binary prefix that is stored
before the actual pointer to the string that is returned by SDS to the user.

    +--------+-------------------------------+-----------+
    | Header | Binary safe C alike string... | Null term |
    +--------+-------------------------------+-----------+
             |
             `-> Pointer returned to the user.

Because of meta data stored before the actual returned pointer as a prefix,
and because of every SDS string implicitly adding a null term at the end of
the string regardless of the actual content of the string, SDS strings work
well together with C strings and the user is free to use them interchangeably
with other std C string functions that access the string in read-only.

SDS was a C string I developed in the past for my everyday C programming needs,
later it was moved into Redis where it is used extensively and where
...[truncated]

## Evidence Boundaries
- This inventory comes from a filesystem scan and does not represent the final installation layout.
- Executables, libraries, or generated directories that were not observed will not be inferred to exist.
- File responsibilities and module behavior must be understood together with the later `01_subsystems` / `03_behaviors` documents.
