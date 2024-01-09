use std::{fs};

fn main() {
    let realpath = "./";
    list_all(&realpath);
}

fn list_all(realpath:&str){
    if let Ok(dirf) = fs::read_dir(realpath)
    {
        for path in dirf{
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata(){
                    if metta.is_dir(){
                        if path.file_name().to_string_lossy() == ".git" || path.file_name().to_string_lossy() == "target" {
                            continue;
                        }
                        println!("{}",path.file_name().to_string_lossy());
                        list_all(&path.path().to_string_lossy());
                    }
                    else if metta.is_file() {
                        println!("{}",path.file_name().to_string_lossy());
                    }
                }
                
            }
            
        }
    }

}
