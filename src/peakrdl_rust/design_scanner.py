from pathlib import Path
from typing import List, Optional, Tuple, Union

from caseconverter import pascalcase, snakecase
from systemrdl.node import (
    AddressableNode,
    AddrmapNode,
    Node,
    RegfileNode,
    RegNode,
    RootNode,
)
from systemrdl.walker import RDLListener, RDLWalker, WalkerAction

from . import utils
from .crate_generator import (
    Addrmap,
    AddrmapRegInst,
    AddrmapSubmapInst,
    Array,
    RegFieldInst,
    Register,
)
from .design_state import DesignState
from .identifier_filter import kw_filter


class DesignScanner(RDLListener):
    def __init__(self, ds: DesignState) -> None:
        self.ds = ds
        self.msg = ds.top_node.env.msg

    @property
    def top_node(self) -> AddrmapNode:
        return self.ds.top_node

    def run(self) -> None:
        RDLWalker().walk(self.top_node, self)
        if self.msg.had_error:
            self.msg.fatal("Unable to export due to previous errors")

    def get_component_path(self, node: Node) -> Path:
        path = utils.crate_module_path(node)
        assert path is not None
        return (
            self.ds.output_dir / "src" / "components" / Path(*path).with_suffix(".rs")
        )

    def enter_addrmap_or_regfile(
        self, node: Union[AddrmapNode, RegfileNode]
    ) -> Optional[WalkerAction]:
        file = self.get_component_path(node)
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
                addr_offset = "i0"
                for i, dim in enumerate(dims):
                    arr_type = f"[{arr_type}; {dim}]"
                    if i != 0:
                        addr_offset = f"({addr_offset} * {dim}) + i{i}"

                if len(dims) > 1:
                    addr_offset = f"({addr_offset}) * {hex(stride)}"
                else:
                    addr_offset = f"{addr_offset} * {hex(stride)}"

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
                path = utils.crate_module_path(child)
                assert path is not None
                module_path = "::".join(["crate", "components"] + path)
                named_type_instances.append((inst_name, module_path))

        self.ds.components[file] = Addrmap(
            file=file,
            use_statements=[],
            anon_instances=anon_instances,
            named_type_instances=named_type_instances,
            named_type_declarations=[],
            comment=utils.doc_comment(node),
            type_name=pascalcase(node.inst_name),
            registers=registers,
            submaps=submaps,
        )
        return WalkerAction.Continue

    def enter_Addrmap(self, node: AddrmapNode) -> Optional[WalkerAction]:
        return self.enter_addrmap_or_regfile(node)

    def enter_Regfile(self, node: RegfileNode) -> Optional[WalkerAction]:
        return self.enter_addrmap_or_regfile(node)

    def enter_Reg(self, node: RegNode) -> Optional[WalkerAction]:
        # TODO: relax regwidth == accesswidth constraint on implementation
        # TODO: enforce max regwidth of 64

        file = self.get_component_path(node)
        if file in self.ds.components:
            # already handled
            return WalkerAction.SkipDescendants

        fields: List[RegFieldInst] = []
        for field in node.fields():
            fields.append(
                RegFieldInst(
                    comment=utils.doc_comment(field),
                    inst_name=snakecase(field.inst_name),
                    type_name="TODO",
                    access=utils.field_access(field),
                    primitive=utils.field_primitive(field),
                    bit_offset=field.low,
                    mask=(1 << field.width) - 1,
                )
            )

        self.ds.components[file] = Register(
            file=file,
            comment=utils.doc_comment(node),
            anon_instances=[],
            named_type_instances=[],
            named_type_declarations=[],
            use_statements=[],
            type_name=pascalcase(node.type_name),
            primitive=f"u{node.get_property('regwidth')}",
            reset_val=0,  # TODO
            fields=fields,
        )

        return WalkerAction.Continue

    def enter_Component(self, node: Node) -> Optional[WalkerAction]:
        if utils.is_anonymous(node):
            return WalkerAction.Continue

        type_name = node.type_name
        assert type_name is not None
        type_name = kw_filter(snakecase(type_name))

        parent = utils.parent_scope(node)
        assert parent is not None
        if isinstance(parent, RootNode):
            if type_name not in self.ds.top_component_modules:
                self.ds.top_component_modules.append(type_name)
            return WalkerAction.Continue

        file = self.get_component_path(parent)
        assert file in self.ds.components
        if type_name not in self.ds.components[file].named_type_declarations:
            self.ds.components[file].named_type_declarations.append(type_name)

        return WalkerAction.Continue
