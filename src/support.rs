use std;
use std::io::{Error, ErrorKind};


pub fn setup_support() -> Result<(), Error> {
    if let Some(support) = support_path() {
        if path_exists(&support) {
            return Ok(())
        } 
        return std::fs::create_dir(support);
    } 
    return Err(Error::new(ErrorKind::Other, "Could not get support path"));
}
pub fn setup_config() -> Result<(), Error> {
    if let Some(config) = config_path() {
        if path_exists(&config) {
            return Ok(())
        }

        return match std::fs::File::create(config) {
            Ok(_)  => Ok(()),
            Err(e) => Err(e),
        }
    }
    return Err(Error::new(ErrorKind::Other, "Could not get config path"));
}


pub fn support_path() -> Option<String> {
    if let Some(home) = std::env::home_dir() {
        let name = env!("CARGO_PKG_NAME");
        let path = format!("{}/.{}", home.display(), name);
        return Some(path);
    }
    return None;
}
pub fn config_path() -> Option<String> {
    if let Some(path) = support_path() {
        let config = format!("{}/config.toml", path);
        return Some(config);
    }
    return None;
}


fn path_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_support_path() {
        // case1: success
        let path = support_path();
        assert!(path.is_some());
    }
    #[test]
    fn test_config_path() {
        // case1: success
        let config = config_path();
        assert!(config.is_some());
    }
    #[test]
    fn test_path_exists() {
        // case1: empty
        let path1 = "";
        let result1 = path_exists(path1);
        assert!(result1 == false);

        // case2: success
        let path2 = ".";
        let result2 = path_exists(path2);
        assert!(result2 == true);
        
        // case3: failure
        let path3 = "/path/does/not/exist";
        let result3 = path_exists(path3);
        assert!(result3 == false);

        // case4: tilde
        let path4 = "~";
        let result4 = path_exists(path4);
        assert!(result4 == false);
    }
}
