use clap::Parser;
use gen::project::{Lang, Project, ProjectKind};
use std::str::FromStr;

#[derive(Parser)]
struct Args {
    #[arg(index = 1)]
    lang: String,
    #[arg(index = 2)]
    name: String,
    #[arg(index = 3)]
    kind: Option<String>,
    #[arg(short, long)]
    domain: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let name = Box::leak(args.name.into_boxed_str());
    let kind = match args.kind {
        Some(kind) => ProjectKind::from_str(&kind)?,
        None => ProjectKind::Executable,
    };

    let lang = Lang::from_str(&args.lang)?;
    if lang == Lang::Java && args.domain.is_none() {
        println!("Java project requires domain name! Use --domain option.");
        std::process::exit(1);
    }

    let project = Project::new(name, lang, kind, args.domain);
    project.generate()?;

    Ok(())
}
