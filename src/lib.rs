#![feature(proc_macro_hygiene)]

mod config;
use arcropolis_api::*;
use config::*;
use skyline::nn::ro::LookupSymbol;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::lua2cpp::L2CFighterCommon;
use std::{collections::HashMap, fs::metadata, path::PathBuf, sync::Mutex};
use walkdir::WalkDir;

lazy_static::lazy_static! {
    pub static ref FILES_CONFIG: Mutex<HashMap<u64, EntryInfo>> = Mutex::new(HashMap::new());
}

#[derive(Debug)]
pub struct EntryInfo {
    pub normal_path: String,
    pub game_path: String,
    pub size: u64,
}

const STARTING_DIR: &str = "rom:/VictoryStage";

pub static mut VICTORY_FIGHTER_KIND: i32 = 0;
pub static mut VICTORY_COLOR_INDEX: i32 = 0;
pub static mut ENTRY_ID: usize = 0;
pub static mut VICTOR: usize = 0;

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
        ENTRY_ID =
            WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let fighter_manager = *(FIGHTER_MANAGER_ADDR as *mut *mut smash::app::FighterManager);
        VICTOR = FighterManager::get_top_rank_player(fighter_manager, 0) as usize;
        if ENTRY_ID == VICTOR {
            VICTORY_FIGHTER_KIND = smash::app::utility::get_kind(module_accessor);
            VICTORY_COLOR_INDEX =
                WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
            // println!(
            //     "Victory Kind (Name): {}\nVictroy Kind (ID): {}\n----------------",
            //     get_character_name(VICTORY_FIGHTER_KIND),
            //     VICTORY_FIGHTER_KIND
            // );
        }
    }
}

pub fn get_character_name(id: i32) -> &'static str {
    match id {
        0 => "mario",
        1 => "donkey",
        2 => "link",
        3 => "samus",
        4 => "samusd",
        5 => "yoshi",
        6 => "kirby",
        7 => "fox",
        8 => "pikachu",
        9 => "luigi",
        10 => "ness",
        11 => "captain",
        12 => "purin",
        13 => "peach",
        14 => "daisy",
        15 => "koopa",
        16 => "sheik",
        17 => "zelda",
        18 => "mariod",
        19 => "pichu",
        20 => "falco",
        21 => "marth",
        22 => "lucina",
        23 => "younglink",
        24 => "ganon",
        25 => "mewtwo",
        26 => "roy",
        27 => "chrom",
        28 => "gamewatch",
        29 => "metaknight",
        30 => "pit",
        31 => "pitb",
        32 => "szerosuit",
        33 => "wario",
        34 => "snake",
        35 => "ike",
        36 => "pzenigame",
        37 => "pfushigisou",
        38 => "plizardon",
        39 => "diddy",
        40 => "lucas",
        41 => "sonic",
        42 => "dedede",
        43 => "pikmin",
        44 => "lucario",
        45 => "robot",
        46 => "toonlink",
        47 => "wolf",
        48 => "murabito",
        49 => "rockman",
        50 => "wiifit",
        51 => "rosetta",
        52 => "littlemac",
        53 => "gekkouga",
        54 => "palutena",
        55 => "pacman",
        56 => "reflet",
        57 => "shulk",
        58 => "koopajr",
        59 => "duckhunt",
        60 => "ryu",
        61 => "ken",
        62 => "cloud",
        63 => "kamui",
        64 => "bayonetta",
        65 => "inkling",
        66 => "ridley",
        67 => "simon",
        68 => "richter",
        69 => "krool",
        70 => "shizue",
        71 => "gaogaen",
        72 => "miifighter",
        73 => "miiswordsman",
        74 => "miigunner",
        75 => "iceclimber", // popo
        76 => "iceclimber", // nana
        77 => "koopag",
        78 => "miienemyf",
        79 => "miienemys",
        80 => "miienemyg",
        81 => "packun",
        82 => "jack",
        83 => "brave",
        84 => "buddy",
        85 => "dolly",
        86 => "master",
        87 => "tantan",
        88 => "pickel",
        89 => "edge",
        90 => "eflame",
        91 => "elight",
        110 => "ice_climber",
        111 => "zenigame",
        112 => "fushigisou",
        113 => "lizardon",
        114 => "ptrainer",
        _ => "unknown",
    }
}

#[arc_callback]
fn file_callback(hash: u64, data: &mut [u8]) -> Option<usize> {
    unsafe {
        let chara_name = get_character_name(VICTORY_FIGHTER_KIND);
        let folder: String;
        if !CHARCTER_CONFIG
            .lock()
            .unwrap()
            .entries
            .contains_key(chara_name)
        {
            folder = VICTORY_FIGHTER_KIND.to_string();
        } else {
            if !CHARCTER_CONFIG
                .lock()
                .unwrap()
                .entries
                .get(chara_name)
                .unwrap()
                .id_color
                .contains_key(&VICTORY_COLOR_INDEX)
            {
                folder = CHARCTER_CONFIG
                    .lock()
                    .unwrap()
                    .entries
                    .get(chara_name)
                    .unwrap()
                    .default
                    .to_string();
            } else {
                folder = CHARCTER_CONFIG
                    .lock()
                    .unwrap()
                    .entries
                    .get(chara_name)
                    .unwrap()
                    .id_color
                    .get(&VICTORY_COLOR_INDEX)
                    .unwrap()
                    .to_string();
            }
        }

        println!("{}", folder);

        let physical_path = format!(
            "{}/{}/{}",
            STARTING_DIR,
            folder,
            &FILES_CONFIG.lock().unwrap().get(&hash).unwrap().game_path
        );

        let original_file = format!(
            "{}/Default/{}",
            STARTING_DIR,
            &FILES_CONFIG.lock().unwrap().get(&hash).unwrap().game_path
        );
        println!("Physical Path: {}", physical_path);

        match std::fs::read(physical_path) {
            Ok(file) => {
                data[..file.len()].copy_from_slice(&file);
                Some(file.len())
            }
            Err(_err) => {
                match std::fs::read(original_file){
                    Ok(default) => {
                        data[..default.len()].copy_from_slice(&default);
        
                        Some(default.len())
                    },
                    Err(_err) => None
                }
            }
        }
    }
}

#[stream_callback]
fn stream_callback(hash: u64) -> Option<String> {
    unsafe {
        let chara_name = get_character_name(VICTORY_FIGHTER_KIND);
        let folder: String;
        if !CHARCTER_CONFIG
            .lock()
            .unwrap()
            .entries
            .contains_key(chara_name)
        {
            folder = VICTORY_FIGHTER_KIND.to_string();
        } else {
            if !CHARCTER_CONFIG
                .lock()
                .unwrap()
                .entries
                .get(chara_name)
                .unwrap()
                .id_color
                .contains_key(&VICTORY_COLOR_INDEX)
            {
                folder = CHARCTER_CONFIG
                    .lock()
                    .unwrap()
                    .entries
                    .get(chara_name)
                    .unwrap()
                    .default
                    .to_string();
            } else {
                folder = CHARCTER_CONFIG
                    .lock()
                    .unwrap()
                    .entries
                    .get(chara_name)
                    .unwrap()
                    .id_color
                    .get(&VICTORY_COLOR_INDEX)
                    .unwrap()
                    .to_string();
            }
        }

        println!("{}", folder);

        let physical_path = format!(
            "{}/{}/{}",
            STARTING_DIR,
            folder,
            &FILES_CONFIG.lock().unwrap().get(&hash).unwrap().normal_path
        );

        let original_file = format!(
            "{}/Default/{}",
            STARTING_DIR,
            &FILES_CONFIG.lock().unwrap().get(&hash).unwrap().normal_path
        );
        println!("Physical Path: {}", physical_path);

        if std::fs::metadata(&physical_path).is_ok() {
            Some(physical_path)
        }else if std::fs::metadata(&original_file).is_ok() {
            Some(original_file)
        }else {
            None
        }
    }
}

pub fn scan_folder(path: &PathBuf) {
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();

        let md = metadata(&entry.path()).unwrap();
        if !md.is_file() {
            continue;
        }

        println!("{}", entry.path().display());

        let normal_path = &format!("{}", entry.path().display())[path.to_str().unwrap().len() + 1..];
        let game_path = &normal_path.replace(";", ":").replace(".mp4", ".webm");
        let hash = hash40(&game_path).as_u64();

        if FILES_CONFIG.lock().unwrap().contains_key(&hash) {
            if FILES_CONFIG.lock().unwrap().get(&hash).unwrap().size <= md.len() {
                FILES_CONFIG.lock().unwrap().get_mut(&hash).unwrap().size = md.len();
            }
        } else {
            let path_entry = EntryInfo {
                normal_path: normal_path.to_string(),
                game_path: game_path.to_string(),
                size: md.len(),
            };

            FILES_CONFIG.lock().unwrap().insert(hash, path_entry);
        }
    }
}

pub fn inital_setup() {
    read_config_file();
    match std::fs::read_dir(STARTING_DIR) {
        Ok(paths) => {
            for x in paths {
                let x = x.unwrap();

                let md = metadata(&x.path()).unwrap();

                if md.is_file() {
                    continue;
                }

                scan_folder(&x.path());
            }
        }
        Err(err) => println!("Inital Setup Error!: {}", err),
    }
}

#[skyline::main(name = "character-result")]
pub fn main() {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );
    }

    inital_setup();
    
    for (k, v) in FILES_CONFIG.lock().unwrap().iter() {
        println!(
            "[] Key: {:#x}\nInfo: {:?}\n-------------------------------",
            k, v
        );

        if v.game_path.contains("stream:") {
            stream_callback::install(hash40(&v.game_path));
        } else {
            file_callback::install(hash40(&v.game_path), v.size as usize);
        }
    }
    
    println!(
        "Character Config:\n{:?}\n----------------------",
        CHARCTER_CONFIG.lock().unwrap().entries
    );
    
    acmd::add_custom_hooks!(once_per_fighter_frame);
}
