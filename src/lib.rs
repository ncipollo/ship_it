use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::string::ToString;
use uuid::Uuid;
use crate::repo::clone::{GitCloner, RepoCloner};
use crate::repo::commit::{GitCommitter, RepoCommitter};
use crate::repo::open::{GitOpener, RepoOpener};
use crate::repo::options::CloneOptions;

mod repo;

pub fn sheep_test() {
    let opener = GitOpener::new();
    let cloner = GitCloner::new();
    let test_repo_path = shellexpand::tilde("~/Desktop/test-sheep").to_string();
    let options = CloneOptions::new(
        "git@github.com:ncipollo/test-sheep.git",
        &test_repo_path,
    );



    let repo = if Path::new(&test_repo_path).exists() {
        opener.open(test_repo_path)
            .expect("git repo open failed")
    } else {
        cloner.clone(options)
            .expect("clone failed")
    };

    write_test_file();

    let committer = GitCommitter::new();
    let thing = "test.txt";
    committer.commit(&repo,
                     vec!["test.txt"],
                     "test commit!");
}

fn test_file_path() -> PathBuf {
    return PathBuf::from(shellexpand::tilde("~/Desktop/test-sheep/test.txt").to_string())
}

fn write_test_file() {
    let id = Uuid::new_v4();

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(test_file_path())
        .expect("couldn't open test file");
    write!(file, "uuid: {}", id.to_string()).expect("failed to write to test file");
    println!("uuid: {}", id.to_string())
}