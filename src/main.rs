use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(index = 1)]
    name: String,
    #[arg(index = 2)]
    lang: String,
    #[arg(index = 3)]
    kind: Option<String>,
}

use gen::project::{Lang, Project, ProjectKind};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let name = Box::leak(args.name.into_boxed_str());

    let lang = match args.lang.as_str() {
        "c" => Lang::C,
        "cpp" | "c++" | "cc" => Lang::Cpp,
        "java" => Lang::Java,
        "rust" | "rs" => Lang::Rust,
        _ => panic!("Invalid language"),
    };

    let kind = match args.kind.as_deref() {
        Some("bin") | Some("binary") | Some("exe") | Some("executable") => ProjectKind::Executable,
        Some("lib") | Some("library") => ProjectKind::Library,
        _ => ProjectKind::Executable,
    };

    let project = Project::new(name, lang, kind);
    project.generate()?;
    Ok(())
}
