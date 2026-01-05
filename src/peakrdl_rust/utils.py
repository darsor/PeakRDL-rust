from typing import Any, Union

from caseconverter import pascalcase, snakecase
from systemrdl.node import (
    AddressableNode,
    FieldNode,
    MemNode,
    Node,
    RegNode,
    RootNode,
    SignalNode,
)
from systemrdl.rdltypes.references import PropertyReference
from systemrdl.rdltypes.user_enum import UserEnum

from peakrdl_rust.identifier_filter import kw_filter


def doc_comment(node: Union[Node, UserEnum]) -> str:
    if isinstance(node, Node):
        name: Union[str, None] = node.get_property("name")
        desc = node.get_property("desc")
    else:
        name = node.rdl_name
        desc = node.rdl_desc

    if name is not None:
        name = name.strip()
        if not name:
            name = None

    if desc is not None:
        desc = desc.strip()
        if not desc:
            desc = None

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
    """Get the parent node that the given node was declared within
    (lexical scope)."""
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
    return None


def _type_name_normalization(node: Node) -> tuple[str, Union[str, None]]:
    """Get the SystemRDL type name and optional type normaliation suffix"""
    # The SystemRDL compiler adds unique identifiers to any type name
    # if properties are dynamically set. But anonymous instances can not be
    # reused, and don't need to be uniquified. If the node is anonymously defined,
    # use the instance name as the type name (ignoring the unique suffix).
    if is_anonymous(node):
        return (node.inst_name, None)

    assert node.type_name is not None
    assert node.orig_type_name is not None
    return (node.orig_type_name, node.type_name.removeprefix(node.orig_type_name))


def rust_type_name(node: Node) -> str:
    """Get the Rust type name of a component, in PascalCase."""
    type_name, suffix = _type_name_normalization(node)
    rust_type_name = str(pascalcase(type_name))
    # Don't change the case of the suffix. It gets really messy in PascalCase.
    if suffix is not None:
        rust_type_name += suffix
    return rust_type_name


def rust_module_name(node: Node) -> str:
    """Get the Rust module name of a component, in snake_case."""
    type_name, suffix = _type_name_normalization(node)
    if suffix is not None:
        type_name = type_name + suffix
    return str(snakecase(type_name))


def enum_parent_scope(node: FieldNode, encoding: type[UserEnum]) -> Union[Node, None]:
    """Get the node within which a field's enum type is declared."""
    assert node.get_property("encode") is encoding
    enum_scope = encoding.get_parent_scope()
    if enum_scope is None:
        return None
    # Due to namespace nesting properties, it is guaranteed that the parent
    # scope definition is also going to be one of the node's ancestors.
    # Seek up and find it
    current_parent_node: Union[Node, None] = node
    while current_parent_node:
        if current_parent_node.inst.original_def is None:
            # Original def reference is unknown
            return None
        if current_parent_node.inst.original_def is enum_scope:
            # Parent node's definition matches the scope we're looking for
            return current_parent_node

        current_parent_node = current_parent_node.parent
    return None


def crate_module_path(node: Node, escaped: bool = False) -> list[str]:
    """Get a list of the nested modules (under crate::components) under
    which this node's type is defined."""
    parent = parent_scope(node)
    assert parent is not None
    module_name = rust_module_name(node)
    if escaped:
        module_name = kw_filter(module_name)
    if isinstance(parent, RootNode):
        return [module_name]
    parent_path = crate_module_path(parent)
    if is_anonymous(node):
        return parent_path + [module_name]
    else:
        return parent_path + ["named_types", module_name]


def crate_enum_module_path(field: FieldNode, enum: type[UserEnum]) -> list[str]:
    """Get a list of the nested modules (under crate::components) under
    which this field's enum type is defined."""
    assert field.get_property("encode") is enum
    declaring_parent = enum_parent_scope(field, enum)
    assert declaring_parent is not None

    module_name = snakecase(enum.type_name)

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


def field_access(node: Union[FieldNode, MemNode]) -> Union[str, None]:
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
    is_signed = node.get_property("is_signed")
    if node.width == 1 and is_signed is None and allow_bool:
        return "bool"
    signedness = "i" if is_signed else "u"
    for w in (8, 16, 32, 64, 128):
        if w >= node.width:
            return f"{signedness}{w}"
    raise RuntimeError("Field widths > 128 are not supported")


def field_reset_value(field: FieldNode) -> int:
    reset = field.get_property("reset", default=0)
    if isinstance(reset, int):
        return reset
    elif isinstance(reset, FieldNode):
        return field_reset_value(reset)
    elif isinstance(reset, PropertyReference):
        reset = reset.node.get_property(reset.name)
        if isinstance(reset, int):
            return reset
        else:
            print(
                "Warning: could not determine reset value for field "
                f"{field.get_path()}. Defaulting to 0"
            )
            return 0
    elif isinstance(reset, SignalNode):
        print(
            f"Warning: reset value for {field.get_path()} is driven by a hardware "
            "signal. Defaulting to 0"
        )
        return 0
    else:
        return 0


def append_unique(list: list, obj: Any) -> None:
    """Append an object to a list only if it's not already present"""
    if obj not in list:
        list.append(obj)


def dut_access_method(node: AddressableNode) -> str:
    """Get register access method, e.g. 'grammeter()[1].status()'"""
    segments = node.get_path_segments()[1:]
    called_segments = []
    for seg in segments:
        idx = seg.find("[")
        if idx == -1:
            # not an array
            called_segments.append(kw_filter(snakecase(seg)) + "()")
        else:
            name = kw_filter(snakecase(seg[:idx]))
            called_segments.append(name + "()" + seg[idx:])
    return ".".join(called_segments)
