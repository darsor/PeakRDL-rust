import shutil
import subprocess
from typing import Any, List, Union

from systemrdl.node import AddrmapNode, MemNode, RegfileNode, RootNode

from .crate_generator import write_crate
from .design_scanner import DesignScanner
from .design_state import DesignState
from .test_generator import write_tests


class RustExporter:
    def export(
        self,
        node: Union[RootNode, AddrmapNode, List[AddrmapNode]],
        path: str,
        **kwargs: Any,
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
            top_nodes = [node.top]
        elif isinstance(node, AddrmapNode):
            top_nodes = [node]
        else:
            top_nodes = node

        ds = DesignState(top_nodes, path, kwargs)

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

        if ds.output_dir.exists() and (
            not ds.output_dir.is_dir() or any(ds.output_dir.iterdir())
        ):
            # Remove the existing output directory
            if ds.output_dir.is_dir():
                shutil.rmtree(ds.output_dir)
            else:
                ds.output_dir.unlink()

        # Write crate modules
        write_crate(ds)

        # Generate integration tests
        write_tests(ds)

        if not ds.no_fmt:
            result = subprocess.run(["cargo", "fmt"], cwd=ds.output_dir)
            if result.returncode == 127:
                print(
                    "Warning: failed to run `cargo fmt`. Install cargo "
                    "(https://rustup.rs/) or silence this warning with `--no-fmt`"
                )
            elif result.returncode != 0:
                print(
                    "Failed to format files. Please submit an bug report: https://github.com/darsor/PeakRDL-rust/issues"
                )
