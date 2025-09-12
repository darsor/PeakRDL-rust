from typing import TYPE_CHECKING, Union

# from peakrdl.config import schema
from peakrdl.plugins.exporter import ExporterSubcommandPlugin

from .exporter import RustExporter

if TYPE_CHECKING:
    import argparse

    from systemrdl.node import AddrmapNode


class Exporter(ExporterSubcommandPlugin):
    short_desc = "Generate a Rust crate for accessing SystemRDL registers"

    # cfg_schema = {
    #     "std": schema.Choice(list(CStandard.__members__.keys())),
    #     "type_style": schema.Choice(["lexical", "hier"]),
    #     "subword_size": schema.Integer(),
    #     "bitfields": schema.Choice(["ltoh", "htol", "none"]),
    # }

    def add_exporter_arguments(self, arg_group: "argparse._ActionsContainer") -> None:
        arg_group.add_argument(
            "--force",
            action="store_true",
            default=False,
            help="""
            Overwrite the output directory if it already exists.
            """,
        )

        arg_group.add_argument(
            "-i",
            "--instantiate",
            action="store_true",
            default=False,
            help="""
            If set, header will also include a macro that instantiates each top-level
            block at a defined hardware address, allowing for direct access.
            """,
        )

        # Wrap constructor to allow hex strings
        def integer(n: Union[int, str]) -> int:
            return int(n, 0)  # type: ignore # bogus error

        arg_group.add_argument(
            "--inst-offset",
            type=integer,
            default=0,
            help="""
            Apply an additional address offset to instance definitions.
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

    def do_export(self, top_node: "AddrmapNode", options: "argparse.Namespace") -> None:
        x = RustExporter()
        x.export(
            top_node,
            path=options.output,
            force=options.force,
            instantiate=options.instantiate,
            inst_offset=options.inst_offset,
            no_fmt=options.no_fmt,
        )
