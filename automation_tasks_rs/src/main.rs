//! automation_tasks_rs with_lib

use cargo_auto_lib::*;

/// automation_tasks_rs with_lib
fn main() {
    if is_not_run_in_rust_project_root_directory() {
        println!("Error: automation_tasks_rs must be called in the root directory of the rust project beside the Cargo.toml file and automation_tasks_rs directory.");
        // early exit
        std::process::exit(0);
    }

    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("Running automation task: {}", &task);
                if &task == "build" || &task == "b" {
                    task_build();
                } else if &task == "release" || &task == "r" {
                    task_release();
                } else if &task == "docs" || &task == "doc" || &task == "d" {
                    task_docs();
                } else {
                    println!("Task {} is unknown.", &task);
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(r#"
User defined tasks in automation_tasks_rs:
cargo auto build - builds the crate in debug mode, fmt
cargo auto release - builds the crate in release mode, version from date, fmt
cargo auto docs - builds the docs, copy to docs directory
    
Create alias for easy use when developing:
  $ alias dropbox_backup_to_external_disk=target/debug/dropbox_backup_to_external_disk
Create auto-completion:
  $ complete -C "dropbox_backup_to_external_disk completion" dropbox_backup_to_external_disk
dropbox_backup_to_external_disk --help - instructions especially for first use because of authentication
"#);
}

/// sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`
fn completion() {
    /// println one, more or all sub_commands
    fn completion_return_one_or_more_sub_commands(sub_commands: Vec<&str>, word_being_completed: &str) {
        let mut sub_found = false;
        for sub_command in sub_commands.iter() {
            if sub_command.starts_with(word_being_completed) {
                println!("{}", sub_command);
                sub_found = true;
            }
        }
        if sub_found == false {
            // print all sub-commands
            for sub_command in sub_commands.iter() {
                println!("{}", sub_command);
            }
        }
    }

    let args: Vec<String> = std::env::args().collect();
    let last_word = args[2].as_str();
    let mut word_being_completed = " ";
    if args.len() > 3 {
        word_being_completed = args[3].as_str();
    }
    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec!["build", "release", "doc", "publish_to_crates_io"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["with_lib"];
        completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// region: tasks

/// example how to call a list of shell commands
fn task_build() {
    auto_version_from_date();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo fmt", 
        "cargo build",
        "target/debug/dropbox_backup_to_external_disk --help",
        ];
    run_shell_commands(shell_commands.to_vec());
    println!(r#"
Create alias for easy use when developing:
  $  alias dropbox_backup_to_external_disk=target/debug/dropbox_backup_to_external_disk
Create auto-completion:
  $  complete -C "dropbox_backup_to_external_disk completion" dropbox_backup_to_external_disk

After `cargo auto build`, run the tests and the code. If ok, then `cargo auto release`
"#);
}

/// example how to call one shell command and combine with rust code
fn task_release() {    
    auto_version_from_date();
    auto_cargo_toml_to_md();
    auto_lines_of_code("");

    run_shell_command("cargo fmt");
    run_shell_command("cargo build --release");
    run_shell_command("target/release/dropbox_backup_to_external_disk --help");
    println!(r#"
Create alias for easy use when developing:
    $  alias dropbox_backup_to_external_disk=target/release/dropbox_backup_to_external_disk
Create auto-completion:
    $  complete -C "dropbox_backup_to_external_disk completion" dropbox_backup_to_external_disk

After `cargo auto release`, run the tests and the code. If ok, then `cargo auto doc`
"#);
}

/// example how to call a list of shell commands and combine with rust code
fn task_docs() {
    auto_cargo_toml_to_md();
    auto_lines_of_code("");
    auto_md_to_doc_comments();
    #[rustfmt::skip]
    let shell_commands = [
        "cargo doc --no-deps --document-private-items --open",
        // copy target/doc into docs/ because it is github standard
        "rsync -a --info=progress2 --delete-after target/doc/ docs/",
        "echo Create simple index.html file in docs directory",
        &format!("echo \"<meta http-equiv=\\\"refresh\\\" content=\\\"0; url={}/index.html\\\" />\" > docs/index.html",package_name().replace("-","_")) ,
    ];
    run_shell_commands(shell_commands.to_vec());
    // message to help user with next move
    println!(
        r#"
After `cargo auto doc`, check `docs/index.html`. If ok, then `git commit -am"message"` and `git push`,
then create release on Github"#);
}



// endregion: tasks

// region: helper functions

/// check if run in rust project root directory error
/// there must be Cargo.toml and the directory automation_tasks_rs
fn is_not_run_in_rust_project_root_directory() -> bool {
    // return negation of exists
    !(std::path::Path::new("automation_tasks_rs").exists() && std::path::Path::new("Cargo.toml").exists())
}

// endregion: helper functions

    