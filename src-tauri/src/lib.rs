pub mod sysfunc {
    use std::{fs};
    use directories::{ProjectDirs};

    fn create_directories() {
        let dir = ProjectDirs::from("com", "Eisvogel", "Momentum").unwrap();

        let prj_data = dir.data_dir();
        let prj_conf = dir.config_dir();
        let prj_cach = dir.cache_dir();

        match fs::create_dir_all(&prj_data) {
            Ok(_) => println!("DEBUG: Momentum Data File created successfully"),
            Err(e) => println!("ERROR: Error creating directory in {}: {}", prj_data.display(), e),
        }
    
        match fs::create_dir_all(&prj_conf) {
            Ok(_) => println!("DEBUG: Momentum Config File created successfully"),
            Err(e) => println!("ERROR: Error creating directory in {}: {}", prj_conf.display(), e),
        }
    
        match fs::create_dir_all(&prj_cach) {
            Ok(_) => println!("DEBUG: Momentum Cache File created successfully"),
            Err(e) => println!("ERROR: Error creating directory in {}: {}", prj_cach.display(), e),
        }
    }

    fn create_new_files() {
        let dir = ProjectDirs::from("com", "Eisvogel", "Momentum").unwrap();

        let prj_data = dir.data_dir();
        let prj_conf = dir.config_dir();
        let prj_cach = dir.cache_dir();

            // Creates blocklist files
        let prj_blocklist = prj_data.join("blocklist.json");

        fs::File::create(&prj_blocklist).unwrap();

        fs::write(&prj_blocklist, r#"[
            {
                "If this message still exists, something went terribly wrong. Delete the DO_NOT_DELETE file to attempt a fresh reset."
            }
        ]"#).ok();

        // Creates config files
        let prj_settings = prj_conf.join("config.json");

        fs::File::create(&prj_settings).unwrap();

        fs::write(&prj_settings, r#"{
            "preferences":{
                "whitelist": true,
                "hyperfocus": true,
                "forceExit": true
            },
            "Blocker Settings":{
                "helloBlock": "yes"
            },
            "Notification Settings":{
                "helloNotif": "yes"
            },
            "Interface Settings":{
                "helloInter": "yes"
            },
            "Database Settings":{
                "helloData": "yes"
            }
        }"#).ok();

        // Creates cache files
        let prj_local = prj_cach.join("volatileProgramData.json");

        fs::File::create(&prj_local).unwrap();

        fs::write(&prj_local, r#"[
            {
                "threadName": "\\0",
                {
                    "pid": "\\0",
                    "path": "\\0"
                }
            }
        ]"#).ok();

        // Creates identifier file
        let prj_settings = prj_conf.join("DO_NOT_DELETE");

        fs::File::create(&prj_settings).unwrap();

        fs::write(&prj_settings, "Do not delete this file! Deleting this file will cause the program to reset everything, removing all your settings, blocklist, etc. Also useful for hard resets.").ok();

    }

    pub fn fresh_install() {
        create_directories();
        create_new_files();
    }

    pub fn check_identifier() {
        let dir = ProjectDirs::from("com", "Eisvogel", "Momentum").unwrap();
        let check_identifier = &dir.config_dir().join("DO_NOT_DELETE");

        if let Ok(_metadata) = fs::metadata(check_identifier) {
            println!("Files have been found in {}! Skipping fresh install!", check_identifier.display());
        } else {
            fresh_install();
        }
    }
}

pub mod scan {
    use sysinfo::{SystemExt, ProcessExt, Pid};
    use winapi::{um::{winuser::{GetForegroundWindow, ShowWindow, SW_MINIMIZE}}};
    use crate::functsys::read_blocklist_raw;

    pub fn persistent_scan() {
        let mut sys = sysinfo::System::new_all();
        let mut counter = 0;
        let mut container = String::new();
        let mut pscan_blocklist = Vec::<String>::new();
        let blocklist = read_blocklist_raw();

        // Gets blocklist
        for item in blocklist {
            container = item.threadName.clone();
            pscan_blocklist.push(container);
        }

        println!("DEBUG: This is the current blocklist: {:?}", pscan_blocklist);

        // This does the scanning
        loop {
            sys.refresh_all();
            for element in &pscan_blocklist {
                for (pid, process) in sys.processes() {
                    if process.name() == element {
                        println!("DEBUG: {} with PID ({}) has been found in the process list via comparison to {}!", process.name(), pid, element);

                        let mut _pid = process.pid();

                        if let Some(process) = sys.process(Pid::from(_pid)) {
                            if crate::config::read_forceexit() {
                                
                                // TODO PAYLOAD
                                // act_on_program_pid(_pid);

                            } else {
                                println!("TODO Timer to alert user of {} ({}).", process.name(), _pid);
                            }
                        }
                    }
                }
            }
            
            println!("Counter at {}", counter);
            counter += 1;
            if counter == 10 {
                break
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }

    pub fn persistent_foreground() {
        let mut counter = 0;
        let mut container = String::new();
        let mut pscan_blocklist = Vec::<String>::new();
        let blocklist = read_blocklist_raw();

        // Gets blocklist
        for item in blocklist {
            container = item.threadName.clone();
            pscan_blocklist.push(container);
        }

        println!("DEBUG: This is the current blocklist: {:?}", pscan_blocklist);

        // This does the scanning
        loop {
            let foreground_name = crate::functsys::read_foreground();

            for element in &pscan_blocklist {
                    if element == &foreground_name {
                        println!("TODO PAYLOAD")
                    // act_on_program(unsafe{ GetForegroundWindow() })
                }
            }
            println!("Counter at {}", counter);
            counter += 1;
            if counter == 20 {
            break
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}

mod blocker {
    use sysinfo::PidExt;
    use winapi::{shared::windef::HWND__, um::{winuser::{SW_MINIMIZE, ShowWindow}, processthreadsapi::{OpenProcess, TerminateProcess}, winnt::PROCESS_ALL_ACCESS, handleapi::CloseHandle}};
    use windows_win::raw::{process::get_id, window::get_thread_process_id};
    use std::ptr::null_mut;
    use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId};
    use winapi::um::winnt::HANDLE;

    pub fn act_on_program(hwnd: *mut HWND__) {
        if crate::config::read_forceexit() {
            println!("DEBUG: Program has been found! Attempting to close.");
            force_exit(hwnd);
        } else {
            println!("DEBUG: Program has been found! Attempting to minimize.");
            force_minimize(hwnd);
        }
    }

    pub fn act_on_program_pid(pid: sysinfo::Pid) {
        if crate::config::read_forceexit() {
            println!("DEBUG: Program has been found! Attempting to close.");
            force_exit_pid(pid);
        } else {
            println!("DEBUG: Not yet programmed. (Line 222, librs)");
            // force_minimize(pid);
        }
    }

    fn force_exit(hwnd: *mut HWND__) {

        let pid = get_thread_process_id(hwnd).0;

        let process_handle = unsafe {
            OpenProcess(PROCESS_ALL_ACCESS, 0, pid)
        };
        if process_handle == std::ptr::null_mut() {
            println!("Error getting process handle");
            return;
        }
        let result = unsafe {
            TerminateProcess(process_handle, 0)
        };
        if result == 0 {
            println!("Error sending shutdown signal");
            return;
        }
        let result = unsafe {
            CloseHandle(process_handle)
        };
        if result == 0 {
            println!("Error closing process handle");
            return;
        }
    }

    fn force_exit_pid(pid_raw: sysinfo::Pid) {

        let pid = pid_raw.as_u32();

        let process_handle = unsafe {
            OpenProcess(PROCESS_ALL_ACCESS, 0, pid)
        };
        if process_handle == std::ptr::null_mut() {
            println!("Error getting process handle");
            return;
        }
        let result = unsafe {
            TerminateProcess(process_handle, 0)
        };
        if result == 0 {
            println!("Error sending shutdown signal");
            return;
        }
        let result = unsafe {
            CloseHandle(process_handle)
        };
        if result == 0 {
            println!("Error closing process handle");
            return;
        }
    }

    fn force_minimize(hwnd: *mut HWND__) {
        unsafe { ShowWindow(hwnd, SW_MINIMIZE) };
    }
}

pub mod functsys {
    use serde_json::{Value, from_reader};
    use sysinfo::{SystemExt, ProcessExt};
    use windows_win::raw::window::get_text;
    use std::{ptr, fs, fs::File, ffi::OsString, os::windows::prelude::OsStringExt};
    use winapi::{um::{winuser::{GetForegroundWindow, GetWindowThreadProcessId, IsWindow}, processthreadsapi::{OpenProcess}, winnt::{PROCESS_QUERY_INFORMATION}, psapi::GetProcessImageFileNameW, tlhelp32::{PROCESSENTRY32, Process32First, CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS, Process32Next}}, shared::minwindef::MAX_PATH};
    use directories::ProjectDirs;
    use serde::{Deserialize, Serialize};

    use crate::winapi_funcs::get_hwnd_by_pid;

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct BlockEntry {
        pub threadName: String,
        inner: BlockInner
    }

    #[derive(Serialize, Deserialize, Clone, Debug)]
    struct BlockInner {
        pid: i32,
        path: std::path::PathBuf
    }
    
    pub fn get_process_info() -> Vec<String> {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        let mut list = Vec::<String>::new();
    
        for (pid, process) in sys.processes() {
            let pidu32 = sysinfo::PidExt::as_u32(pid.to_owned());
            let hwnd = get_hwnd_by_pid(pidu32);
            let is_window = unsafe{ IsWindow(hwnd) };
            if is_window == 1 && !process.name().contains("explorer.exe") && !process.root().display().to_string().contains("System32") && !process.root().display().to_string().contains("SystemApp") {
    
                let proc_name: String;
                if get_text(hwnd).is_ok() {
                    proc_name = get_text(hwnd).unwrap();
                } else {
                    proc_name = "None".to_string();
                }
    
                if proc_name != "None".to_string() {
                    list.push(proc_name.to_string() + " [" + process.name() + "]")
                }
            }
        }
        return list;
    }

    pub fn get_processes() -> Vec<String> {
        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
        let mut list = Vec::<String>::new();

        for (pid, process) in sys.processes() {
            if process.user_id() != None && !process.root().display().to_string().contains("System32") && !process.name().contains("explorer.exe") && !process.root().display().to_string().contains("SystemApp") && !list.contains(&process.name().to_string()) {
                list.push(process.name().to_string())
            }
            
        }
        return list;
    }

    fn get_foreground() -> String {
        // Get the handle of the foreground window
        let foreground_window = unsafe { GetForegroundWindow() };

        // Get the process ID of the foreground window
    
        let mut process_id = 0;
        unsafe {
            GetWindowThreadProcessId(foreground_window, &mut process_id);
        }
    
        let h_process = unsafe { OpenProcess(PROCESS_QUERY_INFORMATION, 0, process_id) };
    
        if h_process == ptr::null_mut() { println!("Failed to open handle to process with PID {}", process_id) }
    
        let mut file_name = [0u16; MAX_PATH];
        let success = unsafe { GetProcessImageFileNameW(h_process, file_name.as_mut_ptr(), file_name.len() as u32) };
    
        if success == 0 {
            println!("DEBUG: Cannot get file name for pid {}", process_id);
            return "".to_string();
        } else {
            // println!("'{:?}'", String::from_utf16_lossy(& file_name).trim_matches('\0'));
            return String::from_utf16_lossy(& file_name).trim_matches('\0').to_owned();
        }
    }

    pub fn read_foreground() -> String {
        let foreground = get_foreground();
        let foreground_split: Vec<&str> = foreground.split("\\").collect();
        let foreground_name = foreground_split.last().unwrap();
        let foreground_string = foreground_name.to_string();
        return foreground_string;
    }

    fn get_blocklist() -> Vec<BlockEntry> {
        let dir = ProjectDirs::from("com", "Eisvogel", "Momentum").unwrap();
        let datadir = dir.data_dir();
        let file = File::open(datadir.join("blocklist.json")).expect("File not found!");
        let contents: Vec<BlockEntry> = match from_reader(file) {
            Ok(contents) => contents,
            Err(error) => {
                println!("Cannot parse JSON: {}", error);
                println!("Recreating database file!");
                Vec::new()
            }
        };

        return contents;
    }

    pub fn read_blocklist_raw() -> Vec<BlockEntry> {
        return get_blocklist();
    }

    pub fn read_blocklist() -> Vec<String> {
        let blocklist_raw = get_blocklist();
        let mut blocklist = Vec::<String>::new();
        let mut container = String::new();
        for item in blocklist_raw {
            container = item.threadName.clone();
            blocklist.push(container);
        }

        blocklist
    }

    pub fn add_blockentry(add_app: &str) {
        let dir = ProjectDirs::from("com", "Eisvogel", "Momentum").unwrap();
        let datadir = dir.data_dir();
        let file = File::open(datadir.join("blocklist.json")).expect("File not found!");
        let mut contents: Vec<BlockEntry> = match from_reader(file) {
            Ok(contents) => contents,
            Err(error) => {
                println!("Cannot parse JSON: {}", error);
                println!("Recreating database file!");
                Vec::new()
            }
        };

        let new_entry = BlockEntry {
            threadName: add_app.to_owned(),
            inner: BlockInner { pid: 0, path: dir.data_dir().to_path_buf() }
        };

        contents.push(new_entry);

        let file = File::create(datadir.join("blocklist.json")).expect("File not found!");
        serde_json::to_writer_pretty(file, &contents).unwrap();
    }

    pub fn del_blockentry(del_app: &str) {
        let dir = ProjectDirs::from("com", "Eisvogel", "Momentum").unwrap();
        let datadir = dir.data_dir();
        let file = File::open(datadir.join("blocklist.json")).expect("File not found!");
        let mut new_array = Vec::<BlockEntry>::new();
        let outer_array: Vec<BlockEntry> = match from_reader(file) {
            Ok(outer_array) => outer_array,
            Err(error) => {
                println!("Cannot parse JSON: {}", error);
                Vec::new()
                
            }
        };

        for (_i, arg) in outer_array.iter().enumerate() {
            if arg.threadName != del_app {
                let temp_entry = BlockEntry {
                    threadName: arg.threadName.to_owned(),
                    inner: BlockInner { 
                        pid: arg.inner.pid.to_owned(),
                        path: arg.inner.path.to_owned()
                    }
                };
    
                new_array.push(temp_entry);
            }
        } 

        let file = File::create(datadir.join("blocklist.json")).expect("File not found!");
        fs::File::create(datadir.join("blocklist.json")).unwrap();
        serde_json::to_writer_pretty(file, &new_array).unwrap();
    }
}

pub mod config {
    use std::{fs::File, io::Read};
    use directories::ProjectDirs;
    use serde_json::Value;

    pub fn get_config_data(get_category: &str, get_params: &str) -> bool {
        let dir = ProjectDirs::from("com", "Eisvogel", "Momentum").unwrap();
        let conf_dir = dir.config_dir();
        let mut file = File::open(conf_dir.join("config.json")).expect("File not found!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file.");
    
        let json: Value = serde_json::from_str(&contents).expect("Failed to parse config.json");
        
        if let Some(category) = json.get(get_category) {
            // println!("DEBUG: category: {}: {}", get_category, category);
            if let Some(setting) = category.get(get_params) {
                // println!("DEBUG: parameter: {}: {}", get_params, setting);
                return setting.as_bool().unwrap();
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    pub fn read_whitelist() -> bool {
        let setting = get_config_data("preferences", "whitelist");
        return setting;
    }

    pub fn read_hyperfocus() -> bool {
        let setting = get_config_data("preferences", "hyperfocus");
        return setting;
    }

    pub fn read_forceexit() -> bool {
        let setting = get_config_data("preferences", "forceExit");
        return setting;
    }
 
    pub fn write_config_data(get_category: &str, get_params: &str, write_bool: bool) {
        // println!("{}, {}, {}", get_category, get_params, write_bool);
        let dir = ProjectDirs::from("com", "Eisvogel", "Momentum").unwrap();
        let conf_dir = dir.config_dir();
        let mut file = File::open(conf_dir.join("config.json")).expect("File not found!");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Could not read file.");
    
        let mut json: Value = serde_json::from_str(&contents).expect("Failed to parse config.json");

        let conf_ptr = "/".to_owned() + get_category + "/" + get_params;

        json.pointer_mut(&conf_ptr).map(|data| *data = write_bool.into());

        let file = File::create(conf_dir.join("config.json")).expect("File not found!");
        serde_json::to_writer_pretty(file, &json).unwrap();
    }
    
    pub fn write_whitelist(write_bool: bool) {
        write_config_data("preferences", "whitelist", write_bool);
    }

    pub fn write_hyperfocus(write_bool: bool) {
        write_config_data("preferences", "hyperfocus", write_bool);
    }

    pub fn write_forceexit(write_bool: bool) {
        write_config_data("preferences", "forceExit", write_bool);
    }
}

mod winapi_funcs {
    use winapi::shared::windef::{HWND};
    use windows_win::raw::window::get_by_pid;
    
    pub fn get_hwnd_by_pid(pid: u32) -> HWND {
        let hwnd_opt = get_hwnd_opt_by_pid(pid);
    
        if hwnd_opt.is_some() {
            let hwnd = hwnd_opt.unwrap();
            return hwnd
        } else {
            return 0 as HWND;
        }
    }
    
    pub fn get_hwnd_opt_by_pid(pid: u32) -> Option<*mut winapi::shared::windef::HWND__> {
        let hwnd_raw = get_by_pid(pid);
        match hwnd_raw {
            Ok(v) => return v,
            Err(e) => {
                println!("HWND not found: {e:?}");
                return None;
                },
        }
    }
}

pub mod scanner {
    use sysinfo::{SystemExt, ProcessExt, Pid};
    use windows_win::raw::window::{get_thread_process_id, get_by_pid};

    use std::io;
    use std::time::Duration;
    use winapi::{um::winuser::{GetForegroundWindow, SetFocus}, shared::windef::HWND};

    use tokio::time;
    use tokio::time::interval;
    use async_recursion::async_recursion;
    use tokio_js_set_interval::set_interval;
    use crate::{functsys::{read_blocklist_raw, read_foreground}, config::read_hyperfocus, blocker::{act_on_program, act_on_program_pid}, winapi_funcs::get_hwnd_by_pid};
    
    #[async_recursion]
    pub async fn foreground_scanner(duration: u64) {
        let mut foreground_interval = time::interval(time::Duration::from_secs(1));
        let mut counter = 0;
        let mut container: bool;
        let mut time_elapsed = duration;

        let mut current_win = get_thread_process_id(unsafe { GetForegroundWindow() });
        let mut previous_win = current_win;


        for _i in 0..duration {
            counter += 1;
            println!("Time spent in scanner without breaking: {} seconds ({})", counter, time_elapsed);
            foreground_interval.tick().await;

            current_win = get_thread_process_id(unsafe { GetForegroundWindow() });
            previous_win = current_win;

            container = foreground_sweep();
            time_elapsed -= 1;
            if container || time_elapsed == 0 { break }
        }

        if time_elapsed == 0 {
            println!("Foreground scan complete!")
            // do after-scan functions
        } else {
            if !read_hyperfocus() {
                let mut counter = 0;
                let mut container: bool;
                for _i in 0..100 {
                    counter += 1;
                    println!("Time spent in grace timer: {} seconds", counter);
                    foreground_interval.tick().await;
                    container = foreground_sweep();
                    if !container { break }
                }
    
                // println!("Write what distracted you earlier:");
                // let mut guess = String::new();
                // io::stdin().read_line(&mut guess).expect("failed to readline");
                // print!("You entered {}", guess);
                
                println!("Returning to main scan with {} seconds to spare.", time_elapsed);
                foreground_scanner(time_elapsed).await;
            } else {
                let _dummy = hwnd_ground(force_window_focus(get_hwnd_by_pid(previous_win.0)));
                act_on_program(unsafe{ GetForegroundWindow() });
                println!("Returning to main scan with {} seconds to spare.", time_elapsed);
                foreground_scanner(time_elapsed+1).await;
            }
        }
    }

    fn hwnd_ground(hwnd: HWND) -> bool {
        return true
    }

    fn foreground_sweep() -> bool {
        let mut container = String::new();
        let mut pscan_blocklist = Vec::<String>::new();

        let blocklist = read_blocklist_raw();

        for item in blocklist {
            container = item.threadName.clone();
            pscan_blocklist.push(container);
        }

        let foreground_name = read_foreground();

        for element in &pscan_blocklist {
            if element == &foreground_name {
                return true
            }
        }

        return false
    }

    fn force_window_focus(hwnd: HWND) -> HWND {
        return unsafe{ SetFocus(hwnd) };
    }

    pub async fn background_scanner(duration: u64) {
        let mut background_interval = time::interval(time::Duration::from_secs(1));
        let mut counter = 0;
        let mut container: bool;
        let mut time_elapsed = duration;

        for _i in 0..duration {
            counter += 1;
            println!("Time spent in scanner: {} seconds", counter);
            background_interval.tick().await;
            container = background_sweep();
            time_elapsed -= 1;
            if container || time_elapsed == 0 { break }
        }
    }

    fn background_sweep() -> bool {
        let mut sys = sysinfo::System::new_all();

        let mut container = String::new();
        let mut blocklist = Vec::<String>::new();

        let blocklist_raw = read_blocklist_raw();

        for item in blocklist_raw {
            container = item.threadName.clone();
            blocklist.push(container);
        }

        let foreground_name = read_foreground();

        for element in &blocklist {
            for (pid, process) in sys.processes() {
                if process.name() == element {
                    println!("DEBUG: {} with PID ({}) has been found in the process list via comparison to {}!", process.name(), pid, element);

                    let mut _pid = process.pid();

                    if let Some(process) = sys.process(Pid::from(_pid)) {
                        if crate::config::read_forceexit() {
                            act_on_program_pid(_pid);
                        } else {
                            println!("DEBUG: User somehow managed to enable Force Exit without Hyper-Focus, and trick the program into using it.");
                        }
                    }
                }
            }
        }

        return false
    }
}