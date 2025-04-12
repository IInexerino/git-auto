use std::process::{Command, exit};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, next_line_help = true)]
pub struct GitAutoArgs {

    #[arg(short, long)]
    message: String,

    #[arg(short, long = "repository", default_value_t = String::from("origin"))]
    repo: String,

    #[arg(short, long, default_value_t = String::from("master"))]
    branch: String,
}

fn update_commit_push_set_origin() {
    let git_auto_args = GitAutoArgs::parse();

    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output().expect("Failed to execute git add command");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files to the git repo.");
        eprintln!("Git stderr: {}", String::from_utf8_lossy(&add_command.stderr));
        eprintln!("Git stdout: {}", String::from_utf8_lossy(&add_command.stdout));
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(git_auto_args.message)
        .output().expect("Failed to execute git commit command");

    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit changes.");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg(git_auto_args.repo.clone())
        .arg(git_auto_args.branch)
        .output().expect("Failed to push to github repository");

    if !push_command.status.success() {
        eprintln!("Error: Failed to push changes to repository.");
        exit(1);
    }

    if git_auto_args.repo == "origin" { } else {
        let origin_switch_command = Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(git_auto_args.repo)
            .output().expect("Failed to change remote origin");

            if !origin_switch_command.status.success() {
                eprintln!("Error: Failed to set remote origin.");
                exit(1);
            }
    }

    println!("Sucessfuly added, committed, and pushed all changes.");
}

fn main() {
    update_commit_push_set_origin();
}
