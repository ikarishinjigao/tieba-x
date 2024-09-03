use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::{
    env,
    ffi::OsStr,
    fmt, fs,
    io::Write,
    ops::Not,
    path::Path,
    process::{Command, Stdio},
};
use walkdir::WalkDir;

const EXCLUDE_JAVA_CLASSES: [&'static str; 1] = ["VideoActive"];

#[derive(Debug)]
struct Proto {
    package_name: String,
    dependencies: Vec<String>,
    name: String,
    fields: Vec<ProtoField>,
}

#[derive(Debug)]
struct ProtoField {
    tag: u32,
    name: String,
    data_type: String,
    java_type: String,
    repeated: bool,
}

fn parse_java_class(content: &str) -> Proto {
    let package_name = extract_package_name(&content);
    let dependencies = extract_dependencies(&content);
    let fields = extract_proto_fields(&content);
    Proto {
        package_name,
        dependencies,
        name: extract_class_name(&content),
        fields,
    }
}

fn extract_class_name(content: &str) -> String {
    Regex::new(r"public final class (?<class_name>.+) extends Message \{")
        .unwrap()
        .captures(content)
        .unwrap()
        .name("class_name")
        .unwrap()
        .as_str()
        .to_string()
}

fn extract_package_name(content: &str) -> String {
    let result = Regex::new(r"package (?<package_name>.+);")
        .unwrap()
        .captures(content)
        .unwrap()
        .name("package_name")
        .map_or("", |m| m.as_str())
        .to_string();
    format_package_name(&result)
}

fn extract_dependencies(content: &str) -> Vec<String> {
    let mut result = Regex::new(r"import (?<dependency>.+);")
        .unwrap()
        .captures_iter(content)
        .filter_map(|c| {
            let dependency = c.name("dependency").unwrap().as_str();
            if dependency.starts_with("tbclient") {
                Some(dependency)
            } else {
                None
            }
        })
        .map(|x| format_package_name(x))
        .filter(|x| EXCLUDE_JAVA_CLASSES.contains(&x.as_str()).not())
        .collect::<Vec<_>>();
    result.sort();
    result
}

fn extract_proto_fields(content: &str) -> Vec<ProtoField> {
    let mut result = Regex::new(r"\s*@ProtoField\((label = Message\.Label\.(?<label>.*?), |)tag = (?<tag>\d+)(, type = Message\.Datatype\.(?<data_type>.*?)|)\)\s*public final (?<java_type>.*?) (?<name>.*?);")
        .unwrap()
        .captures_iter(content)
        .filter_map(|c| {
            Some(ProtoField {
                tag: c.name("tag").unwrap().as_str().parse().unwrap(),
                name: c.name("name").unwrap().as_str().to_string(),
                data_type: c.name("data_type").map_or("", |m| m.as_str()).to_string(),
                java_type: c.name("java_type").map_or("", |m| m.as_str()).to_string(),
                repeated: c.name("label").map_or("", |m| m.as_str()) == "REPEATED",
            })
        })
        .filter(|x| EXCLUDE_JAVA_CLASSES.contains(&x.java_type.as_str()).not())
        .collect::<Vec<_>>();
    result.sort_by_key(|x| x.tag);
    result
}

fn format_package_name(source: &str) -> String {
    source
        .split(".")
        .filter(|x| *x != "tbclient")
        .collect::<Vec<_>>()
        .join("/")
}

fn export_apk_source() -> Result<()> {
    let apk_path = Path::new(env!("TIEBA_APK_DOWNLOAD_PATH"));
    let output_path = Path::new(env!("TIEBA_APK_SOURCE_EXPORT_DIR"));
    Command::new("jadx")
        .arg("--rename-flags")
        .arg("none")
        .arg("--output-dir")
        .arg(output_path)
        .arg(apk_path)
        .stdout(Stdio::inherit())
        .output()
        .expect("Failed to export apk sources");
    Ok(())
}

fn generate_protobuf_files() -> Result<()> {
    let apk_protobuf_dir = Path::new(env!("TIEBA_APK_SOURCE_EXPORT_DIR")).join("sources/tbclient");
    let apk_protobuf_dir = apk_protobuf_dir.as_path();

    let protobuf_dir = Path::new(env!("TIEBA_APK_PROTOBUF_GEN_DIR"));
    if protobuf_dir.exists() {
        fs::remove_dir_all(protobuf_dir)?;
    }

    let source_files = WalkDir::new(apk_protobuf_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            if e.file_type().is_file() {
                if let Some(extension) = path.extension().and_then(OsStr::to_str) {
                    match extension {
                        "java" => Some(e),
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let progress_bar = create_progress_bar(source_files.len().try_into().unwrap());
    for entry in source_files {
        let path = entry.path();
        let relative_path = path.strip_prefix(apk_protobuf_dir).unwrap();
        let mut output_file_path = protobuf_dir.join(relative_path);
        output_file_path.set_extension("proto");
        progress_bar.set_message(format!(
            "Generating {}",
            output_file_path
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap()
        ));
        if let Some(extension) = path.extension().and_then(OsStr::to_str) {
            match extension {
                "java" => {
                    let content: String = fs::read_to_string(path)?;
                    let proto = parse_java_class(&content);
                    fs::create_dir_all(output_file_path.parent().unwrap())?;
                    let mut file = fs::File::create(output_file_path)?;
                    file.write_all(format!("{}", proto).as_bytes())?;
                    progress_bar.inc(1);
                }
                _ => (),
            }
        }
    }
    progress_bar.finish_with_message("All protobuf files generated successfully.");

    Ok(())
}

fn create_progress_bar(total_size: u64) -> ProgressBar {
    let progress_bar_style = ProgressStyle::with_template(
        "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7}",
    )
    .unwrap()
    .progress_chars("#>-");
    let pb = ProgressBar::new(total_size);
    pb.set_style(progress_bar_style);
    pb
}

impl fmt::Display for ProtoField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let field_type: String = if self.repeated {
            Regex::new(r"List<(?<type>.+)>")
                .unwrap()
                .captures(&self.java_type)
                .unwrap()
                .name("type")
                .unwrap()
                .as_str()
                .to_string()
        } else {
            if self.data_type.is_empty() {
                self.java_type.clone()
            } else {
                self.data_type.to_lowercase()
            }
        };
        if self.repeated {
            write!(f, "repeated ")?;
        }
        write!(f, "{} {} = {};", field_type, self.name, self.tag)
    }
}

impl fmt::Display for Proto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "syntax = \"proto3\";")?;
        if !self.package_name.is_empty() {
            writeln!(f, "package {};", self.package_name)?;
        }
        for dep in &self.dependencies {
            writeln!(f, "import \"{}.proto\";", dep)?;
        }
        writeln!(f, "message {} {{", self.name)?;
        for field in &self.fields {
            writeln!(f, "    {}", field)?;
        }
        write!(f, "}}")
    }
}

fn main() -> Result<()> {
    export_apk_source()?;
    generate_protobuf_files()?;
    Ok(())
}
