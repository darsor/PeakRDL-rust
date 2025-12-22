from pathlib import Path
from typing import TYPE_CHECKING, Any, Literal, Optional

import jinja2 as jj
from caseconverter import snakecase
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

        self.crate_name: str
        top_name = top_nodes[-1].orig_type_name or top_nodes[-1].type_name
        assert top_name is not None
        default_crate_name = snakecase(top_name)
        self.crate_name = kwargs.pop("crate_name", None) or default_crate_name
        self.crate_name = self.crate_name.replace("-", "_")
        self.output_dir = output_dir / self.crate_name

        self.crate_version: str
        self.crate_version = kwargs.pop("crate_version", "0.1.0")

        self.no_fmt: bool
        self.no_fmt = kwargs.pop("no_fmt", False)

        self.byte_endian: Optional[Literal["big", "little"]]
        self.byte_endian = kwargs.pop("byte_endian", None)

        self.word_endian: Optional[Literal["big", "little"]]
        self.word_endian = kwargs.pop("word_endian", None)

        # ------------------------
        # Collect info for export
        # ------------------------
        scanner = DesignScanner(self.top_nodes)
        scanner.run()
        self.has_fixedpoint: bool = scanner.has_fixedpoint

        component_context = ContextScanner(self.top_nodes)
        component_context.run()
        self.top_component_modules: list[str] = component_context.top_component_modules
        self.components: dict[Path, Component] = component_context.components
