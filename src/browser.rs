use ::std;
use ::support;
use ::toml;


#[derive(Debug, Default, Deserialize)]
pub struct Bookmarks {
    sites: std::collections::HashMap<String, String>
}
impl Bookmarks {
    pub fn default() -> Bookmarks {
        Bookmarks {..Default::default()} 
    }
    pub fn setup(&mut self) {
        let support = support::setup_support();       
        let config  = support::setup_config();

        if support.is_ok() && config.is_ok() {
            if let Some(config) = support::config_path() {
                self.configure(&config);
            }
        }
    }
    pub fn configure(&mut self, config: &str) {
        if let Ok(contents) = std::fs::read_to_string(config) {
            if let Ok(values) = toml::from_str(&contents) {
                *self = values;
            }
        }
    }
    pub fn open_site(&self, key: &str) {
        if let Some(url) = self.sites.get(key) {
            open_url(url);
        }
    }
}


pub fn edit_config() {
    if let Some(config) = support::config_path() {
        if let Ok(editor) = std::env::var("EDITOR") {
            let process = format!("{} {}", editor, config);
            shell(&process);
        }
    }
}


// deb: portability
fn open_url(url: &str) {
    let process = format!("open {}", url);
    shell(&process);
}
fn shell(process: &str) {
    let process_vector: Vec<_> = process
        .split_whitespace()
        .collect();
    
    if let Some(command) = process_vector.get(0) {
        let arguments = process_vector.get(1..).unwrap();
        let child = std::process::Command::new(command)
            .args(arguments)
            .spawn();

        if let Err(error) = child {
            println!("{}: {}", command, error);
        } else {
            child.unwrap().wait().expect("failed to wait on child");
        }
    }
}