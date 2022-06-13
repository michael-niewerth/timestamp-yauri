#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::env;
use std::{error::Error, time::SystemTime};
use std::fs::OpenOptions;
use csv::Writer;
use std::path::Path;
use std::fs::File;

fn main() {
  let active_id :u8 = 0;
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![btn])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn btn(id: u8) -> Result<u8, u8> {
  if active_id.equals(0:u8){
    active_id = id;
    wrt_id_start(id);
  }else if active_id.equals(id) {
    active_id=0;
    wrt_end(id);
  }else {
    wrt_end(active_id);
    wrt_id_start(id);
  }
  Ok(active_id)
}


fn wrt_id_start(id:u8){
  if !Path::new("./data.csv").exists(){
    createfile();
  }
  let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("data.csv")
      .unwrap();
  let mut writer = Writer::from_writer(file);
  writer.write_field(id.to_string())?;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
      Ok(n) => writer.write_field((n.as_secs().to_string()))?,
      Err(_) => {
        writer.write_field("time_error lmao")?;
        panic!("SystemTime before UNIX EPOCH!");
      },
    }


}

fn wrt_end(id:u8){
  if !Path::new("./data.csv").exists(){
    createfile();
  }
  let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("data.csv")
      .unwrap();
  let mut writer = Writer::from_writer(file);
  writer.write_record(&[(match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => n.as_secs(),
    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
  }.to_string())])?;
  writer.flush()?;
  Ok(())
}


fn createfile() {
  let mut file = File::create("data.csv")
      .expect("Error encountered while creating file!");
}



