use gen::project::{Lang, Project, ProjectKind};

fn main() -> anyhow::Result<()> {
    let project = Project::new("testing", Lang::Java, ProjectKind::Library);
    project.generate()?;
    Ok(())
}
