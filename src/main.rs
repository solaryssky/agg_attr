extern crate walkdir;
use walkdir::WalkDir;
use std::fs;
use std::env;



// cargo run -- 3 ',' /home/dima/tmp/1/ /tmp/out.txt

fn main() {
          
    let args: Vec<String> = env::args().collect();       
    let mut _string_from_char = String::new();

    let position = &args[1];
    let delimeter_str = &args[2];
    let delimeter = delimeter_str.chars().next().expect("string is empty");  
    let _path_in = &args[3]; 
    let _path_out = &args[4]; 
    let number: i8 = match position.parse() {
        Ok(n) => {
            n
        },
        Err(_) => {
            eprintln!("error: second argument not an integer");
            return;
        },
    };


    for e in WalkDir::new(_path_in).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            let fname = e.path();
            let _contents = fs::read_to_string(fname).expect("Should have been able to read the file");
            let mut i = 1;
           
        for found in _contents.chars(){   
            if found == delimeter{ 
                i += 1;    
            }

            if found == '\n'{ 
                i = 1;     
            }

            if i == number{

              //println!("{} {}", found, i);
              _string_from_char.push(found);
             

              //println!("{}", _string_from_char);
        }
                   
                   
                                                   

            }   

            
            let mut _strings: Vec<&str> = _string_from_char.split(",").collect();
                _strings.sort();
                _strings.dedup();
                _strings.retain(|value| *value != "");
            
            //println!("{:?}", _strings);
            fs::write(_path_out, _strings.join("\n")).expect("");


        }

    }

    
}