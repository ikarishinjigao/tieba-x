use anyhow::Result;
use std::{ffi::OsStr, fs, path::Path};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let protobuf_source_dir = Path::new(env!("TIEBA_APK_PROTOBUF_GEN_DIR"));
    let protobuf_gen_dir = Path::new(env!("PROTOBUG_GEN_DIR"));
    if protobuf_gen_dir.exists() {
        fs::remove_dir_all(protobuf_gen_dir)?;
    }
    fs::create_dir_all(protobuf_gen_dir)?;

    let files = WalkDir::new(protobuf_source_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("proto")))
        .map(|e| e.into_path())
        .collect::<Vec<_>>();

    let mut config = prost_build::Config::new();
    config.out_dir(protobuf_gen_dir);
    config.compile_well_known_types();
    config.default_package_filename("shared");
    config.include_file("_include.rs");
    config.compile_protos(&files, &[protobuf_source_dir])?;

    Ok(())
}
