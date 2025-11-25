import shutil

from caseconverter.pascal import pascalcase

from . import utils
from .design_state import DesignState


def write_crate(ds: DesignState) -> None:
    # Cargo.toml
    cargo_toml_path = ds.output_dir / "Cargo.toml"
    cargo_toml_path.parent.mkdir(parents=True, exist_ok=True)
    context = {
        "has_fixedpoint": ds.has_fixedpoint,
        "package_name": ds.crate_name,
        "package_version": ds.crate_version,
    }
    with cargo_toml_path.open("w") as f:
        template = ds.jj_env.get_template("Cargo.toml.tmpl")
        template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed

    # .gitignore
    shutil.copyfile(ds.template_dir / ".gitignore", ds.output_dir / ".gitignore")

    # src/access.rs
    access_rs_path = ds.output_dir / "src" / "access.rs"
    access_rs_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ds.template_dir / "src" / "access.rs", access_rs_path)

    # src/encode.rs
    encode_rs_path = ds.output_dir / "src" / "encode.rs"
    encode_rs_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ds.template_dir / "src" / "encode.rs", encode_rs_path)

    # src/mem.rs
    mem_rs_path = ds.output_dir / "src" / "mem.rs"
    mem_rs_path.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(ds.template_dir / "src" / "mem.rs", mem_rs_path)

    # src/reg.rs
    reg_rs_path = ds.output_dir / "src" / "reg.rs"
    reg_rs_path.parent.mkdir(parents=True, exist_ok=True)
    context = {
        "endianness": "be" if ds.top_nodes[0].get_property("bigendian") else "le",
    }
    with reg_rs_path.open("w") as f:
        template = ds.jj_env.get_template("src/reg.rs")
        template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed

    # src/fixedpoint.rs
    if ds.has_fixedpoint:
        fixedpoint_rs_path = ds.output_dir / "src" / "fixedpoint.rs"
        fixedpoint_rs_path.parent.mkdir(parents=True, exist_ok=True)
        context = {
            "crate_name": ds.crate_name,
        }
        with fixedpoint_rs_path.open("w") as f:
            template = ds.jj_env.get_template("src/fixedpoint.rs")
            template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed

    # src/lib.rs
    lib_rs_path = ds.output_dir / "src" / "lib.rs"
    lib_rs_path.parent.mkdir(parents=True, exist_ok=True)
    context = {
        "has_fixedpoint": ds.has_fixedpoint,
        "top_nodes": [
            "::".join(
                ["crate", "components"]
                + utils.crate_module_path(node, escaped=True)
                + [pascalcase(utils.rust_type_name(node))]
            )
            for node in ds.top_nodes
        ],
    }
    with lib_rs_path.open("w") as f:
        template = ds.jj_env.get_template("src/lib.rs")
        template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed

    # src/components.rs
    components_rs_path = ds.output_dir / "src" / "components.rs"
    components_rs_path.parent.mkdir(parents=True, exist_ok=True)
    context = {
        "components": ds.top_component_modules,
    }
    with components_rs_path.open("w") as f:
        template = ds.jj_env.get_template("src/components.rs")
        template.stream(ctx=context).dump(f)  # type: ignore # jinja incorrectly typed

    for comp in ds.components.values():
        comp.render(ds.output_dir, ds.jj_env)
