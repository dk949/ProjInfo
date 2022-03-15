
UNAME = $(shell uname -s)

ifeq ($(UNAME), Linux)
	DESTDIR=$(HOME)/.local
	CONFIGDIR=$(if $(XDG_CONFIG_HOME),$(XDG_CONFIG_HOME),$(HOME)/.config)
else ifeq ($(UNAME), Darwin)
	DESTDIR=$(HOME)/.cargo
	CONFIGDIR=$(HOME)/Library/Preferences
endif
