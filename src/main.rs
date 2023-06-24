use std::cmp::min;
use std::{env};
use std::process::exit;
use argh::FromArgs;
use threadpool::ThreadPool;

mod files;
use files::{filter_files_to_packages, get_packages};
mod git;
use git::get_changed_files;
mod config;
use config::load_config;
mod runner;
use runner::run_on_shell;


fn get_env_dir() -> String {
    let root_dir_path = match env::current_dir() {
        Ok(root_dir) => root_dir,
        Err(e) => panic!("Can't get current dir {}", e),
    };

    return String::from(root_dir_path.to_str().unwrap());
}

fn default_concurrency() -> usize {
    1
}

/// Arguments
#[derive(FromArgs)]
struct Args {
    /// if shiv should run the command on all packages, or just those changed against main
    #[argh(switch)]
    detect_changes: bool,

    /// main branch name, default "main"
    #[argh(option, default = "String::from(\"main\")")]
    main_branch: String,

    /// root dir to run in
    #[argh(option, default = "get_env_dir()")]
    root_dir: String,

    /// package directory, default "packages"
    #[argh(option, default = "String::from(\"packages\")")]
    package_dir: String,

    /// command to run on packages
    #[argh(option)]
    command: String,

    /// max number of threads to run, default 0 
    #[argh(option, default = "default_concurrency()")]
    concurrency: usize,
}


fn main() {
    let args: Args = argh::from_env();

    run(
        args.detect_changes,
        &args.package_dir,
        &args.main_branch,
        &args.root_dir,
        &args.command,
        args.concurrency,
    );
}

fn run(
    detect_changes: bool,
    package_dir: &String,
    main_branch: &String,
    root_dir: &String,
    command: &String,
    concurrecy: usize
) {
    println!(
        "Running--- detect_changes: {}, root_dir: {}, command: {}, max_concurrency: {}",
        &detect_changes, &root_dir, &command, concurrecy
    );

    let mut packages = get_packages(&String::from(root_dir), package_dir);

    if detect_changes {
        println!("Detecting changes");
        let changed_files = get_changed_files(root_dir, main_branch);
        packages = filter_files_to_packages(packages, changed_files);

        if packages.len() == 0 {
            println!("Detected no changes in packages. Exiting");
            exit(0x0100);
        }
    }


    // if args have concurrecy passed in, use min value of packages.len, concurrency
    let threads = min(concurrecy, packages.len());
    
    let thread_pool = ThreadPool::new(threads.to_owned());
    for package in packages {
        let command = command.clone();
        let root_dir = root_dir.clone();
        println!("Detected package change: {}", package);
        thread_pool.execute(move || {
            run_command(command, root_dir, package);
        });
    }

    thread_pool.join();
}

/// Parses command, runs on runner, true if success, false if fail
fn run_command(command: String, root_dir: String, package: String) -> bool {
    // look for shiv.json file in package
    // process fork here
    let mut path = root_dir.clone();
    path.push_str("/");
    path.push_str(&package);

    let mut config_path = path.clone();
    config_path.push_str("/");
    config_path.push_str("shiv.json");

    let config = load_config(&config_path);

    let mut package_command: Option<String> = None;
    for script in config.scripts {
        if command == script.name {
            package_command = Some(script.run);
        }
    }

    if let Some(package_command) = package_command {
        println!("Running {} in {}", &package_command, package);
        // https://github.com/rust-lang/rust/issues/53667
        if let Ok(res) = run_on_shell(&package_command, &path) {
            let stdout_string = String::from_utf8(res.stdout).unwrap();
            for out_line in stdout_string.lines() {
                println!("[{}]  {}", package, out_line);
            }

            let stderr_string = String::from_utf8(res.stderr).unwrap();
            for err_line in stderr_string.lines() {
                println!("[{}]  {}", package, err_line);
            }

            if res.status.success() {
                println!("Successfully ran {} on {}", &package_command, &path);
                return true;
            }
        }

        // fallthrough err
        println!("Execution of {} on {} failed", &package_command, &path);
        return false;
    } else {
        println!("Found no entries for {} in {}", command, package);
        return true;
    }
}
