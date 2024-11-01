[![Build](https://github.com/dchansen06/timer-rs/actions/workflows/build.yml/badge.svg)](https://github.com/dchansen06/timer-rs/actions/workflows/build.yml)
# timer-rs
A short and simple timer written in rust, with the ability to modify the program by a set offset if you so choose

Adjust the location in the `Makefile` for the compiled program to end up, make sure it is on `$PATH`

Adjust the program's `OFFSET` should you want to always have your timer come back short, or longer, than instructed

## Example usage
```$ timer 5 ms```

```$ short-task && time 15 s && another-task```

## Disclaimer
This program makes no attempt to be perfect, and aims to return just after the time specified. Do not use where (extreme) accuracy is required

## Installation
Install the program with ```cargo install --git https://github.com/dchansen06/timer-rs/``` and then run the binary (generally in `~/.cargo/bin`) with your desired flags
