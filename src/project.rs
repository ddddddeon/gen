use anyhow::anyhow;
use handlebars::Handlebars;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

#[derive(Debug, Serialize, Eq, PartialEq, Clone, Copy)]
pub enum ProjectKind {
    Library,
    Executable,
}

impl FromStr for ProjectKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bin" | "binary" | "exe" | "executable" => Ok(ProjectKind::Executable),
            "lib" | "library" => Ok(ProjectKind::Library),
            _ => Ok(ProjectKind::Executable),
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize)]
pub enum Lang {
    Rust,
    C,
    Cpp,
    Java,
}

impl FromStr for Lang {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "rust" | "rs" => Ok(Lang::Rust),
            "c" => Ok(Lang::C),
            "cpp" | "c++" | "cc" => Ok(Lang::Cpp),
            "java" => Ok(Lang::Java),
            _ => Err(anyhow!("Unknown language {}", s)),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Project {
    name: String,
    lang: Lang,
    kind: ProjectKind,
    project_dir: Option<&'static Path>,
    template_dir: Option<&'static Path>,
    domain: Option<String>,
}

impl Project {
    pub fn new(
        name: &'static str,
        lang: Lang,
        kind: ProjectKind,
        domain: Option<String>,
    ) -> Project {
        let mut project = Project {
            name: String::from(name),
            lang,
            kind,
            project_dir: None,
            template_dir: None,
            domain,
        };

        let project_dir = Path::new(name);
        if project_dir.is_dir() {
            println!(
                "Directory {} already exists! Refusing to overwrite",
                project.name
            );
            std::process::exit(1);
        }

        let template_dir = match project.lang {
            Lang::Rust => Path::new("~/.config/gen/templates/rust"),
            Lang::C => Path::new("~/.config/gen/templates/c"),
            Lang::Cpp => Path::new("~/.config/gen/templates/cpp"),
            Lang::Java => Path::new("~/.config/gen/templates/java"),
        };

        if !template_dir.is_dir() {
            println!(
                "Template directory {} does not exist!",
                template_dir.display()
            );
            std::process::exit(1);
        }

        project.template_dir = Some(template_dir);
        project.project_dir = Some(project_dir);
        project
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn lang(&self) -> &Lang {
        &self.lang
    }

    pub fn kind(&self) -> &ProjectKind {
        &self.kind
    }

    pub fn create_dir(&self) -> anyhow::Result<()> {
        if let Some(project_dir) = &self.project_dir {
            match fs::create_dir(&self.name) {
                Ok(()) => {
                    println!("Created dir  {}", self.name);
                }
                Err(error) => {
                    println!("Error creating directory {}: {:?}", self.name, error);
                    return Err(error.into());
                }
            }

            match fs::create_dir(project_dir.join("src")) {
                Ok(()) => {
                    println!("Created dir  {}", project_dir.join("src").display());
                }
                Err(error) => {
                    println!(
                        "Error creating directory {}: {:?}",
                        project_dir.join("src").display(),
                        error
                    );
                    return Err(error.into());
                }
            }
            Ok(())
        } else {
            Err(anyhow::anyhow!("Project directory not set"))
        }
    }

    pub fn template(
        &self,
        target_name: &str,
        from_path: &Path,
        to_path: &Path,
    ) -> anyhow::Result<()> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_file(target_name, from_path)?;
        let rendered_makefile = handlebars.render(target_name, &self)?;
        fs::File::create(to_path)?;
        fs::write(to_path, rendered_makefile)?;
        println!("Created file {}", to_path.display());
        Ok(())
    }

    pub fn create_makefile(&self) -> anyhow::Result<()> {
        if let (Some(template_dir), Some(project_dir)) = (&self.template_dir, &self.project_dir) {
            let makefile_name = match self.kind {
                ProjectKind::Library => "Makefile.lib",
                ProjectKind::Executable => "Makefile.bin",
            };
            self.template(
                "Makefile",
                &template_dir.join(makefile_name),
                &project_dir.join("Makefile"),
            )?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Project directory not set"))
        }
    }

    pub fn create_gitignore(&self) -> anyhow::Result<()> {
        if let (Some(template_dir), Some(project_dir)) = (&self.template_dir, &self.project_dir) {
            fs::copy(
                template_dir.join(".gitignore"),
                project_dir.join(".gitignore"),
            )?;
            println!("Created file {}", project_dir.join(".gitignore").display());
            Ok(())
        } else {
            Err(anyhow::anyhow!("Template or project directory not set"))
        }
    }

    pub fn create_c_project(&self) -> anyhow::Result<()> {
        if let (Some(project_dir), Some(template_dir)) = (&self.project_dir, &self.template_dir) {
            if self.kind == ProjectKind::Executable {
                fs::copy(
                    template_dir.join("src").join("main.c"),
                    project_dir.join("src").join("main.c"),
                )?;
                println!(
                    "Created file {}",
                    project_dir.join("src").join("main.c").display()
                );
            }
        }
        Ok(())
    }

    pub fn create_cpp_project(&self) -> anyhow::Result<()> {
        if let (Some(project_dir), Some(template_dir)) = (&self.project_dir, &self.template_dir) {
            if self.kind == ProjectKind::Executable {
                self.template(
                    "main.cpp",
                    &template_dir.join("src").join("main.cpp"),
                    &project_dir.join("src").join("main.cpp"),
                )?;
            }
        }
        Ok(())
    }

    pub fn create_java_project(&self) -> anyhow::Result<()> {
        let domain = match &self.domain {
            Some(domain) => domain,
            None => "com.example",
        };

        let output = Command::new("mvn")
            .arg("archetype:generate")
            .arg(format!("-DgroupId={}.{}", domain, self.name))
            .arg(format!("-DartifactId={}", self.name))
            .arg("-DarchetypeArtifactId=maven-archetype-quickstart")
            .arg("-DinteractiveMode=false")
            .output();

        match output {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
            Err(error) => {
                println!("{}", error.to_string());
            }
        }
        if let (Some(project_dir), Some(template_dir)) = (&self.project_dir, &self.template_dir) {
            self.template(
                "manifest.txt",
                &template_dir.join("manifest.txt"),
                &project_dir.join("manifest.txt"),
            )?;
        } else {
            return Err(anyhow::anyhow!("Template or project directory not set"));
        }
        Ok(())
    }

    pub fn create_rust_project(&self) -> anyhow::Result<()> {
        let args = match self.kind {
            ProjectKind::Library => "--lib",
            ProjectKind::Executable => "--bin",
        };

        let output = Command::new("cargo")
            .arg("new")
            .arg(&self.name)
            .arg(args)
            .output();

        match output {
            Ok(output) => {
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
            Err(error) => {
                println!("{}", error.to_string());
            }
        }

        if let (Some(project_dir), Some(template_dir)) = (&self.project_dir, &self.template_dir) {
            if self.kind == ProjectKind::Executable {
                fs::copy(
                    template_dir.join("src").join("main.rs"),
                    project_dir.join("src").join("main.rs"),
                )?;
                println!(
                    "Created file {}",
                    project_dir.join("src").join("main.rs").display()
                );
            }

            fs::File::create(project_dir.join("src").join("lib.rs"))?;
            println!(
                "Created file {}",
                project_dir.join("src").join("lib.rs").display()
            );
        }
        Ok(())
    }

    pub fn generate(&self) -> anyhow::Result<()> {
        match self.lang {
            Lang::C => {
                self.create_dir()?;
                self.create_c_project()?;
            }
            Lang::Cpp => {
                self.create_dir()?;
                self.create_cpp_project()?;
            }
            Lang::Java => {
                self.create_java_project()?;
            }
            Lang::Rust => {
                self.create_rust_project()?;
            }
        }

        self.create_gitignore()?;
        self.create_makefile()?;
        Ok(())
    }
}
