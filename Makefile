RUSTC = /usr/bin/rustc -C opt-level=3
BIN = ~/bin/  # Configure to your desired location, make sure it is on $PATH

.DEFAULT: all
.PHONY: all

all: timer

timer:
	$(RUSTC) $(@).rs -o $(BIN)$(@)
