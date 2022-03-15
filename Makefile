include config.mk


all: target/release/projinfo

target/release/projinfo:
	cargo build --release

install: all
	mkdir -p $(DESTDIR)/bin/
	install target/release/projinfo $(DESTDIR)/bin/projinfo
	mkdir -p $(CONFIGDIR)/projinfo
	install src/res/langs.json $(CONFIGDIR)/projinfo/langs.json

uninstall:
	rm -f $(DESTDIR)/bin/projinfo
	rm -f $(CONFIGDIR)/projinfo/langs.json

clean:
	rm -rf target


options:
	@echo OS = $(UNAME)
	@echo install location = $(DESTDIR)/bin/
	@echo config location = $(CONFIGDIR)/projinfo/



.PHONY: all install uninstall clean
