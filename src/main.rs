use std::env;
use std::fs;

use vimplugin_creater::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let plugin_name: String = args[1].clone();

    execute(plugin_name);
}

fn execute(plugin_name: String) {
    let vim_plugin_dir: String = read_vimplugindir_var();
    let nvim_plugin_dir: String = read_nvimplugindir_var();

    if dir_variable_check(&vim_plugin_dir, &nvim_plugin_dir) {
        let vim_plugin_dir = replace_dir_string(vim_plugin_dir);
        let nvim_plugin_dir = replace_dir_string(nvim_plugin_dir);

        /* vim plugin dir create */
        if !(vim_plugin_dir.is_empty()) {
            let root_plugin_dir =
                create_plugin_dir_or_file_name(vim_plugin_dir, plugin_name.clone());
            let judge_flg: bool = fs::create_dir(root_plugin_dir.clone()).is_ok();

            if judge_flg {
                println!("{} is created complete", root_plugin_dir);
                create_plugin_detail_dir(root_plugin_dir.clone());
                create_plugin_detail_file(root_plugin_dir);
            } else {
                eprintln!("{} is not created", root_plugin_dir);
            }
            println!("vim plugin directory create done")
        }

        /* nvim plugin dir create */
        if !(nvim_plugin_dir.is_empty()) {
            let root_plugin_dir = create_plugin_dir_or_file_name(nvim_plugin_dir, plugin_name);
            let judge_flg: bool = fs::create_dir(root_plugin_dir.clone()).is_ok();

            if judge_flg {
                println!("{} is created complete", root_plugin_dir);
                create_plugin_detail_dir(root_plugin_dir.clone());
                create_plugin_detail_file(root_plugin_dir);
            } else {
                eprintln!("{} is not created", root_plugin_dir);
            }
            println!("nvim plugin directory create done")
        }
    }
}
