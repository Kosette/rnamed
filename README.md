# rnamed

Rename files to their checksum.

# usage

```
A simple tool to rename files to their checksum

Usage: rnamed.exe [OPTIONS] <PATH>

Arguments:
  <PATH>
          Searching path or a single file path

Options:
  -a, --algo <ALGO>
          Set algorithm, default to `md5`. use `--help` to See more.

          "md5" => "Md5" | "b3" or "blake3" => "Blake3" | "sha256" => "Sha256" | "sha512" => "Sha512"

  -p, --pattern <PATTERN>
          Enable globbing and provide patterns, can apply multi times, `-p "*.txt" -p "*.md"`

  -r, --recursive
          Turn on recursively searching

  -s, --silent
          Turn on silent mode, suppressing existing files printing

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
