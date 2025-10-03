from systemrdl.udp import UDPDefinition

from .fixedpoint import FracWidth, IntWidth
from .signed import IsSigned

ALL_UDPS: list[type[UDPDefinition]] = [
    IntWidth,
    FracWidth,
    IsSigned,
]
