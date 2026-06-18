use std::{ io::stdin};

use crate::{resources::paths::CACHE_PATHS, utils::scan::{resolve_paths, scan_dir}};



mod resources;
mod utils;

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
            let resolved_path = resolve_paths(path.path);
        
            let (files,folders,size) = scan_dir(&resolved_path, path.name);
           
             stats.files += files;
             stats.folders += folders;
             stats.size += ((size as f64 / 1024.0 / 1024.0 / 1024.0) * 100.0 ).round() / 100.0;


      }

      println!("\n Total Cache found: \n \n Files : {} \n Folders : {} \n Total Size : {} GB  \n time : 0.6ms" , stats.files , stats.folders , stats.size);
      
      println!("\n Do u want to clear all cache data ? \n \x1B[35m[info]:\x1B[0m  press -yt to see  where files are been deleted from \n (y/n)?");
      stdin()
      .read_line(&mut input)
      .expect("Failed toad line");
   
   if input.trim() == "y" {
        println!("yes");
   }else if input.trim() == "n" {
        println!("No")
   }


}


