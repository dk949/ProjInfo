include config.mk


all: target/release/projinfo $(if $(NOMAN),,man)

target/release/projinfo:
	cargo build --release

install: all
	mkdir -p $(DESTDIR)/bin/
	install target/release/projinfo $(DESTDIR)/bin/projinfo
	mkdir -p $(MAN)/man1
	install man/dist/man1/projinfo.1 $(MAN)/man1/projinfo.1
	mkdir -p $(CONFIGDIR)/projinfo
	install src/res/langs.json $(CONFIGDIR)/projinfo/langs.json

uninstall:
	rm -f $(DESTDIR)/bin/projinfo
	rm -f $(MAN)/man1/projinfo.1

clean:
	rm -f target/release/projinfo
	rm -f man/dist/man1/projinfo.1

options:
	@echo OS = $(UNAME)
	@echo install location = $(DESTDIR)/bin/
	@echo mandir = $(MAN)/man1/
	@echo config location = $(CONFIGDIR)/projinfo/

man: man/dist/man1/projinfo.1


man/dist/man1/projinfo.1:
	mkdir -p man/dist/man1
	pandoc --standalone --to man "./man/man1/projinfo.1.md" -o ./man/dist/man1/projinfo.1


.PHONY: all install uninstall clean options man
