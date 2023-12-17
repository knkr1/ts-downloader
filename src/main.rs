use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::process::Command;

fn create_ts_files_directory() {
    fs::create_dir_all("ts_files").expect("Failed to create ts_files directory");
}

fn get_user_settings() -> (String, i32, i32) {
    let mut link = String::new();
    let mut start_range = String::new();
    let mut end_range = String::new();

    println!("Enter the link:");
    io::stdin().read_line(&mut link).expect("Failed to read line");
    
    println!("Enter the start range:");
    io::stdin().read_line(&mut start_range).expect("Failed to read line");
    let start_range: i32 = start_range.trim().parse().expect("Invalid start range");

    println!("Enter the end range:");
    io::stdin().read_line(&mut end_range).expect("Failed to read line");
    let end_range: i32 = end_range.trim().parse().expect("Invalid end range");

    (link.trim().to_string(), start_range, end_range)
}


fn save_settings(link: &str, start_range: i32, end_range: i32) {
    let settings = format!("{},{},{}", link, start_range, end_range);

    let mut settings_path = std::env::current_exe().unwrap();
    settings_path.set_file_name(".ts_downloader_settings.txt");

    fs::write(&settings_path, settings).expect("Failed to write settings to file");
}

fn load_settings() -> Option<(String, i32, i32)> {
    let mut settings_path = std::env::current_exe().unwrap();
    settings_path.set_file_name(".ts_downloader_settings.txt");

    if let Ok(contents) = fs::read_to_string(&settings_path) {
        let mut parts = contents.split(',').map(|s| s.trim());

        let link = parts.next().unwrap_or_default().to_string();
        let start_range: i32 = parts.next().unwrap_or_default().parse().unwrap_or(0);
        let end_range: i32 = parts.next().unwrap_or_default().parse().unwrap_or(0);

        Some((link, start_range, end_range))
    } else {
        None
    }
}


fn download_ts_files(link: &str, start_range: i32, end_range: i32) {
    create_ts_files_directory(); // Ensure ts_files directory exists

    for i in start_range..=end_range {
        let url = link.replace("{}", &i.to_string());
        let response = reqwest::blocking::get(&url);

        match response {
            Ok(mut res) => {
                if res.status().is_success() {
                    let mut file = File::create(format!("ts_files/file{}.ts", i))
                        .expect("Failed to create file");
                    file.write_all(&res.bytes().expect("Failed to read response content"))
                        .expect("Failed to write to file");
                    println!(
                        "Downloaded file{}.ts | %{:.2}",
                        i,
                        ((i - start_range) as f64 / (end_range - start_range + 1) as f64) * 100.0
                    );
                } else {
                    println!("Failed to download file{}.ts", i);
                }
            }
            Err(err) => {
                println!("Error downloading file{}.ts: {}", i, err);
            }
        }
    }
}






fn merge_and_convert_to_mp4(start_range: i32, end_range: i32) {
    let file_list: Vec<String> = (start_range..=end_range)
        .map(|i| format!("ts_files/file{}.ts", i))
        .collect();
    let temp_file_list_path = "temp_file_list.txt";

    // Write the list of input files to a temporary file
    let mut temp_file =
        File::create(temp_file_list_path).expect("Failed to create temporary file");
    for file_path in &file_list {
        writeln!(temp_file, "file '{}'", file_path).expect("Failed to write to temporary file");
    }

    let output_file = "output.mp4";

    // Run ffmpeg command using the temporary file list
    let command = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg(temp_file_list_path)
        .arg("-c")
        .arg("copy")
        .arg("-bsf:a")
        .arg("aac_adtstoasc")
        .arg(output_file)
        .spawn();

    match command {
        Ok(mut child) => {
            let status = child.wait().expect("Failed to wait for ffmpeg command");
            if status.success() {
                println!("Merged and converted files into {}", output_file);
            } else {
                println!("Failed to merge and convert files");
            }
        }
        Err(err) => {
            println!("Error running ffmpeg command: {}", err);
        }
    }

    // Remove the temporary file list
    fs::remove_file(temp_file_list_path).expect("Failed to remove temporary file");
}

fn main() {
    loop {
        // Check if settings file exists and load settings
        if let Some((link, start_range, end_range)) = load_settings() {
            println!("\n\nSettings loaded: Link: {} \nStart Range: {}\nEnd Range: {}\n\n", link, start_range, end_range);
            loop {
                println!("Enter 1 to download .ts files, 2 to merge and convert to MP4, 3 to change settings, 4 to exit:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");

                match input.trim().parse() {
                    Ok(1) => download_ts_files(&link, start_range, end_range),
                    Ok(2) => merge_and_convert_to_mp4(start_range, end_range),
                    Ok(3) => {
                        let (new_link, new_start_range, new_end_range) = get_user_settings();
                        save_settings(&new_link, new_start_range, new_end_range);
                        println!("Settings updated.");
                    }
                    Ok(4) => {
                        println!("Exiting program. Goodbye!");
                        return; // Exit the inner loop and continue to the outer loop
                    }
                    _ => {
                        println!("Invalid choice. Please enter 1, 2, 3, or 4.");
                    }
                }
            }
        } else {
            println!("Settings not found. Please set the settings first.");
            let (link, start_range, end_range) = get_user_settings();
            save_settings(&link, start_range, end_range);
            println!("Settings saved. Returning to the menu.");
        }
    }
}

