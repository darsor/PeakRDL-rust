from . import utils
from .design_state import DesignState


def write_crate(ds: DesignState) -> None:
    # mod.rs
    mod_rs_path = ds.output_dir / "mod.rs"
    mod_rs_path.parent.mkdir(parents=True, exist_ok=True)
    context = {
        "top_nodes": [
            "::".join(
                ["components"]
                + utils.crate_module_path(node, escaped=True)
                + [utils.rust_type_name(node)]
            )
            for node in ds.top_nodes
        ],
    }
    with mod_rs_path.open("w") as f:
        template = ds.jj_env.get_template("mod.rs")
        template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed

    # components.rs
    components_rs_path = ds.output_dir / "components.rs"
    components_rs_path.parent.mkdir(parents=True, exist_ok=True)
    context = {
        "components": ds.top_component_modules,
    }
    with components_rs_path.open("w") as f:
        template = ds.jj_env.get_template("components.rs")
        template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed

    for comp in ds.components.values():
        comp.render(ds.output_dir, ds.jj_env)
