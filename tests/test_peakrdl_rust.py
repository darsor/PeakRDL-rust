import os
import re
import subprocess
from pathlib import Path
from typing import TYPE_CHECKING

import pytest
from systemrdl.compiler import RDLCompiler

from peakrdl_rust.exporter import RustExporter
from peakrdl_rust.udps import ALL_UDPS

if TYPE_CHECKING:
    from systemrdl.node import AddrmapNode


def get_rdl_files() -> list[Path]:
    rdl_src_dir = Path(__file__).parent / "rdl_src"
    return list(rdl_src_dir.glob("*.rdl"))


def do_export(rdl_file: Path) -> Path:
    crate_dir = Path(__file__).parent / "output"
    crate_dir.mkdir(exist_ok=True, parents=True)

    # Read the file to find top-level addrmap definitions
    with open(rdl_file) as f:
        content = f.read()

    # Use regex to find top-level addrmap names
    addrmap_pattern = r"^\s*addrmap\s+(\w+)"
    addrmap_names = re.findall(addrmap_pattern, content, re.MULTILINE)
    assert len(addrmap_names) > 0

    rdlc = RDLCompiler()

    # Load the UDPs
    for udp in ALL_UDPS:
        rdlc.register_udp(udp)
    # ... including the definition
    udp_file = Path(__file__).parent / "../src/peakrdl_rust/udps/udps.rdl"
    rdlc.compile_file(str(udp_file))

    rdlc.compile_file(str(rdl_file))

    top_nodes: list[AddrmapNode] = []
    for name in addrmap_names:
        root_node = rdlc.elaborate(top_def_name=name)
        top_nodes.append(root_node.top)

    crate_name = rdl_file.stem.replace("-", "_")

    x = RustExporter()
    x.export(
        top_nodes,
        path=str(crate_dir),
        crate_name=crate_name,
        force=True,
    )

    return crate_dir / crate_name


def do_cargo_test(crate_dir: Path) -> None:
    # shared target directory to cache compiled dependencies
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(Path(__file__).parent / "output" / "target")
    env["RUSTFLAGS"] = "-D warnings"
    subprocess.run(["cargo", "test"], cwd=crate_dir, check=True, env=env)


def do_clippy_check(crate_dir: Path) -> None:
    # shared target directory to cache compiled dependencies
    env = os.environ.copy()
    env["CARGO_TARGET_DIR"] = str(Path(__file__).parent / "output" / "target")
    subprocess.run(
        ["cargo", "clippy", "--", "-W", "clippy::pedantic", "-Dwarnings"],
        cwd=crate_dir,
        check=True,
        env=env,
    )


@pytest.mark.parametrize("rdl_file", get_rdl_files(), ids=lambda file: file.stem)
def test_generated_rust(rdl_file: Path) -> None:
    crate_dir = do_export(rdl_file)
    do_cargo_test(crate_dir)
    do_clippy_check(crate_dir)
