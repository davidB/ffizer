// see :
// - [std::error::Error - Rust](https://doc.rust-lang.org/std/error/trait.Error.html)
// - [Error Handling - A Gentle Introduction to Rust](https://stevedonovan.github.io/rust-gentle-intro/6-error-handling.html)
// - [snafu::guide::comparison::failure - Rust](https://docs.rs/snafu/0.4.3/snafu/guide/comparison/failure/index.html)
// - [Error Handling in Rust - Andrew Gallant's Blog](https://blog.burntsushi.net/rust-error-handling/)
use snafu::Snafu;
use std::path::PathBuf;

// pub type Result<T> = std::result::Result<T, Box<std::error::Error + Send + Sync>>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[allow(clippy::large_enum_variant)] // warn to restore to default
#[snafu(visibility = "pub(crate)")]
pub enum Error {
    #[snafu(display("{}", msg))]
    Any {
        msg: String,
    },

    #[snafu(display("value {:?} of {} is not in {:?}", value, value_name, accepted))]
    StringValueNotIn {
        value_name: String,
        value: String,
        accepted: Vec<String>,
    },

    #[snafu(display("git retreive {:?} (rev: {:?}) into folder {:?}", url, rev, dst))]
    GitRetrieve {
        dst: PathBuf,
        url: String,
        rev: String,
        source: git2::Error,
    },
    #[snafu(display("try to find git config '{:?}'", key))]
    GitFindConfig {
        key: String,
        source: git2::Error,
    },

    #[snafu(display("create folder {:?}", path))]
    CreateFolder {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("remove folder {:?}", path))]
    RemoveFolder {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("create file {:?}", path))]
    CreateFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("rename file from {:?} to {:?} ", src, dst))]
    RenameFile {
        src: PathBuf,
        dst: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("copy file from {:?} to {:?} ", src, dst))]
    CopyFile {
        src: PathBuf,
        dst: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("copy permission from {:?} to {:?} ", src, dst))]
    CopyFilePermission {
        src: PathBuf,
        dst: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("read file {:?}", path))]
    ReadFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("write file {:?}", path))]
    WriteFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("remove file {:?}", path))]
    RemoveFile {
        path: PathBuf,
        source: std::io::Error,
    },
    #[snafu(display("run command '{:?}'", cmd))]
    RunCommand {
        cmd: String,
        source: std::io::Error,
    },
    ParsePathPattern {
        value: String,
        source: globset::Error,
    },

    ParseGitUri {
        value: String,
        source: regex::Error,
    },

    #[snafu(display(
        "local path({:?}) not found for uri({:?}) subfolder({:?})",
        path,
        uri,
        subfolder
    ))]
    LocalPathNotFound {
        path: PathBuf,
        uri: String,
        subfolder: Option<PathBuf>,
    },

    #[snafu(display("Application directory not found"))]
    ApplicationPathNotFound {},

    //HACK
    Io {
        source: std::io::Error,
    },
    //HACK
    Handlebars {
        when: String,
        template: String,
        source: handlebars::TemplateRenderError,
    },
    //HACK
    SerdeYaml {
        source: serde_yaml::Error,
    },
    //HACK
    ScriptError {
        script: String,
        source: run_script::ScriptError,
    },
    //HACK
    SerdeJson {
        source: serde_json::Error,
    },
}
