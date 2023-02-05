use std::env;

extern crate dirs;

fn main() {
    //println!("{}", replace_dir_string("/home/stanaka/test".to_string()));
}

pub fn execute() {
    let vim_plugin_dir: String = read_vimplugindir_var();
    let nvim_plugin_dir: String = read_nvimplugindir_var();

    if dir_variable_check(&vim_plugin_dir, &nvim_plugin_dir) {
        let rep_vim_plugin_dir = replace_dir_string(vim_plugin_dir);
    }
}

pub fn read_vimplugindir_var() -> String {
    let mut dir: String = String::new();
    if match env::var("VIMPLUGINDIR") {
        Ok(_) => true,
        Err(_) => false,
    } {
        dir = env::var("VIMPLUGINDIR").unwrap();
    }
    dir
}

pub fn read_nvimplugindir_var() -> String {
    let mut dir: String = String::new();
    if match env::var("NVIMPLUGINDIR") {
        Ok(_) => true,
        Err(_) => false,
    } {
        dir = env::var("NVIMPLUGINDIR").unwrap();
    }
    dir
}

pub fn dir_variable_check(vimdir: &String, nvimdir: &String) -> bool {
    if vimdir.is_empty() && nvimdir.is_empty() {
        return false;
    }
    true
}

pub fn replace_dir_string(dirstring: String) -> String {
    let mut dir: String = dirstring.clone();
    if dirstring.find('~') != None {
        let hd = dirs::home_dir().unwrap().to_str().unwrap().to_string();
        dir = dirstring.replace(&"~".to_string(), "");
        dir = hd + &dir;
    } else if dirstring.find("$HOME") != None {
        let hd = env::var("HOME").unwrap();
        dir = dirstring.replace(&"$HOME".to_string(), "");
        dir = hd + &dir;
    }

    dir
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_vimplugindir_var() {
        assert_eq!(read_vimplugindir_var(), "test");
    }

    #[test]
    fn test_read_nvimplugindir_var() {
        assert_eq!(read_nvimplugindir_var(), "test2");
    }

    #[test]
    fn test_dir_variable_check() {
        let vim_dir: String = read_vimplugindir_var();
        let nvim_dir: String = read_nvimplugindir_var();
        assert_eq!(dir_variable_check(&vim_dir, &nvim_dir), true);
    }

    #[test]
    fn test_replace_dir_string() {
        assert_eq!(
            replace_dir_string("~/test".to_string()),
            "/home/stanaka/test"
        );
        assert_eq!(
            replace_dir_string("$HOME/test".to_string()),
            "/home/stanaka/test"
        );
        assert_eq!(
            replace_dir_string("/home/stanaka/test".to_string()),
            "/home/stanaka/test"
        );
    }

    #[test]
    fn test_home_dir() {
        let hd = dirs::home_dir().unwrap();

        assert_eq!("/home/stanaka", hd.to_str().unwrap());
    }
}
