use std::io::Result;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

pub struct Artifact {
  pub path: PathBuf,
  pub manifest_path: PathBuf,
}

#[derive(Default)]
pub struct Build {
  release: bool,
}

impl Build {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn release(mut self, release: bool) -> Self {
    self.release = release;
    self
  }

  pub fn build(self, path: &Path) -> Result<Artifact> {
    let mut build_cmd = Command::new("cargo");
    build_cmd
      .current_dir(path)
      .arg("build")
      // .arg("--message-format=json")
      .arg("--lib")
      // .arg("--features")
      // .arg("media-gstreamer")
      .stdout(Stdio::piped());

    if self.release {
      build_cmd.arg("--release");
    }

    let build_status = build_cmd.status()?;
    if build_status.success() {
      println!("Build succeeded, continue with metadata parsing.");

      let mut metadata_cmd = Command::new("cargo");
      metadata_cmd
        .current_dir(path)
        .arg("metadata")
        .arg("--format-version=1")
        .stdout(Stdio::piped());

      let metadata_status = metadata_cmd.status()?;
      let metadata_output = metadata_cmd.output()?;
      println!("Parsed, status: {:?}", metadata_status);
      if metadata_status.success() {
        println!("metadata command ok.");
        let reader = std::io::BufReader::new(metadata_output.stdout.as_slice());
        let mut artifacts = vec![];
        for message in cargo_metadata::Message::parse_stream(reader) {
          match message.unwrap() {
            cargo_metadata::Message::CompilerArtifact(artifact) => {
              println!("Artifact {:?}",artifact);
              if artifact.target.kind.contains(&"cdylib".to_string()) {
                artifacts.push(Artifact {
                  path: PathBuf::from(artifact.filenames[0].to_string()),
                  manifest_path: PathBuf::from(
                    artifact.manifest_path.to_string(),
                  ),
                });
              }
            }
            _ => {
              println!("Nope");
            }
          }
        }
  
        println!("After thing");
  
        // TODO: Fix. Not an ideal way to get the artifact of the desired crate, but it
        // works for most case.
        if let Some(artifact) = artifacts.pop() {
          return Ok(artifact);
        }
  
        Err(std::io::Error::new(
          std::io::ErrorKind::Other,
          "failed to parse cargo output",
        ))?
      } else {
        println!(
          "failed to execute `cargo metadata`: exited with {}\n  full command: {:?}",
          metadata_status, metadata_cmd,
        );
  
        std::process::exit(1);
      }
    } else {
      println!(
        "failed to execute `cargo build`: exited with {}\n  full command: {:?}",
        build_status, build_cmd,
      );

      std::process::exit(1);
    }
  }
}

pub fn metadata() -> Result<String> {
  let metadata = cargo_metadata::MetadataCommand::new()
    .exec()
    .map_err(|e| {
      println!("failed to execute `cargo metadata`: {}", e);
      std::process::exit(1);
    })
    .unwrap();

  Ok(metadata.root_package().unwrap().name.clone())
}
