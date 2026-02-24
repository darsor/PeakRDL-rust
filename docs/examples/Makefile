REPO_ROOT = ../..
EXAMPLE_HTML = turboencabulator/target/doc/turboencabulator/index.html
TEST_SRC = $(REPO_ROOT)/tests/rdl_src

$(EXAMPLE_HTML): $(REPO_ROOT)/tests/rdl_src/turboencabulator.*
	mkdir -p $(REPO_ROOT)/docs/rust_docs/examples
	uv run peakrdl rust --force --fmt -o turboencabulator/src/generated $(REPO_ROOT)/src/peakrdl_rust/udps/udps.rdl $(TEST_SRC)/turboencabulator.rdl
	mkdir -p turboencabulator/tests
	cp $(TEST_SRC)/turboencabulator.rs turboencabulator/tests/
	sed -i 's/peakrdl_rust_test/turboencabulator/g' turboencabulator/tests/turboencabulator.rs
	cd turboencabulator && cargo test
	cd turboencabulator && cargo doc
	cp -r turboencabulator/target/doc/. $(REPO_ROOT)/docs/rust_docs/examples/

clean:
	rm -rf turboencabulator/tests
	rm -rf turboencabulator/target
	rm -rf turboencabulator/src/generated
