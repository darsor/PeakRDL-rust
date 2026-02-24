from importlib.metadata import version
from pathlib import Path

from . import PEAKRDL_RUST_CRATE_MIN_VERSION, utils
from .design_state import DesignState


def write_module(ds: DesignState) -> list[Path]:
    generated_files = []

    # mod.rs
    mod_rs_path = ds.output_dir / "mod.rs"
    mod_rs_path.parent.mkdir(parents=True, exist_ok=True)
    if PEAKRDL_RUST_CRATE_MIN_VERSION[0] == 0:
        crate_max_version = (0, PEAKRDL_RUST_CRATE_MIN_VERSION[1] + 1, 0)
    else:
        crate_max_version = (PEAKRDL_RUST_CRATE_MIN_VERSION[0] + 1, 0, 0)
    context = {
        "top_nodes": [
            "::".join(
                ["components"]
                + utils.crate_module_path(node, escaped=True)
                + [utils.rust_type_name(node)]
            )
            for node in ds.top_nodes
        ],
        "peakrdl_rust_version": version("peakrdl-rust"),
        "crate_min_version": PEAKRDL_RUST_CRATE_MIN_VERSION,
        "crate_max_version": crate_max_version,
    }
    with mod_rs_path.open("w") as f:
        template = ds.jj_env.get_template("mod.rs")
        template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed
    generated_files.append(mod_rs_path)

    # components.rs
    components_rs_path = ds.output_dir / "components.rs"
    components_rs_path.parent.mkdir(parents=True, exist_ok=True)
    context = {
        "components": ds.top_component_modules,
    }
    with components_rs_path.open("w") as f:
        template = ds.jj_env.get_template("components.rs")
        template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed
    generated_files.append(components_rs_path)

    for path, comp in ds.components.items():
        comp.render(ds.output_dir, ds.jj_env)
        generated_files.append(ds.output_dir / path)

    return generated_files
