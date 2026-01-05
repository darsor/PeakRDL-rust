from dataclasses import dataclass
from typing import Optional, Union

from caseconverter import snakecase
from systemrdl.node import (
    AddressableNode,
    AddrmapNode,
    RegfileNode,
)
from systemrdl.walker import RDLListener, RDLWalker, WalkerAction

from . import utils
from .design_state import DesignState


@dataclass
class TestAddress:
    dut_method: str  # method chain to call on DUT to access component
    absolute_addr: int


@dataclass
class TestComponent:
    """Top-level component information for test generation"""

    crate_name: str
    name: str  # component instance name
    type_name: str  # component type name
    addresses: list[TestAddress]


class TestScanner(RDLListener):
    def __init__(self, top_node: Union[AddrmapNode, RegfileNode]) -> None:
        self.top_node = top_node
        self.test_addrs: list[TestAddress] = []

    def run(self) -> None:
        RDLWalker(unroll=True).walk(self.top_node, self)

    def enter_AddressableComponent(
        self, node: AddressableNode
    ) -> Optional[WalkerAction]:
        if node is self.top_node:
            return WalkerAction.Continue

        self.test_addrs.append(
            TestAddress(
                dut_method=utils.dut_access_method(node),
                absolute_addr=node.absolute_address,
            )
        )

        return WalkerAction.Continue


def write_tests(ds: DesignState) -> None:
    """Generate test files for the top-level components"""
    tests_dir = ds.output_dir / "tests"
    tests_dir.mkdir(exist_ok=True)

    for top in ds.top_nodes:
        scanner = TestScanner(top)
        scanner.run()

        component = TestComponent(
            crate_name=ds.crate_name,
            name=snakecase(top.inst_name),
            type_name=utils.rust_type_name(top),
            addresses=scanner.test_addrs,
        )

        # Write the test file
        test_file_path = tests_dir / f"test_{component.name}.rs"
        with test_file_path.open("w") as f:
            template = ds.jj_env.get_template("tests/test_addrmap.rs")
            template.stream(ctx=component).dump(f)  # type: ignore # jinja incorrectly typed
