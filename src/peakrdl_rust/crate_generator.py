import abc
import shutil
from dataclasses import dataclass
from pathlib import Path
from typing import ClassVar, List, Tuple, Union

from caseconverter import snakecase
from systemrdl import component
from systemrdl.node import (
    AddrmapNode,
    MemNode,
    RegfileNode,
)

from .design_state import DesignState
from .identifier_filter import kw_filter as kwf


@dataclass
class Component(abc.ABC):
    """Base class for an RDL component, defined in its own Rust module"""

    template: ClassVar[str]  # Jinja template path

    file: Path  # Rust module file path
    comment: str
    # anonymous components used in the body of this addrmap
    anon_instances: List[str]
    # component types declared in the body of this addrmap
    named_type_declarations: List[str]
    # named component types used in the body of this addrmap
    # (instance name, full type module path)
    named_type_instances: List[Tuple[str, str]]
    use_statements: List[str]
    type_name: str

    def render(self, ds: DesignState):
        self.file.parent.mkdir(parents=True, exist_ok=True)
        with self.file.open("w") as f:
            template = ds.jj_env.get_template(self.template)
            template.stream(ctx=self).dump(f)  # type: ignore # jinja incorrectly typed


@dataclass
class Instantiation:
    """Base class for instantiated components"""

    comment: str
    inst_name: str  # name of the instance
    type_name: str  # scoped type name


@dataclass
class AddrmapRegInst(Instantiation):
    """Register instantiated within an Addrmap"""

    addr_offset: int  # address offset from parent component
    access: Union[str, None]  # "R", "W", "RW", or None
    is_array: bool


@dataclass
class AddrmapSubmapInst(Instantiation):
    """Addrmap or Regfile instantiated within an Addrmap"""

    addr_offset: int  # address offset from parent component
    is_array: bool


@dataclass
class RegFieldInst(Instantiation):
    """Field instantiated within a Register"""

    access: Union[str, None]  # "R", "W", "RW", or None
    primitive: str  # which unsigned rust type is used to represent
    bit_offset: int  # lowest bit index
    mask: int  # bitmask of the width of the field


@dataclass
class Addrmap(Component):
    """Addrmap or Regfile component, defined in its own Rust module."""

    template: ClassVar[str] = "src/components/addrmap.rs"

    registers: List[AddrmapRegInst]
    submaps: List[AddrmapSubmapInst]


@dataclass
class Register(Component):
    """Register component, defined in its own Rust module"""

    template: ClassVar[str] = "src/components/reg.rs"

    primitive: str  # rust unsigned primitive used to represent
    reset_val: int
    fields: List[RegFieldInst]


def write_crate(
    top_nodes: List[Union[AddrmapNode, MemNode, RegfileNode]], ds: DesignState
):
    # Cargo.toml
    cargo_toml_path = ds.output_dir / "Cargo.toml"
    cargo_toml_path.parent.mkdir(parents=True, exist_ok=True)
    with cargo_toml_path.open("w") as f:
        context = {
            "package_name": "TODO",
            "package_version": "0.1.0+TODO",
        }
        template = ds.jj_env.get_template("Cargo.toml.tmpl")
        template.stream(context).dump(f)  # type: ignore # jinja incorrectly typed

    # .gitignore
    shutil.copyfile(ds.template_dir / ".gitignore", ds.output_dir / ".gitignore")

    # src/reg.rs
    (ds.output_dir / "src").mkdir(parents=True, exist_ok=True)
    shutil.copyfile(
        ds.template_dir / "src" / "reg.rs", ds.output_dir / "src" / "reg.rs"
    )

    # src/lib.rs
    lib_rs_path = ds.output_dir / "src" / "lib.rs"
    lib_rs_path.parent.mkdir(parents=True, exist_ok=True)
    context = {}
    with lib_rs_path.open("w") as f:
        template = ds.jj_env.get_template("src/lib.rs")
        template.stream(context).dump(f)  # type: ignore # jinja incorrectly typed

    # src/components.rs
    components_rs_path = ds.output_dir / "src" / "components.rs"
    components_rs_path.parent.mkdir(parents=True, exist_ok=True)
    with components_rs_path.open("w") as f:
        template = ds.jj_env.get_template("src/components.rs")
        template.stream(components=ds.top_component_modules).dump(f)  # type: ignore # jinja incorrectly typed

    for comp in ds.components.values():
        comp.render(ds)
