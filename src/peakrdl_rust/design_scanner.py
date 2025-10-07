from typing import Optional

from systemrdl.node import (
    AddrmapNode,
    FieldNode,
)
from systemrdl.walker import RDLListener, RDLWalker, WalkerAction


class DesignScanner(RDLListener):
    def __init__(self, top_nodes: list[AddrmapNode]) -> None:
        self.top_nodes = top_nodes
        self.has_fixedpoint = False

    def run(self) -> None:
        for node in self.top_nodes:
            RDLWalker(unroll=False).walk(node, self)
            if self.has_fixedpoint:
                break

    def enter_Field(self, node: FieldNode) -> Optional[WalkerAction]:
        if node.get_property("fracwidth") is not None:
            self.has_fixedpoint = True
            return WalkerAction.StopNow
        return WalkerAction.Continue
