import re
import subprocess
from pathlib import Path
from typing import List

import pytest
from systemrdl.compiler import RDLCompiler

from peakrdl_rust.exporter import RustExporter


def get_rdl_files() -> List[Path]:
    rdl_src_dir = Path(__file__).parent / "rdl_src"
    return [file for file in rdl_src_dir.glob("*.rdl")]


def do_export(rdl_file: Path) -> Path:
    crate_dir = Path(__file__).parent / "output" / rdl_file.stem
    crate_dir.mkdir(exist_ok=True)

    # Read the file to find top-level addrmap definitions
    with open(rdl_file, "r") as f:
        content = f.read()

    # Use regex to find top-level addrmap names
    addrmap_pattern = r"^\s*addrmap\s+(\w+)"
    addrmap_names = re.findall(addrmap_pattern, content, re.MULTILINE)
    assert len(addrmap_names) > 0

    rdlc = RDLCompiler()
    rdlc.compile_file(str(rdl_file))

    top_nodes = []
    for name in addrmap_names:
        root_node = rdlc.elaborate(top_def_name=name)
        top_nodes.append(root_node.top)

    x = RustExporter()
    x.export(
        top_nodes[0],
        path=str(crate_dir),
        force=True,
    )

    return crate_dir


def do_cargo_test(crate_dir: Path):
    subprocess.run(["cargo", "test"], cwd=crate_dir, check=True)


@pytest.mark.parametrize("rdl_file", get_rdl_files(), ids=lambda file: file.stem)
def test_generated_rust(rdl_file: Path):
    crate_dir = do_export(rdl_file)
    do_cargo_test(crate_dir)
