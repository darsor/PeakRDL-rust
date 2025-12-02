import shutil
from pathlib import Path

import pytest
from systemrdl.compiler import RDLCompiler

from peakrdl_rust.exporter import RustExporter
from peakrdl_rust.udps import ALL_UDPS


def test_access_mode_software() -> None:
    """Test exporter with software access mode (default)."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    crate_dir = Path(__file__).parent / "output" / "access_modes_software"
    
    if crate_dir.exists():
        shutil.rmtree(crate_dir)
    crate_dir.mkdir(exist_ok=True, parents=True)

    rdlc = RDLCompiler()
    
    # Load the UDPs
    for udp in ALL_UDPS:
        rdlc.register_udp(udp)
    udp_file = Path(__file__).parent / "../src/peakrdl_rust/udps/udps.rdl"
    rdlc.compile_file(str(udp_file))
    
    rdlc.compile_file(str(rdl_file))
    root = rdlc.elaborate(top_def_name="access_modes_test")

    exporter = RustExporter()
    exporter.export(
        root.top,
        path=str(crate_dir.parent),
        crate_name="access_modes_software",
        force=True,
        access_mode="software",  # Explicitly set software mode
    )
    
    # Verify the crate was created
    assert crate_dir.exists()
    assert (crate_dir / "Cargo.toml").exists()
    assert (crate_dir / "src" / "lib.rs").exists()


def test_access_mode_hardware() -> None:
    """Test exporter with hardware access mode."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    crate_dir = Path(__file__).parent / "output" / "access_modes_hardware"
    
    if crate_dir.exists():
        shutil.rmtree(crate_dir)
    crate_dir.mkdir(exist_ok=True, parents=True)

    rdlc = RDLCompiler()
    
    # Load the UDPs
    for udp in ALL_UDPS:
        rdlc.register_udp(udp)
    udp_file = Path(__file__).parent / "../src/peakrdl_rust/udps/udps.rdl"
    rdlc.compile_file(str(udp_file))
    
    rdlc.compile_file(str(rdl_file))
    root = rdlc.elaborate(top_def_name="access_modes_test")

    exporter = RustExporter()
    exporter.export(
        root.top,
        path=str(crate_dir.parent),
        crate_name="access_modes_hardware",
        force=True,
        access_mode="hardware",
    )
    
    # Verify the crate was created
    assert crate_dir.exists()
    assert (crate_dir / "Cargo.toml").exists()
    assert (crate_dir / "src" / "lib.rs").exists()


def test_access_mode_read_only() -> None:
    """Test exporter with read_only access mode."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    crate_dir = Path(__file__).parent / "output" / "access_modes_read_only"
    
    if crate_dir.exists():
        shutil.rmtree(crate_dir)
    crate_dir.mkdir(exist_ok=True, parents=True)

    rdlc = RDLCompiler()
    
    # Load the UDPs
    for udp in ALL_UDPS:
        rdlc.register_udp(udp)
    udp_file = Path(__file__).parent / "../src/peakrdl_rust/udps/udps.rdl"
    rdlc.compile_file(str(udp_file))
    
    rdlc.compile_file(str(rdl_file))
    root = rdlc.elaborate(top_def_name="access_modes_test")

    exporter = RustExporter()
    exporter.export(
        root.top,
        path=str(crate_dir.parent),
        crate_name="access_modes_read_only",
        force=True,
        access_mode="read_only",
    )
    
    # Verify the crate was created
    assert crate_dir.exists()
    assert (crate_dir / "Cargo.toml").exists()
    assert (crate_dir / "src" / "lib.rs").exists()


def test_access_mode_default() -> None:
    """Test that default behavior is software mode."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    crate_dir = Path(__file__).parent / "output" / "access_modes_default"
    
    if crate_dir.exists():
        shutil.rmtree(crate_dir)
    crate_dir.mkdir(exist_ok=True, parents=True)

    rdlc = RDLCompiler()
    
    # Load the UDPs
    for udp in ALL_UDPS:
        rdlc.register_udp(udp)
    udp_file = Path(__file__).parent / "../src/peakrdl_rust/udps/udps.rdl"
    rdlc.compile_file(str(udp_file))
    
    rdlc.compile_file(str(rdl_file))
    root = rdlc.elaborate(top_def_name="access_modes_test")

    exporter = RustExporter()
    exporter.export(
        root.top,
        path=str(crate_dir.parent),
        crate_name="access_modes_default",
        force=True,
        # No access_mode specified, should default to 'software'
    )
    
    # Verify the crate was created
    assert crate_dir.exists()
    assert (crate_dir / "Cargo.toml").exists()
    assert (crate_dir / "src" / "lib.rs").exists()


def test_access_mode_invalid() -> None:
    """Test that invalid access mode raises ValueError."""
    rdl_file = Path(__file__).parent / "rdl_src" / "access_modes.rdl"
    crate_dir = Path(__file__).parent / "output" / "access_modes_invalid"

    rdlc = RDLCompiler()
    
    # Load the UDPs
    for udp in ALL_UDPS:
        rdlc.register_udp(udp)
    udp_file = Path(__file__).parent / "../src/peakrdl_rust/udps/udps.rdl"
    rdlc.compile_file(str(udp_file))
    
    rdlc.compile_file(str(rdl_file))
    root = rdlc.elaborate(top_def_name="access_modes_test")

    exporter = RustExporter()
    
    with pytest.raises(ValueError, match="Invalid access_mode"):
        exporter.export(
            root.top,
            path=str(crate_dir.parent),
            crate_name="access_modes_invalid",
            force=True,
            access_mode="invalid_mode",  # This should raise ValueError
        )
