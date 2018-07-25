use ::std;
use std::io::{Error, ErrorKind};


pub fn setup_support() -> Result<(), Error> {
    make_support()?;
    make_container()?;
    Ok(())
}
pub fn setup_config() -> Result<(), Error> {
    make_config()?;
    Ok(())
}


pub fn get_support_path() -> Option<String> {
    match std::env::home_dir() {
        Some(home) => {
            let support_path = format!("{}/.support", home.display());
            Some(support_path)
        },
        None => None
    }
}
pub fn get_container_path() -> Option<String> {
    match get_support_path() {
        Some(support_path) => {
            let container_name = env!("CARGO_PKG_NAME");
            let container_path = format!("{}/{}", support_path, container_name);
            Some(container_path)
        },
        None => None
    }
}
pub fn get_config_path() -> Option<String> {
    match get_container_path() {
        Some(container_path) => {
            let config_path = format!("{}/config.toml", container_path);
            Some(config_path)
        },
        None => None
    }
}


fn make_support() -> Result<(), Error> {
    if let Some(support_path) = get_support_path() {
        match is_path(&support_path) {
            true  => Ok(()),
            false => std::fs::create_dir(support_path)
        }
    } else {
        Err(Error::new(ErrorKind::Other, "Could not get support path"))
    }
}
fn make_container() -> Result<(), Error> {
    if let Some(container_path) = get_container_path() {
        match is_path(&container_path) {
            true  => Ok(()),
            false => std::fs::create_dir(container_path)
        }
    } else {
        Err(Error::new(ErrorKind::Other, "Could not get container path"))
    }
}
fn make_config() -> Result<(), Error> {
    if let Some(config_path) = get_config_path() {
        match is_path(&config_path) {
            true  => Ok(()),
            false => make_file(&config_path)
        }
    } else {
        Err(Error::new(ErrorKind::Other, "Could not get config path"))
    }
}
fn make_file(path: &str) -> Result<(), Error> {
    match std::fs::File::create(path).is_ok() {
        true  => Ok(()),
        false => Err(Error::new(ErrorKind::Other, "Could not create file"))
    }
}


fn is_path(path: &str) -> bool {
    std::path::Path::new(path).exists()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_support_path() {
        // case1: success
        let path = get_support_path();
        assert_eq!(path.is_some(), true);
    }
    #[test]
    fn test_get_container_path() {
        // case1: success
        let container = get_container_path();
        assert_eq!(container.is_some(), true);
    }
    #[test]
    fn test_get_config_path() {
        // case1: success
        let config = get_config_path();
        assert_eq!(config.is_some(), true);
    }

    #[test]
    fn test_make_file() {
        // case1: empty
        let path1   = "";
        let result1 = make_file(path1);
        assert_eq!(result1.is_ok(), false);
    }
    #[test]
    fn test_is_path() {
        // case1: empty
        let path1 = "";
        let result1 = is_path(path1);
        assert_eq!(result1, false);

        // case2: success
        let path2 = ".";
        let result2 = is_path(path2);
        assert_eq!(result2, true);
        
        // case3: failure
        let path3 = "/path/does/not/exist";
        let result3 = is_path(path3);
        assert_eq!(result3, false);

        // case4: tilde
        let path4 = "~";
        let result4 = is_path(path4);
        assert_eq!(result4, false);
    }
}
