import os
from pathlib import Path
from typing import TYPE_CHECKING, Any, Dict, List

import jinja2 as jj
from systemrdl.node import AddrmapNode

if TYPE_CHECKING:
    from peakrdl_rust.crate_generator import Component


class DesignState:
    def __init__(self, top_node: AddrmapNode, path: str, kwargs: Any) -> None:
        loader = jj.FileSystemLoader(
            os.path.join(os.path.dirname(__file__), "templates")
        )
        self.jj_env = jj.Environment(
            loader=loader,
            undefined=jj.StrictUndefined,
            trim_blocks=True,
            lstrip_blocks=True,
        )

        self.top_node = top_node
        self.output_dir = Path(path)
        self.template_dir = Path(__file__).parent / "templates"

        # ------------------------
        # Info about the design
        # ------------------------
        self.top_component_modules: List[str] = []
        self.components: Dict[Path, Component] = {}

        # Each reg that has overlapping fields generates an entry:
        #   reg_path : list of field names involved in overlap
        self.overlapping_fields: Dict[str, List[str]] = {}

        # Pairs of overlapping registers
        #   first_reg_path : partner_register_name
        self.overlapping_reg_pairs: Dict[str, str] = {}

        # ------------------------
        # Extract compiler args
        # ------------------------
        self.force: bool
        self.force = kwargs.pop("force", False)

        self.explode_top: bool
        self.explode_top = kwargs.pop("explode_top", False)

        self.instantiate: bool
        self.instantiate = kwargs.pop("instantiate", False)

        self.inst_offset: int
        self.inst_offset = kwargs.pop("inst_offset", 0)

        self.no_fmt: bool
        self.no_fmt = kwargs.pop("no_fmt", False)
