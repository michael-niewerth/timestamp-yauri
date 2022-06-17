#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
use std::env;
use tauri;
use std::{error::Error, time::SystemTime};
use std::fs::OpenOptions;
use csv::Writer;
use std::path::Path;
use std::fs::File;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![btn])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn btn(id: u8,activeId:u8){
  if activeId == 0{
    println!("start");
    wrt_id_start(id);
  }else if activeId ==id {
    println!("end");
    wrt_end();
  }else {
    println!("end,start");
    wrt_end();
    wrt_id_start(id);
  }
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
  writer.write_field(id.to_string()).expect("");
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
      Ok(n) => writer.write_field((n.as_secs().to_string())),
      Err(_) => {
        writer.write_field("time_error lmao").expect("");
        panic!("SystemTime before UNIX EPOCH!");
      },
    }.expect("");
  writer.flush().expect("");
}

fn wrt_end(){
  if !Path::new("./data.csv").exists(){
    createfile();
  }
  let file = OpenOptions::new()
      .write(true)
      .append(true)
      .open("data.csv")
      .unwrap();
  let mut writer = Writer::from_writer(file);
  writer.write_field("");
  writer.write_field((match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(n) => n.as_secs(),
    Err(_) => panic!("SystemTime before UNIX EPOCH!"),
  }.to_string())).expect("");
  writer.write_record(None::<&[u8]>).expect("");
  writer.flush().expect("");

}


fn createfile() {
  println!("createfile");
  let mut file = File::create("data.csv")
      .expect("Error encountered while creating file!");
}



