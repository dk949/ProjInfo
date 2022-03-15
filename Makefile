include config.mk


all: target/release/projinfo

target/release/projinfo:
	cargo build --release

install:
	mkdir -p $(DESTDIR)/bin/
	install target/release/projinfo $(DESTDIR)/bin/projinfo
	mkdir -p $(CONFIGDIR)/projinfo
	install src/res/langs.json $(CONFIGDIR)/projinfo/langs.json

uninstall:
	rm -f $(DESTDIR)/bin/projinfo
	rm -f $(CONFIGDIR)/projinfo/langs.json

clean:
	rm -rf target

.PHONY: all install uninstall clean
