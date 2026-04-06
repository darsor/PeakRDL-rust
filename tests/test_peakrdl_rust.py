import os
import re
import shutil
import subprocess
from pathlib import Path
from typing import TYPE_CHECKING

import jinja2 as jj
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
    crate_name = rdl_file.stem.replace("-", "_")

    crate_dir = Path(__file__).parent / "output" / crate_name
    crate_dir.mkdir(exist_ok=True, parents=True)

    src_dir = crate_dir / "src"
    src_dir.mkdir(exist_ok=True)

    generated_dir = src_dir / "generated"
    generated_dir.mkdir(exist_ok=True)

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

    x = RustExporter()
    x.export(
        top_nodes,
        path=str(generated_dir),
        fmt=True,
        force=True,
    )

    # copy integration test into package if it exists
    integration_test = rdl_file.parent / (rdl_file.stem + ".rs")
    if integration_test.exists():
        (crate_dir / "tests").mkdir(exist_ok=True)
        shutil.copyfile(integration_test, crate_dir / "tests" / integration_test.name)

    # render boilerplate templates
    templates_dir = Path(__file__).parent / "templates"
    loader = jj.FileSystemLoader(templates_dir.resolve())
    jj_env = jj.Environment(
        loader=loader,
        undefined=jj.StrictUndefined,
        trim_blocks=True,
        lstrip_blocks=True,
    )
    context = {
        "test_name": rdl_file.stem,
    }
    with open(crate_dir / "Cargo.toml", "w") as f:
        jj_env.get_template("Cargo.toml.jinja2").stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed
    with open(src_dir / "lib.rs", "w") as f:
        jj_env.get_template("lib.rs.jinja2").stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed

    return crate_dir


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
