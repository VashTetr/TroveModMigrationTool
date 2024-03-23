use std::{env, fs, io};
use std::path::{Path, PathBuf};

use platform_dirs::AppDirs;
use winreg::enums::*;
use winreg::RegKey;

static mut GLYPH: String = String::new();
static mut STEAM: String = String::new();

fn get_current_working_dir() -> io::Result<PathBuf> {
    env::current_dir()
}


fn main() {
    print_description();
    print_menu();
    let mut loopbool: bool = true;
    println!("Press enter to exit!");
    while loopbool {
        let mut input = String::new();
        let stdin_present = io::stdin();
        stdin_present.read_line(&mut input).unwrap();
        match input.trim_end() {
            _ => {
                loopbool = false;
            }
        }
    }
}

fn print_description() {
    print!("
    -------------------------------------------------------------------------------------
    This Script inserts your mods and ModCfgs into their respective Directory.
    To use this Script you will need to have 2 Folders in the current Script Directory
    They should be named:
    mods
    ModCfgs
    In These Folders there should be the desired .tmod files (mods folder) and .cfg files
    (ModCfgs folder)
    If they are not present, they will get created on the first run of this script.
    -------------------------------------------------------------------------------------
    \n")
}

fn print_menu() {
    let mut loopbool: bool = true;
    let menu_text = "Choose which Trove version you are using \n    [1] Glyph\n    [2] Steam";
    println!("{}", &menu_text);
    while loopbool {
        let mut input = String::new();
        let stdin_present = io::stdin();
        stdin_present.read_line(&mut input).unwrap();

        match input.trim_end() {
            "1" => unsafe {
                find_glyph_install_trove().expect("fetching trove glyph install");
                let local_glyph = GLYPH.clone();
                move_files(local_glyph);
                loopbool = false;
            }
            "2" => unsafe {
                find_steam_install_trove().expect("fetching trove steam install");
                let local_steam = STEAM.clone();
                move_files(local_steam);
                loopbool = false;
            }
            _ => {
                println!("Invalid choice. Please enter 1 or 2.");
                println!("{}", &menu_text);
            }
        }
    }
}

fn find_glyph_install_trove() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let glyph_trove_paths = [
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Glyph Trove",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Glyph Trove Europe",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Glyph Trove North America"];

    for path in glyph_trove_paths {
        if let Ok(ret) = hklm.open_subkey(path) {
            let test: String = ret.get_value("InstallLocation")?;
            unsafe { GLYPH = test; }
        }
    }
    Ok(())
}

fn find_steam_install_trove() -> io::Result<()> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let steam_trove_paths = "SOFTWARE\\WOW6432Node\\Valve\\SteamService";

    if let Ok(ret) = hklm.open_subkey(steam_trove_paths) {
        let test: String = ret.get_value("installpath_default")?;
        unsafe { STEAM = test + "\\steamapps\\common\\Trove\\Games\\Trove\\Live"; }
        unsafe {
            if !Path::new(&STEAM).exists() {
                println!("not found")
            }
        }
    }
    Ok(())
}

fn find_appdata_trove() {
    let app_dirs = AppDirs::new(Some(""), false).unwrap();
    if !Path::new(&app_dirs.config_dir.join("Trove")).exists() {} else if !Path::new(&app_dirs.config_dir.join("Trove\\ModCfgs")).exists() {
        println!("ModCfgs folder in Appdata not found!");
        println!("Creating ModCfgs folder in Appdata...");
        if let Ok(ret) = fs::create_dir(&app_dirs.config_dir.join("Trove\\ModCfgs")) {
            println!("ModCfgs folder created!");
        } else {
            println!("ModCfgs folder could not be created!")
        }
    }
}

fn move_files(mut dest_mods: String) {
    let source_mod_dir = get_current_working_dir().unwrap().into_os_string().into_string().unwrap() + "\\mods";
    let source_modcfgs_dir = get_current_working_dir().unwrap().into_os_string().into_string().unwrap() + "\\ModCfgs";
    if !Path::new(&source_mod_dir).exists() {
        println!("mods folder is not present in current directory!");
        println!("creating mods folder...");
        if let Ok(ret) = fs::create_dir(&source_mod_dir) {
            println!("mods folder created!");
        } else {
            println!("mods folder could not be created!")
        }
    }
    if !Path::new(&source_modcfgs_dir).exists() {
        println!("ModCfgs folder is not present in current directory!");
        println!("creating ModCfgs folder...");
        if let Ok(ret) = fs::create_dir(&source_modcfgs_dir) {
            println!("ModCfgs folder created!");
        } else {
            println!("ModCfgs folder could not be created!")
        }
    }
    let source_mod_dir_amount = fs::read_dir(&source_mod_dir).unwrap();
    let source_modcfgs_dir_amount = fs::read_dir(&source_modcfgs_dir).unwrap();

    if dest_mods.to_lowercase().contains("glyph") {
        dest_mods = dest_mods.as_str().to_owned() + "\\mods";
    }
    if dest_mods.to_lowercase().contains("steam") {
        dest_mods = dest_mods.as_str().to_owned() + "\\mods";
    }
    if !Path::new(&dest_mods).exists() {
        println!("{} {}", &dest_mods, "could not be found");
        return;
    }
    for mods in source_mod_dir_amount {
        let dest_dir = dest_mods.to_owned()
            + "\\"
            + mods.as_ref().unwrap().file_name().to_str().unwrap();

        if !Path::new(&dest_dir).exists() {
            println!("{:?} {} {}", mods.as_ref().unwrap().path(), "moved to", &dest_dir);
            fs::copy(mods.expect("fetch current mod path").path(), &dest_dir).expect("move current mod path");
        }
    }
    println!("Mod Migration Successful");
    find_appdata_trove();
    let app_dirs = AppDirs::new(Some(""), false).unwrap();

    for cfgs in source_modcfgs_dir_amount {
        let dest_dir = format!("{}{}{}", &app_dirs.config_dir.join("Trove\\ModCfgs").into_os_string().into_string().unwrap()
                               , "\\"
                               , cfgs.as_ref().unwrap().file_name().to_str().unwrap());

        if !Path::new(&dest_dir).exists() {
            println!("{:?} {} {}", cfgs.as_ref().unwrap().path(), "moved to", &dest_dir);
            fs::copy(cfgs.expect("fetch current cfgs path").path(), &dest_dir).expect("move current cfgs path");
        }
    }
    println!("ModCfgs Migration Successful");
}


