#!/usr/bin/env python3
"""
Build script for PeakRDL-rust documentation.

This script provides convenient commands for building and managing
the Sphinx documentation. Documentation is automatically built and
hosted on Read the Docs at https://peakrdl-rust.readthedocs.io/
"""

import argparse
import os
import shutil
import subprocess
import webbrowser
from pathlib import Path


def run_command(cmd, cwd=None):
    """Run a shell command and return the result."""
    try:
        result = subprocess.run(
            cmd, shell=True, cwd=cwd, check=True, capture_output=False, text=True
        )
        return result.returncode == 0, result.stdout, result.stderr
    except subprocess.CalledProcessError as e:
        return False, e.stdout, e.stderr


def check_dependencies():
    """Check if required dependencies are installed."""
    try:
        import sphinx
        import sphinx_book_theme

        return True
    except ImportError as e:
        print(f"Missing dependency: {e}")
        print("Install dependencies with: uv sync --group docs")
        return False


def clean_build():
    """Clean build artifacts."""
    build_dir = Path("_build")
    if build_dir.exists():
        print("Cleaning build directory...")
        shutil.rmtree(build_dir)
        print("Build directory cleaned.")
    else:
        print("Build directory doesn't exist, nothing to clean.")


def build_docs(format_type="html", verbose=False):
    """Build documentation in specified format."""
    if not check_dependencies():
        return False

    print(f"Building {format_type} documentation...")

    cmd = f"sphinx-build -b {format_type}"
    if verbose:
        cmd += " -v"
    else:
        cmd += " -q"

    cmd += " . _build/" + format_type

    success, stdout, stderr = run_command(cmd)

    if success:
        print(f"Documentation built successfully in _build/{format_type}/")
        return True
    else:
        print("Build failed!")
        if stdout:
            print("STDOUT:", stdout)
        if stderr:
            print("STDERR:", stderr)
        return False


def serve_docs(port=8000, auto_build=False):
    """Serve documentation with local HTTP server."""
    if auto_build:
        try:
            import sphinx_autobuild
        except ImportError:
            print("sphinx-autobuild not installed. Install with: uv sync --group docs")
            return False

        print(f"Starting auto-build server on port {port}...")
        cmd = f"sphinx-autobuild . _build/html --port {port}"
        subprocess.run(cmd, shell=True)
    else:
        html_dir = Path("_build/html")
        if not html_dir.exists():
            print("HTML documentation not found. Building first...")
            if not build_docs("html"):
                return False

        print(f"Starting server on http://localhost:{port}")
        print("Press Ctrl+C to stop the server")

        import http.server
        import socketserver

        os.chdir(html_dir)
        handler = http.server.SimpleHTTPRequestHandler

        try:
            with socketserver.TCPServer(("", port), handler) as httpd:
                print("Local documentation preview available.")
                print("Published documentation: https://peakrdl-rust.readthedocs.io/")
                webbrowser.open(f"http://localhost:{port}")
                httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped.")


def check_links():
    """Check for broken links in documentation."""
    print("Checking for broken links...")
    success, stdout, stderr = run_command(
        "sphinx-build -b linkcheck . _build/linkcheck"
    )

    if success:
        print("Link check completed successfully.")
        # Print any warnings about broken links
        linkcheck_output = Path("_build/linkcheck/output.txt")
        if linkcheck_output.exists():
            content = linkcheck_output.read_text()
            if "broken" in content.lower():
                print("Broken links found:")
                print(content)
    else:
        print("Link check failed!")
        if stderr:
            print("STDERR:", stderr)


def spell_check():
    """Run spell check on documentation."""
    try:
        import sphinxcontrib.spelling
    except ImportError:
        print("Spell checking requires sphinxcontrib.spelling")
        print("Install with: uv sync --group docs")
        return False

    print("Running spell check...")
    success, stdout, stderr = run_command("sphinx-build -b spelling . _build/spelling")

    if success:
        print("Spell check completed.")
        spelling_dir = Path("_build/spelling")
        if spelling_dir.exists():
            for spelling_file in spelling_dir.glob("*.spelling"):
                if spelling_file.stat().st_size > 0:
                    print(f"Spelling errors in {spelling_file.name}:")
                    print(spelling_file.read_text())
    else:
        print("Spell check failed!")


def main():
    """Main entry point."""
    # Change to docs directory if not already there
    script_dir = Path(__file__).parent
    if script_dir.name == "docs":
        os.chdir(script_dir)

    parser = argparse.ArgumentParser(description="Build PeakRDL-rust documentation")
    parser.add_argument(
        "command",
        choices=[
            "build",
            "clean",
            "serve",
            "auto",
            "check-links",
            "spell-check",
            "pdf",
            "rtd-test",
        ],
        help="Command to execute",
    )
    parser.add_argument("-v", "--verbose", action="store_true", help="Verbose output")
    parser.add_argument(
        "-p",
        "--port",
        type=int,
        default=8000,
        help="Port for serve command (default: 8000)",
    )
    parser.add_argument(
        "--open",
        action="store_true",
        help="Open browser after building (for build command)",
    )

    args = parser.parse_args()

    if args.command == "clean":
        clean_build()

    elif args.command == "build":
        success = build_docs("html", verbose=args.verbose)
        if success and args.open:
            html_file = Path("_build/html/index.html").resolve()
            webbrowser.open(f"file://{html_file}")

    elif args.command == "serve":
        serve_docs(port=args.port, auto_build=False)

    elif args.command == "auto":
        serve_docs(port=args.port, auto_build=True)

    elif args.command == "check-links":
        check_links()

    elif args.command == "spell-check":
        spell_check()

    elif args.command == "pdf":
        build_docs("pdf", verbose=args.verbose)


if __name__ == "__main__":
    main()
