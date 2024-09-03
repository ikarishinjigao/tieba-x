use anyhow::Result;
use std::{ffi::OsStr, fs, path::Path};
use walkdir::WalkDir;

fn main() -> Result<()> {
    let protobuf_source_dir = Path::new(env!("TIEBA_APK_PROTOBUF_GEN_DIR"));
    let protobuf_gen_dir = Path::new(env!("PROTOBUG_GEN_DIR"));
    fs::create_dir_all(protobuf_gen_dir)?;

    for entry in WalkDir::new(protobuf_source_dir).max_depth(1) {
        println!("{}", entry?.path().display());
    }

    // let files = WalkDir::new(protobuf_source_dir)
    //     .sort_by_file_name()
    //     .into_iter()
    //     .filter_map(|e| e.ok())
    //     .filter_map(|e| {
    //         let path = e.path();
    //         if e.file_type().is_file() {
    //             if let Some(extension) = path.extension().and_then(OsStr::to_str) {
    //                 match extension {
    //                     "proto" => Some(e),
    //                     _ => None,
    //                 }
    //             } else {
    //                 None
    //             }
    //         } else {
    //             None
    //         }
    //     })
    //     .collect::<Vec<_>>();
    // for entry in files {
    //     let path = entry.path();
    //     let relative_path = path.strip_prefix(protobuf_source_dir).unwrap();
    //     let output_path = protobuf_gen_dir.join(relative_path);
    //     println!("{:?}", path);
    //     println!("{:?}", output_path);

    //     let mut config = prost_build::Config::new();
    //     config.out_dir(protobuf_gen_dir);
    //     config.compile_protos(&[path], &[protobuf_source_dir])?;
    // }

    Ok(())
}
