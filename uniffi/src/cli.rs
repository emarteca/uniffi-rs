/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};

// Structs to help our cmdline parsing. Note that docstrings below form part
// of the "help" output.

/// Scaffolding and bindings generator for Rust
#[derive(Parser)]
#[clap(name = "uniffi-bindgen")]
#[clap(version = clap::crate_version!())]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate foreign language bindings
    Generate {
        /// Foreign language(s) for which to build bindings.
        #[clap(long, short, possible_values = &["kotlin", "python", "swift", "ruby"])]
        language: Vec<String>,

        /// Directory in which to write generated files. Default is same folder as .udl file.
        #[clap(long, short)]
        out_dir: Option<Utf8PathBuf>,

        /// Do not try to format the generated bindings.
        #[clap(long, short)]
        no_format: bool,

        /// Path to the optional uniffi config file. If not provided, uniffi-bindgen will try to guess it from the UDL's file location.
        #[clap(long, short)]
        config: Option<Utf8PathBuf>,

        /// Extract proc-macro metadata from a native lib (cdylib or staticlib) for this crate.
        #[clap(long)]
        lib_file: Option<Utf8PathBuf>,

        /// Pass in a crate name rather than a UDL file
        #[clap(long = "crate")]
        crate_mode: bool,

        /// Path to the UDL file or crate
        source: String,
    },

    /// Generate Rust scaffolding code
    Scaffolding {
        /// Directory in which to write generated files. Default is same folder as .udl file.
        #[clap(long, short)]
        out_dir: Option<Utf8PathBuf>,

        /// Path to the optional uniffi config file. If not provided, uniffi-bindgen will try to guess it from the UDL's file location.
        #[clap(long, short)]
        config: Option<Utf8PathBuf>,

        /// Do not try to format the generated bindings.
        #[clap(long, short)]
        no_format: bool,

        /// Path to the UDL file.
        udl_file: Utf8PathBuf,
    },

    /// Print the JSON representation of the interface from a dynamic library
    PrintJson {
        /// Path to the library file (.so, .dll, .dylib, or .a)
        path: Utf8PathBuf,
    },
}

pub fn run_main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate {
            language,
            out_dir,
            no_format,
            config,
            lib_file,
            source,
            crate_mode,
        } => {
            if crate_mode {
                if lib_file.is_some() {
                    panic!("--lib-file is not compatible with --crate.  The library will be found automatically.")
                }
                if config.is_some() {
                    panic!("--config is not compatible with --crate.  The config file(s) will be found automatically.")
                }
                let out_dir = out_dir.expect("--out-dir is required when using --crate");
                if language.is_empty() {
                    panic!("no languages specified")
                }
                uniffi_bindgen::crate_mode::generate_bindings(
                    &source, &language, &out_dir, !no_format,
                )?;
            } else {
                uniffi_bindgen::generate_bindings(
                    &Utf8PathBuf::from(source),
                    config.as_deref(),
                    language.iter().map(String::as_str).collect(),
                    out_dir.as_deref(),
                    lib_file.as_deref(),
                    !no_format,
                )?;
            }
        }
        Commands::Scaffolding {
            out_dir,
            config,
            no_format,
            udl_file,
        } => {
            uniffi_bindgen::generate_component_scaffolding(
                &udl_file,
                config.as_deref(),
                out_dir.as_deref(),
                !no_format,
            )?;
        }
        Commands::PrintJson { path } => {
            uniffi_bindgen::print_json(&path)?;
        }
    };
    Ok(())
}
