import abc
import math
from dataclasses import dataclass
from pathlib import Path
from typing import ClassVar, Optional, Union

import jinja2 as jj
from caseconverter import pascalcase, snakecase
from systemrdl.node import (
    AddressableNode,
    AddrmapNode,
    FieldNode,
    MemNode,
    Node,
    RegfileNode,
    RegNode,
    RootNode,
)
from systemrdl.rdltypes.user_enum import UserEnum
from systemrdl.walker import RDLListener, RDLWalker, WalkerAction

from . import utils
from .identifier_filter import kw_filter, kw_filter_path


@dataclass
class Component(abc.ABC):
    """Base class for an RDL component or type, defined in its own Rust module"""

    template: ClassVar[str]  # Jinja template path

    file: Path  # Rust module file path
    module_comment: str
    comment: str
    # anonymous components used in the body of this addrmap
    anon_instances: list[str]
    # component types declared in the body of this addrmap
    named_type_declarations: list[str]
    # named component types used in the body of this addrmap
    # (instance name, full type module path)
    named_type_instances: list[tuple[str, str]]
    use_statements: list[str]
    type_name: str

    def render(self, output_dir: Path, jj_env: jj.Environment) -> None:
        out_file = output_dir / self.file
        out_file.parent.mkdir(parents=True, exist_ok=True)
        with out_file.open("w") as f:
            template = jj_env.get_template(self.template)
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
class RegisterInst(Instantiation):
    """Register instantiated within an Addrmap"""

    # address offset from parent component, only used if array is None
    addr_offset: Optional[int]
    access: str  # "R", "W", or "RW"
    array: Optional[Array]


@dataclass
class SubmapInst(Instantiation):
    """Addrmap or Regfile instantiated within an Addrmap"""

    # address offset from parent component, only used if array is None
    addr_offset: Optional[int]
    array: Optional[Array]


@dataclass
class MemoryInst(Instantiation):
    """Memory instantiated within an Addrmap"""

    # address offset from parent component, only used if array is None
    addr_offset: Optional[int]
    array: Optional[Array]


@dataclass
class FieldInst(Instantiation):
    """Field instantiated within a Register"""

    access: Union[str, None]  # "R", "W", "RW", or None
    primitive: str  # which unsigned rust type is used to represent
    encoding: Optional[str]  # encoding enum
    exhaustive: bool  # True if the encoding is exhaustive (covers all bit patterns)
    bit_offset: int  # lowest bit index
    width: int  # bit width
    mask: int  # bitmask of the width of the field
    reset_val: Union[int, str]
    is_signed: Optional[bool]
    fracwidth: Optional[int]
    intwidth: Optional[int]


@dataclass
class Addrmap(Component):
    """Addrmap or Regfile component, defined in its own Rust module."""

    template: ClassVar[str] = "src/components/addrmap.rs"

    registers: list[RegisterInst]
    submaps: list[SubmapInst]
    memories: list[MemoryInst]
    size: int


@dataclass
class Memory(Component):
    """Memory component, defined in its own Rust module."""

    template: ClassVar[str] = "src/components/memory.rs"

    mementries: int
    memwidth: int
    primitive: str
    registers: list[RegisterInst]
    size: int
    access: str  # "R", "W", or "RW"


@dataclass
class Register(Component):
    """Register component, defined in its own Rust module"""

    template: ClassVar[str] = "src/components/register.rs"

    regwidth: int
    accesswidth: int
    reset_val: int
    fields: list[FieldInst]
    has_sw_readable: bool


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
    variants: list[EnumVariant]


class ContextScanner(RDLListener):
    def __init__(self, top_nodes: list[AddrmapNode]) -> None:
        self.top_nodes = top_nodes
        self.top_component_modules: list[str] = []
        self.components: dict[Path, Component] = {}
        self.msg = top_nodes[0].env.msg

    def run(self) -> None:
        for node in self.top_nodes:
            RDLWalker().walk(node, self)
        if self.msg.had_error:
            self.msg.fatal("Unable to export due to previous errors")

    def get_node_module_file(self, node: Node) -> Path:
        """Get the file name of the module defining a Node"""
        module_names = utils.crate_module_path(node)
        return self.file_from_modules(module_names)

    def get_enum_module_file(self, field: FieldNode, enum: type[UserEnum]) -> Path:
        """Get the file name of the module defining an Enum"""
        module_names = utils.crate_enum_module_path(field, enum)
        return self.file_from_modules(module_names)

    def file_from_modules(self, module_names: list[str]) -> Path:
        """Construct a filename from a list of module names in the hierarchy"""
        escaped_names = list(map(kw_filter_path, module_names))
        return Path("src") / "components" / Path(*escaped_names).with_suffix(".rs")

    def enter_addrmap_or_regfile_or_memory(
        self, node: Union[AddrmapNode, RegfileNode, MemNode]
    ) -> Optional[WalkerAction]:
        file = self.get_node_module_file(node)
        if file in self.components:
            # already handled
            return WalkerAction.SkipDescendants

        registers: list[RegisterInst] = []
        submaps: list[SubmapInst] = []
        memories: list[MemoryInst] = []
        anon_instances: list[str] = []
        named_type_instances: list[tuple[str, str]] = []

        for child in node.children():
            if not isinstance(child, AddressableNode):
                continue
            inst_name = snakecase(child.inst_name)
            if child.is_array:
                dims = child.array_dimensions
                assert dims is not None
                stride = child.array_stride
                assert stride is not None

                arr_type = "{}"
                for dim in dims[::-1]:
                    arr_type = f"[{arr_type}; {dim}]"

                addr_calc = "i0"
                for i, dim in enumerate(dims):
                    if i != 0:
                        addr_calc = f"({addr_calc} * {dim}) + i{i}"

                if len(dims) > 1:
                    addr_calc = f"({addr_calc}) * {hex(stride)}"
                else:
                    addr_calc = f"{addr_calc} * {hex(stride)}"

                if child.raw_absolute_address != 0:
                    addr_calc = f"{hex(child.raw_address_offset)} + {addr_calc}"

                array = Array(type=arr_type, dims=dims, addr_offset=addr_calc)
                addr_offset = None
            else:
                array = None
                addr_offset = child.address_offset

            if isinstance(child, RegNode):
                if not (access := utils.reg_access(child)):
                    continue
                registers.append(
                    RegisterInst(
                        comment=utils.doc_comment(child),
                        inst_name=inst_name,
                        type_name=kw_filter(inst_name)
                        + "::"
                        + kw_filter(utils.rust_type_name(child)),
                        array=array,
                        addr_offset=addr_offset,
                        access=access,
                    )
                )
            elif isinstance(child, (AddrmapNode, RegfileNode)):
                submaps.append(
                    SubmapInst(
                        comment=utils.doc_comment(child),
                        inst_name=inst_name,
                        type_name=kw_filter(inst_name)
                        + "::"
                        + kw_filter(utils.rust_type_name(child)),
                        array=array,
                        addr_offset=addr_offset,
                    )
                )
            elif isinstance(child, MemNode):
                memories.append(
                    MemoryInst(
                        comment=utils.doc_comment(child),
                        inst_name=inst_name,
                        type_name=kw_filter(inst_name)
                        + "::"
                        + kw_filter(utils.rust_type_name(child)),
                        array=array,
                        addr_offset=addr_offset,
                    )
                )
            else:
                raise NotImplementedError(f"Unhandled node type: {type(child)}")

            if utils.is_anonymous(child):
                anon_instances.append(inst_name)
            else:
                module_names = utils.crate_module_path(child)
                scoped_module = "::".join(
                    ["crate", "components"] + list(map(kw_filter, module_names))
                )
                named_type_instances.append((inst_name, scoped_module))

        if isinstance(node, (AddrmapNode, RegfileNode)):
            comp_type_name = "Addrmap" if isinstance(node, AddrmapNode) else "Regfile"
            self.components[file] = Addrmap(
                file=file,
                module_comment=f"{comp_type_name}: {node.get_property('name')}",
                comment=utils.doc_comment(node),
                use_statements=[],
                anon_instances=anon_instances,
                named_type_instances=named_type_instances,
                named_type_declarations=[],
                type_name=utils.rust_type_name(node),
                registers=registers,
                submaps=submaps,
                memories=memories,
                size=node.size,
            )
        else:
            assert len(submaps) == 0
            assert len(memories) == 0
            if not (access := utils.field_access(node)):
                return WalkerAction.Continue
            memwidth = node.get_property("memwidth")
            primitive_width = 2 ** int(math.ceil(math.log2(memwidth)))
            self.components[file] = Memory(
                file=file,
                module_comment=f"Memory: {node.get_property('name')}",
                comment=utils.doc_comment(node),
                use_statements=[],
                anon_instances=anon_instances,
                named_type_instances=named_type_instances,
                named_type_declarations=[],
                type_name=utils.rust_type_name(node),
                mementries=node.get_property("mementries"),
                memwidth=memwidth,
                primitive=f"u{primitive_width}",
                registers=registers,
                size=node.size,
                access=access,
            )
        return WalkerAction.Continue

    def enter_Addrmap(self, node: AddrmapNode) -> Optional[WalkerAction]:
        return self.enter_addrmap_or_regfile_or_memory(node)

    def enter_Regfile(self, node: RegfileNode) -> Optional[WalkerAction]:
        return self.enter_addrmap_or_regfile_or_memory(node)

    def enter_Mem(self, node: MemNode) -> Optional[WalkerAction]:
        return self.enter_addrmap_or_regfile_or_memory(node)

    def enter_Reg(self, node: RegNode) -> Optional[WalkerAction]:
        file = self.get_node_module_file(node)
        if file in self.components:
            # already handled
            return WalkerAction.SkipDescendants

        reg_reset_val = 0
        fields: list[FieldInst] = []
        for field in node.fields():
            encoding = field.get_property("encode")
            if encoding is not None:
                encoding_name = (
                    kw_filter(snakecase(field.inst_name))
                    + "::"
                    + kw_filter(pascalcase(encoding.type_name))
                )
                num_encodings = len(encoding.members)
                # encoded values must be unique within an enum per 6.2.5.1-b-3
                exhaustive = bool(num_encodings == 2**field.width)
            else:
                encoding_name = None
                exhaustive = True

            primitive = utils.field_primitive(field, allow_bool=(encoding is None))

            reset_val_int = utils.field_reset_value(field)
            reg_reset_val |= reset_val_int << field.low
            reset_val = str(reset_val_int)
            if encoding is not None:
                is_valid_variant = False
                for variant_name, variant_value in encoding.members.items():
                    if int(variant_value) == reset_val_int:
                        is_valid_variant = True
                        reset_val = (
                            kw_filter(snakecase(field.inst_name))
                            + "::"
                            + kw_filter(pascalcase(encoding.type_name))
                            + "::"
                            + kw_filter(pascalcase(variant_name))
                        )
                        if not exhaustive:
                            reset_val = f"Ok({reset_val})"
                        break
                if not is_valid_variant:
                    # specified (or unspecified default 0) reset value is not a valid
                    # encoding
                    reset_val = (
                        f"Err(crate::encode::UnknownVariant::new({reset_val_int}))"
                    )
            elif primitive == "bool":
                reset_val = "true" if reset_val_int else "false"
            elif field.get_property("is_signed"):
                if reset_val_int >= 2 ** (field.width - 1):
                    reset_val_int = reset_val_int - 2**field.width
                reset_val = str(reset_val_int)

            fracwidth: Union[int, None] = field.get_property("fracwidth")
            if fracwidth is not None:
                reset_val = f"{reset_val_int * 2**-fracwidth:.32g}_f64"

            fields.append(
                FieldInst(
                    comment=utils.doc_comment(field),
                    inst_name=snakecase(field.inst_name),
                    type_name=pascalcase(field.inst_name),
                    access=utils.field_access(field),
                    primitive=primitive,
                    encoding=encoding_name,
                    exhaustive=exhaustive,
                    bit_offset=field.low,
                    width=field.width,
                    mask=(1 << field.width) - 1,
                    reset_val=reset_val,
                    is_signed=field.get_property("is_signed"),
                    fracwidth=field.get_property("fracwidth"),
                    intwidth=field.get_property("intwidth"),
                )
            )

        self.components[file] = Register(
            file=file,
            module_comment=f"Register: {node.get_property('name')}",
            comment=utils.doc_comment(node),
            anon_instances=[],
            named_type_instances=[],
            named_type_declarations=[],
            use_statements=[],
            type_name=utils.rust_type_name(node),
            regwidth=node.get_property("regwidth"),
            accesswidth=node.get_property("accesswidth"),
            reset_val=reg_reset_val,
            fields=fields,
            has_sw_readable=node.has_sw_readable,
        )

        return WalkerAction.Continue

    def enter_Component(self, node: Node) -> Optional[WalkerAction]:
        if utils.is_anonymous(node) or isinstance(node, FieldNode):
            return WalkerAction.Continue

        module_name = kw_filter(utils.rust_module_name(node))

        parent = utils.parent_scope(node)
        assert parent is not None
        if isinstance(parent, RootNode):
            utils.append_unique(self.top_component_modules, module_name)
            return WalkerAction.Continue

        file = self.get_node_module_file(parent)
        assert file in self.components
        utils.append_unique(self.components[file].named_type_declarations, module_name)

        return WalkerAction.Continue

    def enter_Field(self, node: FieldNode) -> Optional[WalkerAction]:
        field = node
        encoding = field.get_property("encode")
        if encoding is None:
            return WalkerAction.Continue

        comment = ""
        declaring_parent = utils.enum_parent_scope(field, encoding)
        assert declaring_parent is not None
        module_names = utils.crate_enum_module_path(field, encoding)
        module_name = module_names[-1]

        if declaring_parent is field:
            # Enum used in the same field where it's defined. Its definition can't
            # be reused, so consider it an anonymous type even though it has a name.
            owning_reg = self.file_from_modules(module_names[:-1])
            assert owning_reg in self.components
            utils.append_unique(
                self.components[owning_reg].anon_instances, kw_filter(module_name)
            )
            comment = utils.doc_comment(field)
        else:
            # Enum is a reusable, named type. The module defining it is a submodule
            # of the "named_types" submodule of its declaring parent.
            #
            # Components that use this module have a "pub use" to re-export the
            # submodule as the name of the field that uses it.

            # 1. Add to the declaring parent's named_type_declarations
            if isinstance(declaring_parent, RootNode):
                utils.append_unique(self.top_component_modules, kw_filter(module_name))
            else:
                assert module_names[-2] == "named_types"
                parent_path = self.file_from_modules(module_names[:-2])
                assert parent_path in self.components
                utils.append_unique(
                    self.components[parent_path].named_type_declarations,
                    kw_filter(module_name),
                )

            # 2. Add to the instantiating node's named_type_instances
            instantiating_node = field.parent
            instantiating_file = self.get_node_module_file(instantiating_node)
            assert instantiating_file in self.components
            scoped_module = "::".join(
                ["crate", "components"] + list(map(kw_filter, module_names))
            )
            self.components[instantiating_file].named_type_instances.append(
                (snakecase(field.inst_name), scoped_module)
            )

        file = self.get_enum_module_file(field, encoding)
        if file in self.components:
            # already handled
            return WalkerAction.Continue

        # collect necessary context to render the Enum module template
        variants = []
        for variant in encoding.members.values():
            variants.append(
                EnumVariant(
                    comment=utils.doc_comment(variant),
                    name=pascalcase(variant.name),
                    value=variant.value,
                )
            )

        self.components[file] = Enum(
            file=file,
            module_comment=f"Field Enum: {node.get_property('name')}",
            comment=comment,
            anon_instances=[],
            named_type_declarations=[],
            named_type_instances=[],
            use_statements=[],
            type_name=pascalcase(encoding.type_name),
            primitive=utils.field_primitive(field, allow_bool=False),
            variants=variants,
        )

        return WalkerAction.Continue
