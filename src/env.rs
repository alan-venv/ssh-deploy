use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

fn get(key: &str) -> String {
    return env::var(format!("INPUT_{}", key)).unwrap_or_else(|_| {
        println!("Environment variable \"{}\" not found.", key);
        std::process::exit(1);
    });
}

fn key_handler() -> String {
    let content = get("REMOTE_KEY");
    let path = "/home/runner/key";
    let directory = Path::new(path);
    let parent_directory = directory.parent().expect("invalid path");
    fs::create_dir_all(parent_directory).expect("error trying to create the directory path");
    let final_content = format!("{}\n", content.trim());
    fs::write(directory, final_content.as_bytes()).expect("error trying to save private key");
    let mut permissoes = fs::metadata(directory)
        .expect("Error trying to retrieve file metadata")
        .permissions();
    permissoes.set_mode(0o600);
    fs::set_permissions(directory, permissoes).expect("Error trying to setting file permissions");
    return path.to_owned();
}

pub struct Environment {
    pub key: String,
    pub host: String,
    pub user: String,
    pub port: String,
    pub dir: String,
    pub targets: Vec<String>,
    pub script_before: String,
    pub script_after: String,
}

impl Environment {
    pub fn load() -> Self {
        let user = get("REMOTE_USER");
        let host = get("REMOTE_HOST");
        let port = get("REMOTE_PORT");
        let dir = get("REMOTE_DIR");
        let files = get("TARGETS");
        let targets = files.split(" ").into_iter().map(String::from).collect();
        let key = key_handler();
        let script_before = get("SCRIPT_BEFORE");
        let script_after = get("SCRIPT_AFTER");
        return Environment {
            user,
            host,
            port,
            dir,
            targets,
            key,
            script_before,
            script_after,
        };
    }
}

#[allow(dead_code)] // for local tests
fn load_file() {
    if let Ok(contents) = fs::read_to_string(".env") {
        for line in contents.lines() {
            if let Some((key, value)) = line.split_once('=') {
                unsafe { env::set_var(key.trim(), value.trim()) };
            }
        }
    }
}
