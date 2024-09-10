use anyhow::{Context, Ok, Result};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::{
    collections::HashSet,
    env,
    ffi::OsStr,
    fmt, fs,
    io::{BufWriter, Write},
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
    field_type: String,
    has_dependency: bool,
    repeated: bool,
}

fn parse_java_class(content: &str, root_path: &Path) -> Result<Proto> {
    let package_name = extract_package_name(&content)?;
    let class_name = extract_class_name(content)?;
    let fields = extract_proto_fields(&content);
    let mut dependencies = extract_dependencies(&content);
    let extra_dependencies = fields.iter().filter(|x| x.has_dependency).filter_map(|x| {
        extract_proto_dependency(root_path, &package_name, &class_name, &x.field_type)
    });
    dependencies.extend(extra_dependencies);
    let mut dependencies = dependencies.into_iter().collect::<Vec<_>>();
    dependencies.sort();
    Ok(Proto {
        package_name,
        dependencies,
        name: class_name,
        fields,
    })
}

fn extract_class_name(content: &str) -> Result<String> {
    Regex::new(r"public final class (?<class_name>.+) extends Message \{")
        .unwrap()
        .captures(content)
        .and_then(|c| c.name("class_name"))
        .map(|m| m.as_str().to_string())
        .context("Failed to extract calss name")
}

fn extract_package_name(content: &str) -> Result<String> {
    let result = Regex::new(r"package (?<package_name>.+);")
        .unwrap()
        .captures(content)
        .and_then(|c| c.name("package_name"))
        .map(|m| m.as_str())
        .context("Failed to extract package name")?;
    Ok(format_package_name(&result))
}

fn extract_dependencies(content: &str) -> HashSet<String> {
    Regex::new(r"import (?<dependency>.+);")
        .unwrap()
        .captures_iter(content)
        .filter_map(|c| {
            let dependency = c.name("dependency")?.as_str();
            if dependency.starts_with("tbclient") {
                Some(format_package_name(dependency))
            } else {
                None
            }
        })
        .filter(|x| !EXCLUDE_JAVA_CLASSES.contains(&x.as_str()))
        .collect()
}

fn extract_proto_dependency(
    root_path: &Path,
    package_name: &str,
    class_name: &str,
    java_type: &str,
) -> Option<String> {
    let java_file = format!("{}.java", java_type);
    let proto_file_path_from_package = root_path.join(package_name).join(&java_file);
    let proto_file_path_from_root = root_path.join(&java_file);
    if class_name == java_type {
        None
    } else if !package_name.is_empty() && proto_file_path_from_package.exists() {
        Some(format!("{}/{}", package_name, java_type))
    } else if proto_file_path_from_root.exists() {
        Some(java_type.to_string())
    } else {
        None
    }
}

fn extract_proto_fields(content: &str) -> Vec<ProtoField> {
    let mut result = Regex::new(r"\s*@ProtoField\((label = Message\.Label\.(?<label>.*?), |)tag = (?<tag>\d+)(, type = Message\.Datatype\.(?<data_type>.*?)|)\)\s*public final (?<java_type>.*?) (?<name>.*?);")
        .unwrap()
        .captures_iter(content)
        .filter_map(|c| {
            let data_type = c.name("data_type").map_or("", |m| m.as_str()).to_string();
            let java_type = c.name("java_type").map_or("", |m| m.as_str()).to_string();
            let repeated =  c.name("label").map_or("", |m| m.as_str()) == "REPEATED";
            let has_dependency :bool;
            let field_type = if repeated && data_type.is_empty() {
                has_dependency = true;
                Regex::new(r"List<(?<type>.+)>")
                    .unwrap()
                    .captures(&java_type)
                    .and_then(|c| c.name("type"))
                    .map(|m| m.as_str())
                    .unwrap_or(&java_type)
                    .to_string()
            } else if data_type.is_empty() {
                has_dependency = true;
                java_type
            } else {
                has_dependency = false;
                data_type.to_lowercase()
            };
            Some(ProtoField {
                tag: c.name("tag").unwrap().as_str().parse().unwrap(),
                name: c.name("name").unwrap().as_str().to_string().to_lowercase(),
                field_type: field_type.clone(),
                has_dependency,
                repeated: c.name("label").map_or("", |m| m.as_str()) == "REPEATED",
            })
        })
        .filter(|x| !EXCLUDE_JAVA_CLASSES.contains(&x.field_type.as_str()))
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
    if output_path.exists() {
        fs::remove_dir_all(output_path)?;
    }
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
    let protobuf_dir = Path::new(env!("TIEBA_APK_PROTOBUF_GEN_DIR"));
    if protobuf_dir.exists() {
        fs::remove_dir_all(protobuf_dir)?;
    }

    let source_files = WalkDir::new(&apk_protobuf_dir)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file() && e.path().extension() == Some(OsStr::new("java")))
        .collect::<Vec<_>>();

    let progress_bar = create_progress_bar(source_files.len() as u64);
    source_files.iter().try_for_each(|entry| {
        let path = entry.path();
        let relative_path = path.strip_prefix(&apk_protobuf_dir).unwrap();
        let mut output_file_path = protobuf_dir.join(relative_path);
        output_file_path.set_extension("proto");

        progress_bar.set_message(format!(
            "Generating {}",
            output_file_path
                .file_name()
                .and_then(OsStr::to_str)
                .unwrap()
        ));

        let content: String = fs::read_to_string(path)?;
        let proto = parse_java_class(&content, &apk_protobuf_dir)?;

        fs::create_dir_all(output_file_path.parent().unwrap())?;
        let file = fs::File::create(output_file_path)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "{}", proto)?;

        progress_bar.inc(1);
        Ok(())
    })?;

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
        if self.repeated {
            write!(f, "repeated ")?;
        }
        write!(f, "{} {} = {};", self.field_type, self.name, self.tag)
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
