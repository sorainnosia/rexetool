use std::fs::File;
use std::io::Read;
use std::ops::Index;
mod between;

fn library(buf: &Vec<u8>, start: String, end: String) -> Vec<String> {
    let mut prog_names: Vec<String> = std::env::args().collect();
    prog_names.remove(0);

    let mut prog_name = String::from("");
    if prog_names.len() > 0 {
        prog_name = between::get_filename_without_extension(prog_names[0].to_string());
    }

    let mut result = vec![];
    let s4 = between::between(&buf, start, end, 0, true);
    for i in 0..s4.len() {
        if s4[i].len() < 50 {
            let str = s4[i].to_string();
            let mut contain = false;
            if str.to_string() == "main".to_string() || str.to_string() == "untrusted".to_string() || str.to_uppercase() == prog_name.to_uppercase() {
                contain = true;
            }

            if contain == false {
                let mut x = between::index_of(&str, "\\".to_string(), 0, true);
                let y = between::index_of(&str, "/".to_string(), 0, true);
                if x != -1 && y != -1 && y < x { x = y; }
                if x == -1 { x = y; }

                if x > 0 {
                    let mut lib_name = between::substring(&str, x + 1, str.len() as i32 - x - 1);
                    let mut x2 = between::index_of(&lib_name, "\\".to_string(), 0, true);
                    let y2 = between::index_of(&lib_name, "/".to_string(), 0, true);
                    if x2 != -1 && y2 != -1 && y2 < x2 { x2 = y2; }
                    if x2 == -1 { x2 = y2; }

                    if x2 != -1 {
                        lib_name = between::substring(&lib_name, 0, x2);
                        if result.contains(&lib_name) == false {
                            result.push(lib_name.to_string());
                        }
                    }
                }
            }       
        }
    }

    result.sort();
    let mut result2 = vec![];
    for r in result {
        println!("- {}", r.to_string());
        result2.push(r.to_string());
    }
    
    return result2;
}

fn print_help() {
    println!("{}", "rexetool 0.1.0");
    println!("");
    println!("{}", "USAGE");
    println!("{}", " <Binary File path>               to check compiled rust binary and the crates it used");
}

fn main() {
    let mut  s: Vec<String> = std::env::args().collect();

    if s.len() <= 1 {
        print_help();
        return;
    }
    
    if s.len() >= 2 {
        s.remove(0);
    }

    let f = File::open(s[0].to_string());
    match f {
        Ok(mut file) => {
            let mut buf = vec![];
            let x = file.read_to_end(&mut buf);
            match x {
                Ok(x) => {
                    let mut f = x as f64;
                    f = f / 1024. / 1024.;
                    println!("Loaded {} MB", f);

                    let s1 = between::between(&buf, ".cargo/registry".to_string(), "/".to_string(), 1, true);
                    if s1.len() > 0 {
                        println!("Program {} is rust binary", s[0].to_string());
                    } else {
                        let s2 = between::between(&buf, ".cargo\\registry".to_string(), "\\".to_string(), 1, true);
                        if s2.len() > 0 {
                            println!("Program {} is rust binary", s[0].to_string());
                        } else {
                            println!("This program is not rust binary");
                            return;
                        }
                    }
                    
                    println!("Crates used :");
                    _ = library(&buf, "crates.io-".to_string(), ".rs".to_string());
                },
                Err(x) => {
                    println!("{}", x);
                    return;
                }
            }
        },
        Err(x) => {
            println!("{}", x);
            return;
        }
    }
}
