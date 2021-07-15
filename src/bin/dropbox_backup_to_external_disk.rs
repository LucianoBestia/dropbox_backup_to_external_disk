//! dropbox_backup_to_external_disk.rs

use dropbox_backup_to_external_disk::*;

fn main() {
    env_logger::init();

    let bin_name = "dropbox_backup_to_external_disk";

    match std::env::args().nth(1).as_deref() {
        None | Some("--help") | Some("-h") => print_help(&bin_name),
        Some("test") => test_connection(),
        Some("one_way_sync") => match std::env::args().nth(2).as_deref() {
            Some(path) => {
                let ns_started = ns_start("one_way_sync");
                one_way_sync(path);
                ns_print("one_way_sync", ns_started);
            }
            _ => println!("Unrecognized arguments. Try {} --help", &bin_name),
        },
        Some("list_remote") => {
            ansi_clear_screen();
            println!(
                "{}{}",
                ansi_set_row(1),
                "list_remote into temp_data/list_remote_files.csv"
            );
            let ns_started = ns_start("");
            list_remote();
            ns_print("list_remote", ns_started);
        }
        Some("list_local") => match std::env::args().nth(2).as_deref() {
            Some(path) => {
                ansi_clear_screen();
                println!(
                    "{}{}",
                    ansi_set_row(1),
                    "list_local into temp_data/list_local_files.csv"
                );
                let ns_started = ns_start("");
                list_local(path);
                ns_print("list_local", ns_started);
            }
            _ => println!("Unrecognized arguments. Try {} --help", &bin_name),
        },
        Some("compare_sorted_lists") => {
            let ns_started = ns_start("compare sorted lists");
            compare_sorted_lists();
            ns_print("compare_sorted_lists", ns_started);
        }
        Some("download") => match std::env::args().nth(2).as_deref() {
            Some(path) => download(path),
            _ => println!("Unrecognized arguments. Try {} --help", &bin_name),
        },
        Some("download_from_list") => {
            let ns_started = ns_start("download from temp_data/list_for_download.csv");
            download_from_list();
            ns_print("download_from_list", ns_started);
        }
        Some("trash_from_list") => {
            let ns_started = ns_start("trash from temp_data/list_for_trash.csv");
            trash_from_list();
            ns_print("trash_from_list", ns_started);
        }
        Some("correct_time_from_list") => {
            let ns_started = ns_start("correct time of files from temp_data/list_for_correct_time.csv");
            correct_time_from_list();
            ns_print("correct_time_from_list", ns_started);
        }
        _ => println!("Unrecognized arguments. Try {} --help", &bin_name),
    }
}

fn print_help(bin_name: &str) {
    println!("usage: $ {} <command> [options] [<args>]", bin_name);
    println!("  ");

    println!("1. Before first use, create your private Dropbox app:");
    println!("- open browser on <https://www.dropbox.com/developers/apps?_tk=pilot_lp&_ad=topbar4&_camp=myapps>");
    println!("- click Create app");
    println!("- choose Scoped access");
    println!("- choose Full dropbox");
    println!("- choose a unique app name like `backup_20210715_125500`");
    println!("- go to tab Permissions");
    println!("- check `files.metadata.read` and `files.content.read`");
    println!("- click Submit");
    println!("- close browser");
    println!("  ");

    println!("2. Before every use, create a temporary access token:");
    println!("- open browser on <https://www.dropbox.com/developers/apps?_tk=pilot_lp&_ad=topbar4&_camp=myapps>");
    println!(". choose your private Dropbox app `backup_20210715_125500`");
    println!("- click button Generate to generated temporary access token and copy it");
    println!("- close browser");
    println!("- In you Linux terminal session set a temporary private/secret environment variable:");
    println!("$ export DBX_OAUTH_TOKEN= here paste the access token ");
    println!("- create an alias for easy of use:"); 
    println!("$ alias dropbox_backup_to_external_disk=target/debug/dropbox_backup_to_external_disk");
    println!("- test if the authentication works: ");
    println!("$ dropbox_backup_to_external_disk test");

    println!("  ");
    println!("  ");

    println!("One-way sync download:");
    println!("  $ {} one_way_sync /mnt/d/DropBoxBackup2", bin_name);
    println!("  ");

    println!("For debugging purpose, you can run every step separately.");

    println!("Test connection and authorization:");
    println!("  $ {} test", bin_name);
    println!("  ");

    println!("List all files in your remote Dropbox to temp_data/list_remote_files.csv:");
    println!("  $ {} list_remote", bin_name);
    println!("  ");

    println!("List local files to temp_data/list_local_files.csv:");
    println!("  $ {} list_local /mnt/d/DropBoxBackup2", bin_name);
    println!("  ");

    println!("Compare lists and create temp_data/list_for_download.csv, temp_data/list_for_trash.csv and temp_data/list_for_correct_time.csv:");
    println!("  $ {} compare_sorted_lists", bin_name);
    println!("  ");

    println!("Correct time of files from temp_data/list_for_correct_time.csv:");
    println!("  $ {} correct_time_from_list", bin_name);
    println!();

    println!("Move to trash folder from temp_data/list_for_trash.csv:");
    println!("  $ {} trash_from_list", bin_name);
    println!();

    println!("Download files from temp_data/list_for_download.csv:");
    println!("  $ {} download_from_list", bin_name);
    println!();

    println!("Download one single file:");
    println!("  $ {} download <path>", bin_name);
    println!();

}
