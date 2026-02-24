REPO_ROOT = ../..
EXAMPLE_HTML = turboencabulator/target/doc/turboencabulator/index.html
TEST_SRC = $(REPO_ROOT)/tests/rdl_src

.PHONY: $(EXAMPLE_HTML)
$(EXAMPLE_HTML):
	mkdir -p $(REPO_ROOT)/docs/rust_docs/examples
	mkdir -p turboencabulator/tests
	cp $(TEST_SRC)/turboencabulator.rs turboencabulator/tests/
	sed -i 's/peakrdl_rust_test/turboencabulator/g' turboencabulator/tests/turboencabulator.rs
	cd turboencabulator && cargo test
	cd turboencabulator && cargo doc
	cp -r turboencabulator/target/doc/. $(REPO_ROOT)/docs/rust_docs/examples/

clean:
	rm -rf turboencabulator/tests
	rm -rf turboencabulator/target
