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
    let mut process = process.split_whitespace();
    
    if let Some(command) = process.nth(0) {
        let arguments: Vec<_> = process.collect();
        let child = std::process::Command::new(command)
            .args(arguments)
            .spawn();

        if let Err(error) = child {
            println!("{}: {}", command, error);
        } 
        else if let Ok(mut child) = child {
            if let Err(error) = child.wait() {
                println!("{}: {}", command, error);
            }
        }
    }
}
