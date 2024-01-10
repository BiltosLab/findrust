use std::{fs,env, process::exit};

use colored::*;

fn main() {
    let args:Vec<String>= env::args().collect();

    if args.len() == 3{
        list_all(args[1].as_str(),args[2].as_str(),true);
    }
    else if args.len() == 4 && args[3].contains("-showhidden"){
        list_all(args[1].as_str(),args[2].as_str(),false);
    }
    else if args.len() > 4 || args.len() < 3  {
        println!("Usage findit <PATH> <KEYWORD> OPTIONAL<-showhidden>");
        exit(1);
    }    
    println!("DONE!");
}

fn list_all(realpath:&str,search:&str,hidden:bool){
   // let mut arrtest:String = String::new();

    if let Ok(dirf) = fs::read_dir(realpath)
    {
        for path in dirf{
            if let Ok(path) = path {
                if let Ok(metta) = path.metadata(){
                    if path.file_name().to_str().unwrap().contains(search) {
                        println!("{}", format!("{:}", path.path().to_str().unwrap()).bright_red());

                    }
                    if metta.is_dir(){
                        if path.file_name().to_str().unwrap().starts_with(".") && hidden{
                            continue;
                        }
                        //println!("{:?}",path.path().to_str().unwrap());
                        list_all(&path.path().to_string_lossy(),search,hidden);
                    }
                    else if metta.is_file() {
                        //println!("{:?}",path.path().to_str().unwrap());

                    }
                }
                
            }
            
        }
    }

}
