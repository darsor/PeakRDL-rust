import shutil
import subprocess
from typing import Any, Union

from systemrdl.node import AddrmapNode, RootNode

from .crate_generator import write_crate
from .design_state import DesignState
from .test_generator import write_tests


class RustExporter:
    def export(
        self,
        node: Union[RootNode, AddrmapNode, list[AddrmapNode]],
        path: str,
        **kwargs: Any,
    ) -> None:
        """
        Parameters
        ----------
        node: AddrmapNode
            Top-level SystemRDL node(s) to export.
        path: str
            Output directory for generated crate. A subfolder with the name of
            the crate is generated in this directory.
        force: bool
            Overwrite the contents of the output directory if it already exists.
        crate_name: Optional[str]
            Name of the generated crate. Should be in snake_case. If not set, derived
            from the root addrmap name.
        crate_version: str
            Semantic version number for the generated crate.
        no_fmt: bool
            Don't attempt to format the generated rust code using `cargo fmt`.
        byte_endian: Optional[Literal["big", "little"]]
            Ordering of bytes within `accesswidth`-sized accesses to the register
            file. Overrides the `littleendian` and `bigendian` addrmap properties.
        word_endian: Optional[Literal["big", "little"]]
            Ordering of `accesswidth`-sized words within a wide register. Overrides
            the `littleendian` and `bigendian` addrmap properties.
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

        print(f"Generated Rust crate at {ds.output_dir}")

        if not ds.no_fmt:
            result = subprocess.run(["cargo", "fmt"], cwd=ds.output_dir)
            if result.returncode == 127:
                print(
                    "Warning: failed to run `cargo fmt`. Install cargo "
                    "(https://rustup.rs/) or silence this warning with `--no-fmt`"
                )
            elif result.returncode != 0:
                print("Failed to format files. Silence this warning with '--no-fmt'.")
