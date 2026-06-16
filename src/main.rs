use std::{env, fs, io::stdin, path::Path};

use crate::resources::paths::CACHE_PATHS;



mod resources;

struct Display {
     files:u32,
     folders:u32,
     size:f64
}

fn main(){
     let mut stats  = Display {
          files:0,
          folders:0,
          size:0.0
     };

     let mut input = String::new();

      for path in &CACHE_PATHS {
            let resolved_path = resolve_path(path.path);
            let (files,folders,size) = scan_dir(&resolved_path);

             stats.files += files;
             stats.folders += folders;
             stats.size += size as f64 / 1024.0 / 1024.0 / 1024.0;


      }

      println!("\n Total Cache found: \n \n Files : {} \n Folders : {} \n Total Size : {} GB " , stats.files , stats.folders , stats.size);
      
      println!("\n Do u want to clear all cache data ? \n \x1B[35m[info]:\x1B[0m  press -yt to see  where files are been deleted from \n (y/n)?");
      stdin()
      .read_line(&mut input)
      .expect("Failed toad line");

   if input == "y" {
        println!("yes");
   }else if input == "n" {
        println!("No")
   }


}


fn scan_dir(path: &str)->(u32,u32,f32){
       let dir = Path::new(&path); 

       if !dir.exists() {
            return  (0,0,0.0);
       }

       if !dir.is_dir() {
            return (0,0,0.0);
       }

       let mut files:u32 = 0;
       let mut folders:u32 = 0;
       let mut size:u32 = 0;
       
       let count = match  fs::read_dir(dir) {
            Ok(entry)=> { 
                 println!("\n \x1B[32m[READ]:\x1B[0m Entry found {}  " , path );
                entry }
            ,
         
            Err(e) =>{
                 eprintln!("\n \x1B[31m[ERROR]:\x1B[0m Cannot Read {} , {} " , path , e);
                 return (0,0,0.0);
            }
       };


       for entry in  count.filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                  folders += 1 ;
            }else if entry.path().is_file() {
                   files += 1;
            }

            if let Ok(meta) = entry.metadata() {
                 size += meta.len() as u32;
            }
       }


       return (files , folders , size as f32)


}

fn resolve_path(path: &str) -> String{
      
      let user = env::var("USERNAME")
      .or_else(|_| env::var("USER"))
      .unwrap_or_default();
      
      return path.replace("<user>", &user );
}