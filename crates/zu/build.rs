// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

#[path = "consts/theme.rs"]
mod theme;

#[path = "consts/svg_icons.rs"]
mod svg_icons;

use inflections::Inflect;
use rsass::{compile_scss_path, output};
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use zu_util::icon::need_update_with_name;

fn merge_themes(style_files: &[&str], output_name: &Path) -> io::Result<()> {
    let mut output_file = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output_name)?;

    for file in style_files {
        let content = fs::read_to_string(file)?;
        output_file.write_all(content.as_bytes())?;
        output_file.write_all(b"\n\n")?;
    }

    Ok(())
}

fn compile_scss(input_name: &Path, output_name: &Path) -> Result<(), Box<dyn Error>> {
    let format = output::Format {
        style: output::Style::Expanded,
        ..Default::default()
    };
    let css = compile_scss_path(input_name, format)?;

    let mut output_file = fs::OpenOptions::new()
        .truncate(true)
        .create(true)
        .write(true)
        .open(output_name)?;
    let css: String = String::from_utf8(css).unwrap();
    // NOTE(Shaohua): Remove @charset, as it is not supported by stylist yet.
    let css = css.replace("@charset \"UTF-8\";", "");
    output_file.write_all(css.as_bytes())?;

    Ok(())
}

fn generate_style_files() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut dark_files = theme::COLORS.to_vec();
    dark_files.extend_from_slice(theme::DARK_COLORS);
    dark_files.extend_from_slice(theme::COMMON_STYLES);
    let dark_theme_scss = Path::new(&out_dir).join("dark_theme.scss");
    let dark_theme_css = Path::new(&out_dir).join("dark_theme.css");
    merge_themes(&dark_files, &dark_theme_scss)?;
    compile_scss(&dark_theme_scss, &dark_theme_css)?;
    println!(
        "cargo:rustc-env=DARK_THEME_CSS={}",
        dark_theme_css.display()
    );

    let mut light_files = theme::COLORS.to_vec();
    light_files.extend_from_slice(theme::LIGHT_COLORS);
    light_files.extend_from_slice(theme::COMMON_STYLES);
    let light_theme_scss = Path::new(&out_dir).join("light_theme.scss");
    let light_theme_css = Path::new(&out_dir).join("light_theme.css");
    merge_themes(&light_files, &light_theme_scss)?;
    compile_scss(&light_theme_scss, &light_theme_css)?;
    println!(
        "cargo:rustc-env=LIGHT_THEME_CSS={}",
        light_theme_css.display()
    );

    let color_scheme_scss = Path::new(&out_dir).join("color_scheme.scss");
    merge_themes(theme::COLORS, &color_scheme_scss)?;

    Ok(())
}

fn generate_svg_icons() -> Result<(), Box<dyn Error>> {
    const TEMPLATE_FILE: &str = include_str!("src/internal/svg_icons/template.rs");

    let mut mod_file = OpenOptions::new()
        .append(true)
        .open("src/internal/svg_icons.rs")?;
    for (node_name, path_data) in svg_icons::SVG_ICONS {
        let module_name = node_name.to_snake_case();
        println!("module name: {module_name}");
        let mut rs_filepath = PathBuf::new();
        rs_filepath.push("src/internal/svg_icons");
        rs_filepath.push(&module_name);
        rs_filepath.set_extension("rs");

        let rs_content = TEMPLATE_FILE
            .replace("{MODULE_NAME}", &module_name)
            .replace("{NODE_NAME}", node_name)
            .replace("{ICON_NAME}", node_name)
            .replace("{PATH_DATA}", path_data);

        fs::write(rs_filepath, rs_content).unwrap();

        let line = format!(
            r#"mod {module_name};
pub use {module_name}::{node_name};

"#
        );
        mod_file.write_all(line.as_bytes())?;
    }

    drop(mod_file);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    generate_style_files()?;

    // Check ZU_ICON_UPDATE=zu environment.
    if need_update_with_name("zu") {
        generate_svg_icons()?;
    }

    Ok(())
}
