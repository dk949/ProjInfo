
UNAME = $(shell uname -s)

DESTDIR=/usr/local
MAN=$(DESTDIR)/share/man

ifeq ($(UNAME), Linux)
	CONFIGDIR=$(if $(XDG_CONFIG_HOME),$(XDG_CONFIG_HOME),$(HOME)/.config)
else ifeq ($(UNAME), Darwin)
	CONFIGDIR=$(HOME)/Library/Preferences
endif
