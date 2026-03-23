from pathlib import Path
from typing import TYPE_CHECKING, Any, Literal

import jinja2 as jj
from systemrdl.node import AddrmapNode

from .component_context import ContextScanner
from .design_scanner import DesignScanner
from .utils import kw_filter

if TYPE_CHECKING:
    from peakrdl_rust.component_context import Component


class DesignState:
    def __init__(self, top_nodes: list[AddrmapNode], path: str, kwargs: Any) -> None:
        loader = jj.FileSystemLoader(Path(__file__).resolve().parent / "templates")
        self.jj_env = jj.Environment(
            loader=loader,
            undefined=jj.StrictUndefined,
            trim_blocks=True,
            lstrip_blocks=True,
        )
        self.jj_env.filters["kw_filter"] = kw_filter

        self.top_nodes = top_nodes
        output_dir = Path(path).resolve()
        self.template_dir = Path(__file__).resolve().parent / "templates"

        # ------------------------
        # Extract compiler args
        # ------------------------
        self.force: bool
        self.force = kwargs.pop("force", False)

        self.output_dir = output_dir

        self.fmt: bool
        self.fmt = kwargs.pop("fmt", False)

        if self.top_nodes[0].get_property("bigendian", default=False):
            default_endian = "big"
        else:
            default_endian = "little"
        byte_endian = kwargs.pop("byte_endian", None) or default_endian
        word_endian = kwargs.pop("word_endian", None) or default_endian
        self.byte_endian: Literal["Big", "Little"] = byte_endian.capitalize()  # type: ignore
        self.word_endian: Literal["Big", "Little"] = word_endian.capitalize()  # type: ignore

        self.access_mode: str
        self.access_mode = kwargs.pop("access_mode", "software")
        if self.access_mode not in ("software", "hardware"):
            raise ValueError(
                f"Invalid access_mode '{self.access_mode}'. "
                "Must be one of: 'software', 'hardware'"
            )

        self.read_only: bool
        self.read_only = kwargs.pop("fmt", False)

        # ------------------------
        # Collect info for export
        # ------------------------
        scanner = DesignScanner(self.top_nodes)
        scanner.run()
        self.has_fixedpoint: bool = scanner.has_fixedpoint

        component_context = ContextScanner(
            self.top_nodes, self.byte_endian, self.word_endian, self.access_mode
        )
        component_context.run()
        self.top_component_modules: list[str] = component_context.top_component_modules
        self.components: dict[Path, Component] = component_context.components
