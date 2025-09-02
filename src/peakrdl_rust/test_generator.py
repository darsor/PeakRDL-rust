import shutil
from dataclasses import dataclass
from typing import List, Optional, Union

from caseconverter import pascalcase, snakecase
from systemrdl.node import (
    AddrmapNode,
    FieldNode,
    RegfileNode,
)
from systemrdl.walker import RDLListener, RDLWalker, WalkerAction

from . import utils
from .design_state import DesignState
from .identifier_filter import kw_filter


@dataclass
class TestPattern:
    """Test pattern for a field"""

    description: str  # human-readable description of the test pattern
    value: str  # value to use in Rust code (e.g. "true", "0x42", "SomeEnum::Variant")
    raw_value: int  # the raw bit pattern value for memory access
    raw_write: bool = False  # write the memory directly, bypassing the register
    is_valid_enum: bool = True  # whether this is a valid enum variant


@dataclass
class TestField:
    """Field information for test generation"""

    name: str
    reg_method: str  # method chain to call on DUT to access parent register
    has_encoding: bool
    primitive: str  # rust primitive type (u8, u16, etc)
    address: int  # byte address
    bit_offset: int  # low bit within register
    width: int  # bit width
    is_readable: bool
    test_patterns: List[TestPattern]


@dataclass
class TestComponent:
    """Top-level component information for test generation"""

    name: str  # component instance name
    type_name: str  # component type name
    fields: List[TestField]


def generate_test_patterns(field: FieldNode) -> List[TestPattern]:
    patterns: List[TestPattern] = []

    encoding = field.get_property("encode")

    if encoding is not None:
        enum_type_name = pascalcase(encoding.type_name)
        enum_scope = field.get_path(hier_separator="::", array_suffix="")
        # Field uses an enum - test all valid variants
        for variant in encoding.members.values():
            patterns.append(
                TestPattern(
                    description=f"enum variant {variant.name}",
                    value=f"{enum_scope}::{enum_type_name}::{pascalcase(variant.name)}",
                    raw_value=variant.value,
                    raw_write=not field.is_sw_writable,
                    is_valid_enum=True,
                )
            )

        # Add one invalid enum pattern if field is readable (to test None return)
        if field.is_sw_readable:
            valid_values = set(variant.value for variant in encoding.members.values())
            invalid_variant_value = next(
                (i for i in range(2**field.width) if i not in valid_values), None
            )
            if invalid_variant_value is not None:
                patterns.append(
                    TestPattern(
                        description="invalid enum value",
                        value=str(invalid_variant_value),
                        raw_value=invalid_variant_value,
                        raw_write=True,
                        is_valid_enum=False,
                    )
                )
    else:
        # Field is boolean, test true and false
        primitive = utils.field_primitive(field)
        if primitive == "bool":
            patterns.extend(
                [
                    TestPattern(
                        description="false",
                        value="false",
                        raw_value=0,
                        raw_write=not field.is_sw_writable,
                    ),
                    TestPattern(
                        description="true",
                        value="true",
                        raw_value=1,
                        raw_write=not field.is_sw_writable,
                    ),
                ]
            )
        else:
            # For integer fields, test all zeros and all ones within the field width
            max_value = 2**field.width - 1
            patterns.extend(
                [
                    TestPattern(
                        description="all zeros",
                        value="0",
                        raw_value=0,
                        raw_write=not field.is_sw_writable,
                    ),
                    TestPattern(
                        description="all ones",
                        value=str(max_value),
                        raw_value=max_value,
                        raw_write=not field.is_sw_writable,
                    ),
                ]
            )

    return patterns


class TestScanner(RDLListener):
    def __init__(self, top_node: Union[AddrmapNode, RegfileNode]) -> None:
        self.top_node = top_node
        self.test_fields: List[TestField] = []

    def run(self) -> None:
        RDLWalker(unroll=True).walk(self.top_node, self)

    def enter_Field(self, node: FieldNode) -> Optional[WalkerAction]:
        self.test_fields.append(
            TestField(
                name=kw_filter(snakecase(node.inst_name)),
                reg_method=utils.reg_access_method(node.parent),
                has_encoding=node.get_property("encode") is not None,
                primitive=utils.field_primitive(node, allow_bool=False),
                address=node.parent.absolute_address,
                bit_offset=node.low,
                width=node.width,
                is_readable=node.is_sw_readable,
                test_patterns=generate_test_patterns(node),
            )
        )
        return WalkerAction.Continue


def write_tests(top_nodes: List[Union[AddrmapNode, RegfileNode]], ds: DesignState):
    """Generate test files for the top-level components"""
    tests_dir = ds.output_dir / "tests"

    # tests/memory/mod.rs
    (tests_dir / "memory").mkdir(parents=True)
    shutil.copyfile(
        ds.template_dir / "tests" / "memory" / "mod.rs",
        tests_dir / "memory" / "mod.rs",
    )

    for top in top_nodes:
        scanner = TestScanner(top)
        scanner.run()

        component = TestComponent(
            name=kw_filter(snakecase(top.inst_name)),
            type_name=pascalcase(top.type_name),
            fields=scanner.test_fields,
        )

        # Write the test file
        test_file_path = tests_dir / f"test_{component.name}.rs"
        with test_file_path.open("w") as f:
            template = ds.jj_env.get_template("tests/test_addrmap.rs")
            template.stream(ctx=component).dump(f)  # type: ignore # jinja incorrectly typed
