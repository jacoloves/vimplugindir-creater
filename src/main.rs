use std::env;
use std::fs;

extern crate dirs;

static AUTOLOAD: &str = "autoload";
static DOC: &str = "doc";
static PLUGIN: &str = "plugin";
static VIMPLUGINDIR: &str = "VIMPLUGINDIR";
static NVIMPLUGINDIR: &str = "NVIMPLUGINDIR";

fn main() {
    let args: Vec<String> = env::args().collect();
    let plugin_name: String = args[1].clone();

    execute(plugin_name);
}

pub fn execute(plugin_name: String) {
    let vim_plugin_dir: String = read_vimplugindir_var();
    let nvim_plugin_dir: String = read_nvimplugindir_var();

    if dir_variable_check(&vim_plugin_dir, &nvim_plugin_dir) {
        let vim_plugin_dir = replace_dir_string(vim_plugin_dir);
        let nvim_plugin_dir = replace_dir_string(nvim_plugin_dir);

        /* vim plugin dir create */
        if !(vim_plugin_dir.is_empty()) {
            let root_plugin_dir = create_plugin_dir_name(vim_plugin_dir, plugin_name);
            let judge_flg: bool = match fs::create_dir(root_plugin_dir.clone()) {
                Ok(_) => true,
                Err(_) => false,
            };

            if judge_flg {
                println!("{} is created complete", root_plugin_dir.clone());
            } else {
                eprintln!("{} is not created", root_plugin_dir);
            }
        }
    }
}

pub fn read_vimplugindir_var() -> String {
    let mut dir: String = String::new();
    if match env::var(VIMPLUGINDIR) {
        Ok(_) => true,
        Err(_) => false,
    } {
        dir = env::var(VIMPLUGINDIR).unwrap();
    }
    dir
}

pub fn read_nvimplugindir_var() -> String {
    let mut dir: String = String::new();
    if match env::var(NVIMPLUGINDIR) {
        Ok(_) => true,
        Err(_) => false,
    } {
        dir = env::var(NVIMPLUGINDIR).unwrap();
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

pub fn create_plugin_dir_name(dir: String, plugin_name: String) -> String {
    let plugin_dir: String = dir + &"/".to_string() + &plugin_name;
    plugin_dir
}

pub fn create_plugin_detail_dir(dir: String) {
    let autoload_dir: String = create_plugin_dir_name(dir.clone(), AUTOLOAD.to_string());
    let doc_dir: String = create_plugin_dir_name(dir.clone(), DOC.to_string());
    let plugin_dir: String = create_plugin_dir_name(dir.clone(), PLUGIN.to_string());
}

#[cfg(test)]
mod test {
    use std::{fs::create_dir, path::Path};

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
    fn test_create_plugin_dir_name() {
        assert_eq!(
            create_plugin_dir_name("/home/stanaka/test".to_string(), "test-tool".to_string()),
            "/home/stanaka/test/test-tool"
        );
    }

    #[test]
    fn test_create_plugin_detail_dir() {
        let vim_plugin_dir: String = read_vimplugindir_var();
        let vim_plugin_dir = replace_dir_string(vim_plugin_dir);
        let root_plugin_dir = create_plugin_dir_name(vim_plugin_dir, "test-tool".to_string());
        fs::create_dir(root_plugin_dir.clone()).unwrap();

        create_plugin_detail_dir(root_plugin_dir)
    }

    #[test]
    fn test_home_dir() {
        let hd = dirs::home_dir().unwrap();

        assert_eq!("/home/stanaka", hd.to_str().unwrap());
    }
}
