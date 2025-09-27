from pathlib import Path
from typing import TYPE_CHECKING, Any

import jinja2 as jj
from systemrdl.node import AddrmapNode

if TYPE_CHECKING:
    from peakrdl_rust.crate_generator import Component


class DesignState:
    def __init__(self, top_nodes: list[AddrmapNode], path: str, kwargs: Any) -> None:
        loader = jj.FileSystemLoader(Path(__file__).resolve().parent / "templates")
        self.jj_env = jj.Environment(
            loader=loader,
            undefined=jj.StrictUndefined,
            trim_blocks=True,
            lstrip_blocks=True,
        )

        self.top_nodes = top_nodes
        self.output_dir = Path(path).resolve()
        self.template_dir = Path(__file__).resolve().parent / "templates"

        # ------------------------
        # Info about the design
        # ------------------------
        self.top_component_modules: list[str] = []
        self.components: dict[Path, Component] = {}

        # Each reg that has overlapping fields generates an entry:
        #   reg_path : list of field names involved in overlap
        self.overlapping_fields: dict[str, list[str]] = {}

        # Pairs of overlapping registers
        #   first_reg_path : partner_register_name
        self.overlapping_reg_pairs: dict[str, str] = {}

        # ------------------------
        # Extract compiler args
        # ------------------------
        self.force: bool
        self.force = kwargs.pop("force", False)

        self.instantiate: bool
        self.instantiate = kwargs.pop("instantiate", False)

        self.inst_offset: int
        self.inst_offset = kwargs.pop("inst_offset", 0)

        self.no_fmt: bool
        self.no_fmt = kwargs.pop("no_fmt", False)
