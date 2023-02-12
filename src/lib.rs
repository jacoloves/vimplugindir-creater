use std::env;
use std::fs;
extern crate dirs;

static AUTOLOAD: &str = "autoload";
static DOC: &str = "doc";
static GITKEEP: &str = ".gitkeep";
static NVIMPLUGINDIR: &str = "NVIMPLUGINDIR";
static PLUGIN: &str = "plugin";
static VIMPLUGINDIR: &str = "VIMPLUGINDIR";

pub fn read_vimplugindir_var() -> String {
    let mut dir: String = String::new();
    if match env::var(VIMPLUGINDIR).is_ok() {
        true => true,
        false => false,
    } {
        dir = env::var(VIMPLUGINDIR).unwrap();
    }
    dir
}

pub fn read_nvimplugindir_var() -> String {
    let mut dir: String = String::new();
    if match env::var(NVIMPLUGINDIR).is_ok() {
        true => true,
        false => false,
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
    } else if dirstring.find("$XDG_CONFIG_HOME") != None {
        let hd = env::var("XDG_CONFIG_HOME").unwrap();
        dir = dirstring.replace(&"$XDG_CONFIG_HOME".to_string(), "");
        dir = hd + &dir;
    }

    dir
}

pub fn create_plugin_dir_or_file_name(dir: String, plugin_name: String) -> String {
    let plugin_dir: String = dir + &String::from("/") + &plugin_name;
    plugin_dir
}

pub fn create_plugin_detail_dir(dir: String) {
    let autoload_dir: String = create_plugin_dir_or_file_name(dir.clone(), AUTOLOAD.to_string());
    let doc_dir: String = create_plugin_dir_or_file_name(dir.clone(), DOC.to_string());
    let plugin_dir: String = create_plugin_dir_or_file_name(dir, PLUGIN.to_string());

    match fs::create_dir(autoload_dir.clone()) {
        Ok(_) => println!("{} is created complete.", autoload_dir),
        Err(_) => eprintln!("{} is not created", autoload_dir),
    }

    match fs::create_dir(doc_dir.clone()) {
        Ok(_) => println!("{} is created complete.", doc_dir),
        Err(_) => eprintln!("{} is not created", doc_dir),
    }

    match fs::create_dir(plugin_dir.clone()) {
        Ok(_) => println!("{} is created complete.", plugin_dir),
        Err(_) => eprintln!("{} is not created", plugin_dir),
    }
}

pub fn create_plugin_detail_file(dir: String) {
    let autoload_dir: String = create_plugin_dir_or_file_name(dir.clone(), AUTOLOAD.to_string());
    let doc_dir: String = create_plugin_dir_or_file_name(dir.clone(), DOC.to_string());
    let plugin_dir: String = create_plugin_dir_or_file_name(dir, PLUGIN.to_string());

    let autoload_file: String = create_plugin_dir_or_file_name(autoload_dir, GITKEEP.to_string());
    let doc_file: String = create_plugin_dir_or_file_name(doc_dir, GITKEEP.to_string());
    let plugin_file: String = create_plugin_dir_or_file_name(plugin_dir, GITKEEP.to_string());

    match fs::File::create(autoload_file.clone()) {
        Ok(_) => println!("{} is created complete.", autoload_file),
        Err(_) => eprintln!("{} is not created", autoload_file),
    }

    match fs::File::create(doc_file.clone()) {
        Ok(_) => println!("{} is created complete.", doc_file),
        Err(_) => eprintln!("{} is not created", doc_file),
    }

    match fs::File::create(plugin_file.clone()) {
        Ok(_) => println!("{} is created complete.", plugin_file),
        Err(_) => eprintln!("{} is not created", plugin_file),
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_read_vimplugindir_var() {
        assert_eq!(read_vimplugindir_var(), "/home/runner");
    }

    #[test]
    fn test_read_nvimplugindir_var() {
        assert_eq!(read_nvimplugindir_var(), "/home/runner");
    }

    #[test]
    fn test_dir_variable_check() {
        let vim_dir: String = read_vimplugindir_var();
        let nvim_dir: String = read_nvimplugindir_var();
        assert!(dir_variable_check(&vim_dir, &nvim_dir));
    }

    #[test]
    fn test_replace_dir_string() {
        assert_eq!(
            replace_dir_string("~/test".to_string()),
            "/home/runner/test"
        );
        assert_eq!(
            replace_dir_string("$HOME/test".to_string()),
            "/home/runner/test"
        );
        assert_eq!(
            replace_dir_string("/home/runner/test".to_string()),
            "/home/runner/test"
        );
    }

    #[test]
    fn test_create_plugin_dir_or_file_name() {
        assert_eq!(
            create_plugin_dir_or_file_name(
                "/home/runner/test".to_string(),
                "test-tool".to_string()
            ),
            "/home/runner/test/test-tool"
        );
    }

    #[test]
    fn test_create_plugin_detail_dir() {
        let vim_plugin_dir: String = read_vimplugindir_var();
        let vim_plugin_dir = replace_dir_string(vim_plugin_dir);
        let root_plugin_dir =
            create_plugin_dir_or_file_name(vim_plugin_dir, "test-tool".to_string());
        fs::create_dir(root_plugin_dir.clone()).unwrap();
        create_plugin_detail_dir(root_plugin_dir.clone());

        let autoload_dir: String =
            create_plugin_dir_or_file_name(root_plugin_dir.clone(), AUTOLOAD.to_string());
        let doc_dir: String =
            create_plugin_dir_or_file_name(root_plugin_dir.clone(), DOC.to_string());
        let plugin_dir: String =
            create_plugin_dir_or_file_name(root_plugin_dir.clone(), PLUGIN.to_string());

        assert!(Path::new(&autoload_dir).exists());
        assert!(Path::new(&doc_dir).exists());
        assert!(Path::new(&plugin_dir).exists());

        fs::remove_dir_all(root_plugin_dir).unwrap();
    }

    #[test]
    fn test_create_plugin_detail_file() {
        let vim_plugin_dir: String = read_vimplugindir_var();
        let vim_plugin_dir = replace_dir_string(vim_plugin_dir);
        let root_plugin_dir =
            create_plugin_dir_or_file_name(vim_plugin_dir, "test-tool".to_string());
        fs::create_dir(root_plugin_dir.clone()).unwrap();
        create_plugin_detail_dir(root_plugin_dir.clone());

        create_plugin_detail_file(root_plugin_dir.clone());

        let autoload_dir: String =
            create_plugin_dir_or_file_name(root_plugin_dir.clone(), AUTOLOAD.to_string());
        let doc_dir: String =
            create_plugin_dir_or_file_name(root_plugin_dir.clone(), DOC.to_string());
        let plugin_dir: String =
            create_plugin_dir_or_file_name(root_plugin_dir.clone(), PLUGIN.to_string());

        let autoload_file: String =
            create_plugin_dir_or_file_name(autoload_dir, GITKEEP.to_string());
        let doc_file: String = create_plugin_dir_or_file_name(doc_dir, GITKEEP.to_string());
        let plugin_file: String = create_plugin_dir_or_file_name(plugin_dir, GITKEEP.to_string());

        assert!(Path::new(&autoload_file).exists());
        assert!(Path::new(&doc_file).exists());
        assert!(Path::new(&plugin_file).exists());

        fs::remove_dir_all(root_plugin_dir).unwrap();
    }
}
