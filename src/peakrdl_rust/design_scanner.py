from pathlib import Path
from typing import List, Optional, Tuple, Union

from caseconverter import pascalcase, snakecase
from systemrdl.node import (
    AddressableNode,
    AddrmapNode,
    FieldNode,
    Node,
    RegfileNode,
    RegNode,
    RootNode,
)
from systemrdl.rdltypes.user_enum import UserEnum
from systemrdl.walker import RDLListener, RDLWalker, WalkerAction

from . import utils
from .crate_generator import (
    Addrmap,
    AddrmapRegInst,
    AddrmapSubmapInst,
    Array,
    Enum,
    EnumVariant,
    RegFieldInst,
    Register,
)
from .design_state import DesignState
from .identifier_filter import kw_filter


class DesignScanner(RDLListener):
    def __init__(self, ds: DesignState) -> None:
        self.ds = ds
        self.msg = ds.top_nodes[0].env.msg

    @property
    def top_nodes(self) -> List[AddrmapNode]:
        return self.ds.top_nodes

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

    def file_from_modules(self, module_names: List[str]) -> Path:
        """Construct a filename from a list of module names in the hierarchy"""
        return (
            self.ds.output_dir
            / "src"
            / "components"
            / Path(*module_names).with_suffix(".rs")
        )

    def enter_addrmap_or_regfile(
        self, node: Union[AddrmapNode, RegfileNode]
    ) -> Optional[WalkerAction]:
        file = self.get_node_module_file(node)
        if file in self.ds.components:
            # already handled
            return WalkerAction.SkipDescendants

        registers: List[AddrmapRegInst] = []
        submaps: List[AddrmapSubmapInst] = []
        anon_instances: List[str] = []
        named_type_instances: List[Tuple[str, str]] = []

        for child in node.children():
            assert isinstance(child, AddressableNode)
            inst_name = kw_filter(snakecase(child.inst_name))
            if child.is_array:
                dims = child.array_dimensions
                assert dims is not None
                stride = child.array_stride
                assert stride is not None

                arr_type = "{}"
                for dim in dims[::-1]:
                    arr_type = f"[{arr_type}; {dim}]"

                addr_offset = "i0"
                for i, dim in enumerate(dims):
                    if i != 0:
                        addr_offset = f"({addr_offset} * {dim}) + i{i}"

                if len(dims) > 1:
                    addr_offset = f"({addr_offset}) * {hex(stride)}"
                else:
                    addr_offset = f"{addr_offset} * {hex(stride)}"

                if child.raw_absolute_address != 0:
                    addr_offset = f"{hex(child.raw_address_offset)} + {addr_offset}"

                array = Array(type=arr_type, dims=dims, addr_offset=addr_offset)
                addr_offset = None
            else:
                array = None
                addr_offset = child.address_offset

            if isinstance(child, RegNode):
                if (access := utils.reg_access(child)) is None:
                    continue
                registers.append(
                    AddrmapRegInst(
                        comment=utils.doc_comment(child),
                        inst_name=inst_name,
                        type_name=inst_name + "::" + pascalcase(child.type_name),
                        array=array,
                        addr_offset=addr_offset,
                        access=access,
                    )
                )
            elif isinstance(child, (AddrmapNode, RegfileNode)):
                submaps.append(
                    AddrmapSubmapInst(
                        comment=utils.doc_comment(child),
                        inst_name=inst_name,
                        type_name=inst_name + "::" + pascalcase(child.type_name),
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
                scoped_module = "::".join(["crate", "components"] + module_names)
                named_type_instances.append((inst_name, scoped_module))

        comp_type_name = "Addrmap" if isinstance(node, AddrmapNode) else "Regfile"
        self.ds.components[file] = Addrmap(
            file=file,
            module_comment=f"{comp_type_name}: {node.get_property('name')}",
            comment=utils.doc_comment(node),
            use_statements=[],
            anon_instances=anon_instances,
            named_type_instances=named_type_instances,
            named_type_declarations=[],
            type_name=pascalcase(node.inst_name),
            registers=registers,
            submaps=submaps,
            size=node.size,
        )
        return WalkerAction.Continue

    def enter_Addrmap(self, node: AddrmapNode) -> Optional[WalkerAction]:
        return self.enter_addrmap_or_regfile(node)

    def enter_Regfile(self, node: RegfileNode) -> Optional[WalkerAction]:
        return self.enter_addrmap_or_regfile(node)

    def enter_Reg(self, node: RegNode) -> Optional[WalkerAction]:
        # TODO: enforce max regwidth of 64

        file = self.get_node_module_file(node)
        if file in self.ds.components:
            # already handled
            return WalkerAction.SkipDescendants

        reg_reset_val = 0
        fields: List[RegFieldInst] = []
        for field in node.fields():
            encoding = field.get_property("encode")
            if encoding is not None:
                encoding = (
                    kw_filter(snakecase(field.inst_name))
                    + "::"
                    + pascalcase(encoding.type_name)
                )

            reset_val = utils.field_reset_value(field)
            reg_reset_val |= reset_val << field.low

            fields.append(
                RegFieldInst(
                    comment=utils.doc_comment(field),
                    inst_name=snakecase(field.inst_name),
                    type_name="TODO",
                    access=utils.field_access(field),
                    primitive=utils.field_primitive(
                        field, allow_bool=(encoding is None)
                    ),
                    encoding=encoding,
                    bit_offset=field.low,
                    width=field.width,
                    mask=(1 << field.width) - 1,
                )
            )

        self.ds.components[file] = Register(
            file=file,
            module_comment=f"Register: {node.get_property('name')}",
            comment=utils.doc_comment(node),
            anon_instances=[],
            named_type_instances=[],
            named_type_declarations=[],
            use_statements=[],
            type_name=pascalcase(node.type_name),
            regwidth=node.get_property("regwidth"),
            accesswidth=node.get_property("accesswidth"),
            reset_val=reg_reset_val,
            fields=fields,
        )

        return WalkerAction.Continue

    def enter_Component(self, node: Node) -> Optional[WalkerAction]:
        if utils.is_anonymous(node) or isinstance(node, FieldNode):
            return WalkerAction.Continue

        type_name = node.type_name
        assert type_name is not None
        type_name = kw_filter(snakecase(type_name))

        parent = utils.parent_scope(node)
        assert parent is not None
        if isinstance(parent, RootNode):
            utils.append_unique(self.ds.top_component_modules, type_name)
            return WalkerAction.Continue

        file = self.get_node_module_file(parent)
        assert file in self.ds.components
        utils.append_unique(self.ds.components[file].named_type_declarations, type_name)

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
            # be reused, so it's consider an anonymous type even though it has a name.
            owning_reg = self.file_from_modules(module_names[:-1])
            assert owning_reg in self.ds.components
            utils.append_unique(
                self.ds.components[owning_reg].anon_instances, module_name
            )
            comment = utils.doc_comment(field)
        else:
            # Enum is a reusable, named type. The module defining it is a submodule
            # of the "named_types" submodule of its declaring parent.
            #
            # Components that use this module have a "pub use" to re-export the submodule
            # as the name of the field that uses it.

            # 1. Add to the declaring parent's named_type_declarations
            if isinstance(declaring_parent, RootNode):
                utils.append_unique(self.ds.top_component_modules, module_name)
            else:
                assert module_names[-2] == "named_types"
                parent_path = self.file_from_modules(module_names[:-2])
                assert parent_path in self.ds.components
                utils.append_unique(
                    self.ds.components[parent_path].named_type_declarations, module_name
                )

            # 2. Add to the instantiating node's named_type_instances
            instantiating_node = field.parent
            instantiating_file = self.get_node_module_file(instantiating_node)
            assert instantiating_file in self.ds.components
            scoped_module = "::".join(["crate", "components"] + module_names)
            self.ds.components[instantiating_file].named_type_instances.append(
                (kw_filter(snakecase(field.inst_name)), scoped_module)
            )

        file = self.get_enum_module_file(field, encoding)
        if file in self.ds.components:
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

        self.ds.components[file] = Enum(
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
