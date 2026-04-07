import shutil
import subprocess
from typing import Any, Union

from systemrdl.node import AddrmapNode, RootNode

from .design_state import DesignState
from .generator import write_module


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
        fmt: bool
            Attempt to format the generated rust code using `rustfmt`.
        byte_endian: Optional[Literal["big", "little"]]
            Ordering of bytes within `accesswidth`-sized accesses to the register
            file. Overrides the `littleendian` and `bigendian` addrmap properties.
        word_endian: Optional[Literal["big", "little"]]
            Ordering of `accesswidth`-sized words within a wide register. Overrides
            the `littleendian` and `bigendian` addrmap properties.
        access_mode: str
            Access mode for register/field access functions. One of:
            - `software` (default): Use software read/write access properties
            - `hardware`: Use hardware read/write access properties
        read_only: bool
            Treat all registers and fields as read-only. Write-only registers and
            fields are not exposed.
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

        # Write module files
        generated_files = write_module(ds)

        print(f"Generated Rust module at {ds.output_dir / 'mod.rs'}")

        if ds.fmt:
            cmd = ["rustfmt"] + list(map(str, generated_files))
            subprocess.check_call(cmd)
