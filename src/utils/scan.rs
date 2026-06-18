use std::{ env, fs::{self}, path::{ Path}};







pub fn scan_dir(path: &str , pathname : &str)->(u32,u32,f32){
       let dir = Path::new(&path); 

       if !dir.exists() {
        //   eprintln!("\n \x1B[31m[ERROR]:\x1B[0m Directory not found {}  " , path);
            return  (0,0,0.0);
       }

       if !dir.is_dir() {
            //  eprintln!("\n \x1B[31m[ERROR]:\x1B[0m Invalid paths {}  " , path);
            return (0,0,0.0);
       }

       let mut files:u32 = 0;
       let mut folders:u32 = 0;
       let mut size:f64 = 0.0;
             
       let count = match  fs::read_dir(dir) {
            Ok(entry)=> { 

                //  println!("\n \x1B[32m[READ]:\x1B[0m Entry found {}  " , path );
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
                 size += meta.len() as f64;
            }
       }

      println!("\n {pathname} \n Files : {files} \n Folders : {folders} \n Size : {}GB " ,( (size / 1024.0 / 1024.0 /1024.0) * 100.0 ).round() / 100.0 );
       return (files , folders , size.floor() as f32)


}


pub fn resolve_paths(path: &str)-> String{
       let user: String = env::var("USERNAME")
       .or_else(|_| env::var("USER"))
       .unwrap_or_default();

      return path.replace("<user>", &user);
}
 