# yank Repository Inventory

This document records only the facts directly observed during the current repository scan. It does not fill in missing directory trees or guess at artifacts that have not appeared yet.

## Snapshot
- Project name: `yank`
- Build system: `Makefile`
- C file count: 1
- Header file count: 0
- Other file count: 25
- Build files: Makefile
- Entry files: none
- Executables observed at the repository root: yank
- Library files observed at the repository root: none

## Directory Inventory
- `root`: 17 files total (1 C files, 0 header files, 16 other files). Examples: .clang-format, .gitignore, .valgrindrc, CHANGELOG.md, LICENSE, Makefile
- `test`: 9 files total (0 C files, 0 header files, 9 other files). Examples: test/_lib.bash, test/yank-basic.sh, test/yank-ctrl-c.sh, test/yank-delim.sh, test/yank-help-rc.sh, test/yank-line-mode.sh

## Source File Inventory
- `yank.c`

## Header Files by Directory
- No header files were observed.

## README Excerpt
# yank

Yank terminal output to clipboard.

![yank](https://raw.githubusercontent.com/mptre/yank/gh-pages/screencast.gif)

## Description

The
[yank(1)][yank]
utility reads input from `stdin` and display a selection interface that allows a
field to be selected and copied to the clipboard.
Fields are either recognized by a regular expression using the `-g` option or by
splitting the input on a delimiter sequence using the `-d` option.

Using the arrow keys will move the selected field.
The interface supports several Emacs and Vi like key bindings,
consult the man page for further reference.
Pressing the return key will invoke the yank command and write the selected
field to its `stdin`.
The yank command defaults to
[xsel(1)][xsel]
but could be anything that accepts input on `stdin`.
When invoking yank,
everything supplied after the `--` option will be used as the yank command,
see examples below.

## Motivation

Others including myself consider it a cache miss when resort to using the mouse.
Copying output from the terminal is still one of the few cases where I still use
the mouse.
Several terminal multiplexers solves this issue,
however I don't want to be required to use a multiplexer but instead use a
terminal agnostic solution.

## Examples

- Yank an environment variable key or value:

  ```sh
  $ env | yank -d =
  ```

- Yank a field from a CSV file:

  ```sh
  $ yank -d \", <file.csv
  ```

- Yank a whole line using the `-l` option:

  ```sh
  $ make 2>&1 | yank -l
  ```

- If `stdout` is not a terminal the selected field will be written to `stdout`
  and exit without invoking the yank command.
  Kill the selected PID:

  ```sh
  $ ps ux | yank -g [0-9]+ | xargs kill
  ```

- Yank the selected field to the clipboard as opposed of the default primary
  clipboard:

  ```sh
  $ yank -- xsel -b
  ```

## Installation

### Arch Linux

```sh
$ pacman -S yank
```

### Debian

```sh
$ sudo apt-get install yank
```

The binary is installed at `/usr/bin/yank-cli` due to
...[truncated]

## Evidence Boundaries
- This inventory comes from a filesystem scan and does not represent the final installation layout.
- Executables, libraries, or generated directories that were not observed will not be inferred to exist.
- File responsibilities and module behavior must be understood together with the later `01_subsystems` / `03_behaviors` documents.
