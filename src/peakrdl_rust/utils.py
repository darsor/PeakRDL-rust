from typing import List, Union

from caseconverter import snakecase
from systemrdl.node import FieldNode, Node, RegNode, RootNode

from peakrdl_rust.identifier_filter import kw_filter


def doc_comment(node: Node) -> str:
    name = node.get_property("name")
    desc = node.get_property("desc")

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


def crate_module_path(node: Node) -> Union[List[str], None]:
    parent = parent_scope(node)
    if parent is None:
        return None
    assert node.type_name is not None
    type_name = kw_filter(snakecase(node.type_name))
    if isinstance(parent, RootNode):
        return [type_name]
    parent_path = crate_module_path(parent)
    if parent_path is None:
        return None
    if is_anonymous(node):
        return parent_path + [type_name]
    else:
        return parent_path + ["named_types", type_name]


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


def field_primitive(node: FieldNode) -> str:
    if node.width == 1:
        return "bool"
    for w in (8, 16, 32, 64):
        if w >= node.width:
            return f"u{w}"
    raise RuntimeError("Field widths > 64 are not supported")
