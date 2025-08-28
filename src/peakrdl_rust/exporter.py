import shutil
import subprocess
from pathlib import Path
from typing import Any, Union

from systemrdl.node import AddrmapNode, MemNode, RegfileNode, RootNode

from .crate_generator import write_crate
from .design_scanner import DesignScanner
from .design_state import DesignState

# from .design_scanner import DesignScanner


class RustExporter:
    def export(
        self, node: Union[RootNode, AddrmapNode], path: str, **kwargs: Any
    ) -> None:
        """
        Parameters
        ----------
        node: AddrmapNode
            Top-level SystemRDL node to export.
        path: str
            Output directory for generated crate.
        force: bool
            Overwrite the contents of the output directory if it already exists.
        explode_top: bool
            If set, the top-level hiearchy is skipped. Instead, definitions for
            all the direct children are generated.

            Note that only block-like definitons are generated.
            i.e: children that are registers are skipped.
        instantiate: bool
            If set, header will also include a macro that instantiates each top-level
            block at a defined hardware address, allowing for direct access.
        inst_offset: int
            Apply an additional address offset to instance definitions.
        no_fmt: bool
            Don't attempt to format the generated rust code using `cargo fmt`.
        """
        # If it is the root node, skip to top addrmap
        if isinstance(node, RootNode):
            top_node = node.top
        else:
            top_node = node

        ds = DesignState(top_node, path, kwargs)

        # Check for stray kwargs
        if kwargs:
            raise TypeError(
                f"got an unexpected keyword argument '{list(kwargs.keys())[0]}'"
            )

        # Check if the output already exists
        if (
            ds.output_dir.exists()
            and (not ds.output_dir.is_dir() or any(ds.output_dir.iterdir()))
            and not ds.force
        ):
            raise FileExistsError(
                f"'{ds.output_dir}' already exists (use --force to overwrite)"
            )

        # Collect info for export
        DesignScanner(ds).run()

        top_nodes = []
        if ds.explode_top:
            for child in top_node.children():
                if isinstance(child, (AddrmapNode, MemNode, RegfileNode)):
                    top_nodes.append(child)
        else:
            top_nodes.append(top_node)

        if ds.output_dir.exists() and (
            not ds.output_dir.is_dir() or any(ds.output_dir.iterdir())
        ):
            # Remove the existing output directory
            if ds.output_dir.is_dir():
                shutil.rmtree(ds.output_dir)
            else:
                ds.output_dir.unlink()

        # Write output
        write_crate(top_nodes, ds)
        # HeaderGenerator(ds).run(path, top_nodes)
        # if ds.testcase:
        #     TestcaseGenerator(ds).run(path, top_nodes)

        if not ds.no_fmt:
            result = subprocess.run(["cargo", "fmt"], cwd=ds.output_dir)
            if result.returncode != 0:
                print(
                    "Warning: failed to run `cargo fmt`. Install cargo "
                    "(https://rustup.rs/) or silence this warning with `--no-fmt`"
                )
