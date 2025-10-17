REPO_ROOT = ../..
EXAMPLE_HTML = build/turboencabulator/target/doc/turboencabulator/index.html

$(EXAMPLE_HTML): $(REPO_ROOT)/tests/rdl_src/turboencabulator.*
	mkdir -p $(REPO_ROOT)/docs/rust_docs/examples
	mkdir -p build
	cp $(REPO_ROOT)/tests/rdl_src/turboencabulator.* ./
	uv run peakrdl rust --force -o build --crate-name turboencabulator $(REPO_ROOT)/src/peakrdl_rust/udps/udps.rdl turboencabulator.rdl
	cp turboencabulator.rs build/turboencabulator/tests/
	cd build/turboencabulator && cargo test
	cd build/turboencabulator && cargo doc --no-deps
	cp -r build/turboencabulator/target/doc/. $(REPO_ROOT)/docs/rust_docs/examples/
