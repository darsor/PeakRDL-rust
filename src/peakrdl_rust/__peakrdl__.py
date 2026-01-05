from typing import TYPE_CHECKING

from peakrdl.config import schema
from peakrdl.plugins.exporter import ExporterSubcommandPlugin

from .exporter import RustExporter
from .udps import ALL_UDPS

if TYPE_CHECKING:
    import argparse

    from systemrdl.node import AddrmapNode


class Exporter(ExporterSubcommandPlugin):
    short_desc = "Generate a Rust crate for accessing SystemRDL registers"

    udp_definitions = ALL_UDPS

    cfg_schema = {
        "crate_name": schema.String(),
        "crate_version": schema.String(),
        "force": schema.Boolean(),
        "no_fmt": schema.Boolean(),
        "byte_endian": schema.Choice(["big", "little"]),
        "word_endian": schema.Choice(["big", "little"]),
    }

    def add_exporter_arguments(self, arg_group: "argparse._ActionsContainer") -> None:
        arg_group.add_argument(
            "-n",
            "--crate-name",
            help="""
            Name of the generated crate. Should be snake_case. Derived from the root
            addrmap if not given. The rust crate is generated under a directory of
            this name.
            """,
        )

        arg_group.add_argument(
            "-v",
            "--crate-version",
            default="0.1.0",
            help="""
            Semantic version of the generated crate. Default is "0.1.0".
            """,
        )

        arg_group.add_argument(
            "--force",
            action="store_true",
            default=False,
            help="""
            Overwrite the output directory if it already exists.
            """,
        )

        arg_group.add_argument(
            "--no-fmt",
            action="store_true",
            default=False,
            help="""
            Don't attempt to format the generated rust code using `cargo fmt`.
            """,
        )

        arg_group.add_argument(
            "--byte-endian",
            choices=["big", "little"],
            default=None,
            help="""
            Ordering of bytes within `accesswidth`-sized accesses to the register
            file. Overrides the `littleendian` and `bigendian` addrmap properties.
            """,
        )

        arg_group.add_argument(
            "--word-endian",
            choices=["big", "little"],
            default=None,
            help="""
            Ordering of `accesswidth`-sized words within a wide register. Overrides
            the `littleendian` and `bigendian` addrmap properties.
            """,
        )

    def do_export(self, top_node: "AddrmapNode", options: "argparse.Namespace") -> None:
        x = RustExporter()
        x.export(
            top_node,
            path=options.output,
            force=options.force,
            crate_name=options.crate_name,
            crate_version=options.crate_version,
            no_fmt=options.no_fmt,
        )
