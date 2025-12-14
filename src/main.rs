mod env;

use std::process::{Command, Output};

use crate::env::Environment;

fn main() {
    let env = Environment::load();

    if !env.script_before.is_empty() {
        exec(&env, &env.script_before);
    }
    deploy(&env);
    if !env.script_after.is_empty() {
        exec(&env, &env.script_after);
    }
}

fn deploy(env: &Environment) {
    let mut command = Command::new("scp");
    command.arg("-r");
    command.args(&["-i", &env.key]);
    command.args(&["-P", &env.port]);
    command.args(&["-o", "StrictHostKeyChecking=no"]);
    for target in &env.targets {
        command.arg(target);
    }
    let remote = format!("{}@{}:{}", env.user, env.host, env.dir);
    command.arg(remote);

    let output = command.output().unwrap();
    if output.status.success() {
        println!("[DEPLOY] SUCCESS");
    } else {
        println!("[DEPLOY] ERROR");
    }
    handle_output(&output);
}

pub fn exec(env: &Environment, command: &str) {
    let remote = format!("{}@{}", env.user, env.host);
    let output = Command::new("ssh")
        .args(&[
            "-i",
            &env.key,
            "-o",
            "BatchMode=yes",
            "-o",
            "StrictHostKeyChecking=no",
        ])
        .arg(&remote)
        .arg(command)
        .output()
        .unwrap();

    println!("[EXEC] {}", command.trim().replace("\n", " && "));
    handle_output(&output);
}

fn handle_output(output: &Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.is_empty() {
        println!("{}", stdout.trim());
    }
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            eprintln!("{}", stderr);
        } else {
            let status = output.status.code().map_or(-1, |c| c);
            eprintln!("The command failed with exit code: {}", status);
        }
        std::process::exit(1);
    }
}
