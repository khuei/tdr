.POSIX:

VERSION=1.2.2
PREFIX = /usr/local
MANPREFIX = $(PREFIX)/share/man

all: todo-rs

todo-rs:
	cargo build --release -vv
	cp target/release/todo-rs ./

clean:
	cargo clean -vv
	rm todo-rs

install: todo-rs
	mkdir -p $(DESTDIR)$(PREFIX)/bin
	cp -f target/release/todo-rs $(DESTDIR)$(PREFIX)/bin
	chmod 755 $(DESTDIR)$(PREFIX)/bin/todo-rs
	mkdir -p $(DESTDIR)$(MANPREFIX)/man1
	cp -f todo-rs.1 $(DESTDIR)$(MANPREFIX)/man1/todo-rs.1
	chmod 644 $(DESTDIR)$(MANPREFIX)/man1/todo-rs.1

uninstall:
	rm -r $(DESTDIR)$(PREFIX)/bin/todo-rs
	rm -r $(DESTDIR)$(MANPREFIX)/man1/todo-rs.1

.PHONY: all clean install uninstall
