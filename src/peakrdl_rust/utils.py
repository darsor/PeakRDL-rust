from typing import Any, List, Union

from caseconverter import snakecase
from systemrdl.node import FieldNode, Node, RegNode, RootNode
from systemrdl.rdltypes.user_enum import UserEnum

from peakrdl_rust.identifier_filter import kw_filter


def doc_comment(node: Union[Node, UserEnum]) -> str:
    if isinstance(node, Node):
        name = node.get_property("name")
        desc = node.get_property("desc")
    else:
        name = node.rdl_name
        desc = node.rdl_desc

    if name is None and desc is None:
        return ""

    comment = ""
    if name is not None:
        comment += "/// " + name
        if desc is not None:
            comment += "\n///\n"
    if desc is not None:
        comment += "\n".join(["/// " + line for line in desc.splitlines()])
    return comment


def is_anonymous(node: Node) -> bool:
    return node.orig_type_name is None


def parent_scope(node: Node) -> Union[Node, None]:
    # Due to namespace nesting properties, it is guaranteed that the parent
    # scope definition is also going to be one of the node's ancestors.
    # Seek up and find it
    current_parent_node = node.parent
    while current_parent_node:
        if current_parent_node.inst.original_def is None:
            # Original def reference is unknown
            return None
        if current_parent_node.inst.original_def is node.inst.parent_scope:
            # Parent node's definition matches the scope we're looking for
            return current_parent_node

        current_parent_node = current_parent_node.parent


def enum_parent_scope(node: FieldNode, encoding: type[UserEnum]) -> Union[Node, None]:
    """Get the node within which a field's enum type is declared."""
    assert node.get_property("encode") is encoding
    enum_scope = encoding.get_parent_scope()
    if enum_scope is None:
        return None
    # Due to namespace nesting properties, it is guaranteed that the parent
    # scope definition is also going to be one of the node's ancestors.
    # Seek up and find it
    current_parent_node = node
    while current_parent_node:
        if current_parent_node.inst.original_def is None:
            # Original def reference is unknown
            return None
        if current_parent_node.inst.original_def is enum_scope:
            # Parent node's definition matches the scope we're looking for
            return current_parent_node

        current_parent_node = current_parent_node.parent


def crate_module_path(node: Node) -> List[str]:
    parent = parent_scope(node)
    assert parent is not None
    assert node.type_name is not None
    type_name = kw_filter(snakecase(node.type_name))
    if isinstance(parent, RootNode):
        return [type_name]
    parent_path = crate_module_path(parent)
    if is_anonymous(node):
        return parent_path + [type_name]
    else:
        return parent_path + ["named_types", type_name]


def crate_enum_module_path(field: FieldNode, enum: type[UserEnum]) -> List[str]:
    assert field.get_property("encode") is enum
    declaring_parent = enum_parent_scope(field, enum)
    assert declaring_parent is not None

    module_name = kw_filter(snakecase(enum.type_name))

    if isinstance(declaring_parent, RootNode):
        return [module_name]

    if declaring_parent is field:
        # Enum used in the same field where it's defined. Its definition can't
        # be reused. The module defining it is a submodule of the containing
        # register. The module name is the name of the field that uses it.
        module_name = kw_filter(snakecase(field.inst_name))
        parent_reg_modules = crate_module_path(field.parent)
        assert parent_reg_modules is not None
        return parent_reg_modules + [module_name]
    else:
        # Enum not used in the same field where it's defined, so it must have been
        # defined in a parent of the field (not in a field component). The module
        # defining it is a submodule the "named_types" submodule of its declaring
        # parent. The name of the module is the name of the enum type.
        parent_modules = crate_module_path(declaring_parent)
        assert parent_modules is not None
        return parent_modules + ["named_types", module_name]


def reg_access(node: RegNode) -> Union[str, None]:
    if node.has_sw_readable:
        if node.has_sw_writable:
            return "RW"
        else:
            return "R"
    else:
        if node.has_sw_writable:
            return "W"
        else:
            return None


def field_access(node: FieldNode) -> Union[str, None]:
    if node.is_sw_readable:
        if node.is_sw_writable:
            return "RW"
        else:
            return "R"
    else:
        if node.is_sw_writable:
            return "W"
        else:
            return None


def field_primitive(node: FieldNode, allow_bool: bool = True) -> str:
    if node.width == 1 and allow_bool:
        return "bool"
    for w in (8, 16, 32, 64):
        if w >= node.width:
            return f"u{w}"
    raise RuntimeError("Field widths > 64 are not supported")


def append_unique(list: List, obj: Any):
    """Append an object to a list only if it's not already present"""
    if obj not in list:
        list.append(obj)
