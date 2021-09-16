.POSIX:

VERSION=1.3.3
PREFIX = /usr/local
MANPREFIX = $(PREFIX)/share/man

all: tdr

tdr:
	cargo build --release -vv
	cp target/release/tdr ./

clean:
	cargo clean -vv
	rm tdr

install: tdr
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f target/release/tdr $(DESTDIR)$(PREFIX)/bin
	chmod 755 $(DESTDIR)$(PREFIX)/bin/tdr
	mkdir -p $(DESTDIR)$(MANPREFIX)/man1
	cp -f tdr.1 $(DESTDIR)$(MANPREFIX)/man1/tdr.1
	chmod 644 $(DESTDIR)$(MANPREFIX)/man1/tdr.1

uninstall:
	rm -r $(DESTDIR)$(PREFIX)/bin/tdr
	rm -r $(DESTDIR)$(MANPREFIX)/man1/tdr.1

.PHONY: all clean install uninstall
