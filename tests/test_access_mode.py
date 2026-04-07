from pathlib import Path

import pytest
from test_peakrdl_rust import do_cargo_test, do_clippy_check, do_export


def test_access_mode_software() -> None:
    """Test exporter with software access mode (default)."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    crate_dir = do_export(rdl_file, "access_modes_software", access_mode="software")
    do_cargo_test(crate_dir)
    do_clippy_check(crate_dir)


def test_access_mode_hardware() -> None:
    """Test exporter with hardware access mode."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    crate_dir = do_export(rdl_file, "access_modes_hardware", access_mode="hardware")
    do_cargo_test(crate_dir)
    do_clippy_check(crate_dir)


def test_read_only() -> None:
    """Test exporter with read_only mode."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    crate_dir = do_export(
        rdl_file, "access_modes_read_only", access_mode="software", read_only=True
    )
    do_cargo_test(crate_dir)
    do_clippy_check(crate_dir)


def test_access_mode_invalid() -> None:
    """Test that invalid access mode raises ValueError."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    with pytest.raises(ValueError, match="Invalid access_mode"):
        do_export(rdl_file, "access_modes_invalid", access_mode="invalid_mode")
