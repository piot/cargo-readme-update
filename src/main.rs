/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/cargo-readme-update
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */

use cargo_metadata::MetadataCommand;
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to get metadata");

    let package = metadata
        .root_package()
        .expect("Failed to find root package");
    let package_name = &package.name;
    let package_version = &package.version;

    let readme_content = fs::read_to_string("README.md")?;

    let re = Regex::new(r"```toml\s*\[dependencies\][^`]*```").unwrap();

    let updated_toml_section = format!(
        "```toml\n[dependencies]\n{} = \"{}\"\n```",
        package_name, package_version
    );

    let updated_readme = re.replace(&readme_content, updated_toml_section.as_str());

    let mut readme_file = File::create("README.md")?;
    readme_file.write_all(updated_readme.as_bytes())?;

    println!("README.md updated: '{package_name}' = '{package_version}'");

    Ok(())
}
