
NAME=cairn
PREFIX=/usr/local

all:
	cargo build --release

install:
	mkdir -p ${DESTDIR}${PREFIX}/bin
	cp -f target/release/${NAME} ${DESTDIR}${PREFIX}/bin
	chmod +x ${DESTDIR}${PREFIX}/bin/${NAME}

.PHONY: all install


