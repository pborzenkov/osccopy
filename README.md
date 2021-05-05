# oscopy - copy text from remote machine into local clipboard

`osccopy` uses OSC 52 terminal escape sequence to copy data from stdin into the clipboard on the machine where the
terminal emulator is running.

This requires support for OSC 52 from the terminal emulator. Though, most of the modern ones support it.

## Usage

```bash
# On a remote machine via SSH
$ cat file.txt | osccopy
# file.txt contents is placed into the clipboard of the local machine
```

## License

MIT
