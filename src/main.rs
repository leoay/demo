use whoami;
use std::process::Command;

fn extractby7z(from: &String, to: &String) {
    let cmm: String = "C:/Program\\ Files/7-Zip/7z.exe x ".to_string() + from + " -o" + to;
    print!("{}", cmm);
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", &cmm])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
    };
    print!("{:?}", output.stderr.to_ascii_lowercase())
}


fn main() {
    let username = whoami::username();
    let mypath: String = "C:/Users/".to_string() + &username + &"/AppData/Local/LiteDSP".to_string();
    //print!("{:?}", mypath);

    let filepath: String = "C:/Users/leoay/Downloads/DSP_CAPA_JSON.zip".to_string();

    extractby7z(&filepath, &mypath);
    
    // let tarpath = format!("C:/Users/{:?}/AppData/Roaming", username);
    // print!("{:?}", tarpath);
}
