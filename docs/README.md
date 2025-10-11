# PeakRDL-Rust Documentation

This directory contains the Sphinx documentation for PeakRDL-rust.

The documentation is automatically built and hosted on [Read the Docs](https://peakrdl-rust.readthedocs.io/) whenever changes are pushed to the main branch.

## Building the Documentation

### Prerequisites

Install the required dependencies:

```bash
uv sync
```

Or install the documentation dependencies directly:

```bash
uv pip install sphinx sphinx-book-theme
```

### Building HTML Documentation

To build the HTML documentation:

```bash
make html
```

The generated documentation will be available in `_build/html/index.html`.

### Other Output Formats

Sphinx supports multiple output formats:

```bash
make pdf        # Generate PDF (requires LaTeX)
make epub       # Generate EPUB
make man        # Generate man pages
make text       # Generate plain text
```

### Development

For development, you can use the following commands:

```bash
make clean      # Clean build artifacts
make html       # Rebuild documentation
```

To automatically rebuild when files change, you can use `sphinx-autobuild`:

```bash
uv pip install sphinx-autobuild
sphinx-autobuild . _build/html
```

This will start a local server at http://127.0.0.1:8000 and automatically rebuild the documentation when files are modified.

## Read the Docs Integration

The documentation is automatically built and deployed using [Read the Docs](https://readthedocs.org/).

### Configuration

Read the Docs configuration is managed through `.readthedocs.yaml` in the project root. This file specifies:

- Python version and build environment
- Sphinx configuration location
- Dependencies and build requirements
- Output formats (HTML, PDF, ePub)

### Automatic Building

Documentation is automatically rebuilt when:

- Changes are pushed to the main branch
- Pull requests are opened (for preview builds)
- Manual builds are triggered from the Read the Docs dashboard

### Preview Builds

Read the Docs automatically creates preview builds for pull requests, making it easy to review documentation changes before merging.

## Contributing

When adding new documentation:

1. Create new `.rst` files for major sections
2. Add them to the `toctree` in `index.rst`
3. Use proper reStructuredText formatting
4. Test that the documentation builds without errors locally
5. Check that cross-references work correctly
6. Preview changes using Read the Docs preview builds on pull requests

### Testing Documentation Changes

Before submitting documentation changes:

```bash
# Build locally to check for errors
make html

# Check for broken links
make linkcheck

# Use the build script for additional checks
python build.py build --open
```

The GitHub Actions workflow also validates that documentation builds successfully on every pull request.
