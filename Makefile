PKG_NAME=simplemap
DOCS_DEFAULT_MODULE=simplemap
DEFAULT_TARGET=build

default: $(DEFAULT_TARGET)

.PHONY: run test build doc clean release rrun bench
run test build doc clean:
	cargo $@

simple:
	cargo run

release:
	cargo build --release

rrun:
	cargo run --release

bench:
	cargo bench

publishdoc: doc
	echo '<meta http-equiv="refresh" content="0;url='${DOCS_DEFAULT_MODULE}'/index.html">' > target/doc/index.html
	ghp-import -n target/doc
	git push origin gh-pages


.PHONY: docview
docview: doc
	xdg-open target/doc/$(PKG_NAME)/index.html
