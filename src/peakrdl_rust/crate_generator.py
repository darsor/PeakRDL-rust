import abc
import shutil
from dataclasses import dataclass
from pathlib import Path
from typing import ClassVar, List, Optional, Tuple, Union

from systemrdl.node import (
    AddrmapNode,
    MemNode,
    RegfileNode,
)

from .design_state import DesignState


@dataclass
class Component(abc.ABC):
    """Base class for an RDL component or type, defined in its own Rust module"""

    template: ClassVar[str]  # Jinja template path

    file: Path  # Rust module file path
    module_comment: str
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
class Array:
    """Instantiated array"""

    # format-ready string, e.g. "[[[{}; 5]; 3]; 4]"
    type: str
    dims: list[int]
    # string using loop variables i0, i1, ..., etc. to calculate address of an instance
    # for example: "(((i0 * 3) + i1) * 4) + i2) * 0x100"
    addr_offset: str


@dataclass
class AddrmapRegInst(Instantiation):
    """Register instantiated within an Addrmap"""

    # address offset from parent component, only used if array is None
    addr_offset: Optional[int]
    access: Union[str, None]  # "R", "W", "RW", or None
    array: Optional[Array]


@dataclass
class AddrmapSubmapInst(Instantiation):
    """Addrmap or Regfile instantiated within an Addrmap"""

    # address offset from parent component, only used if array is None
    addr_offset: Optional[int]
    array: Optional[Array]


@dataclass
class RegFieldInst(Instantiation):
    """Field instantiated within a Register"""

    access: Union[str, None]  # "R", "W", "RW", or None
    primitive: str  # which unsigned rust type is used to represent
    encoding: Optional[str]  # encoding enum
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


@dataclass
class EnumVariant:
    """Variant of a user-defined enum"""

    comment: str
    name: str
    value: int


@dataclass
class Enum(Component):
    """User-defined enum type used to encode a field"""

    template: ClassVar[str] = "src/components/enum.rs"

    primitive: str  # which unsigned rust type is used to represent
    variants: List[EnumVariant]


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
