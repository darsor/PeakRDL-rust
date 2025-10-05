from typing import TYPE_CHECKING

from .fixedpoint import FracWidth, IntWidth
from .signed import IsSigned

if TYPE_CHECKING:
    from systemrdl.udp import UDPDefinition

ALL_UDPS: list[type[UDPDefinition]] = [
    IntWidth,
    FracWidth,
    IsSigned,
]
