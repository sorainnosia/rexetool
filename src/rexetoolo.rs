use std::fmt::Binary;
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use indexmap::IndexMap;
mod bytesops;
use bytesops::{*};

fn crates_list(buf: &Vec<u8>, start: String, end: String) -> Vec<String> {
    let mut prog_names: Vec<String> = std::env::args().collect();
    prog_names.remove(0);

    let mut prog_name = String::from("");
    if prog_names.len() > 0 {
        prog_name = bytesops::get_filename_without_extension(prog_names[0].to_string());
    }

    let mut result = vec![];
    let s4 = bytesops::between(&buf, start, end, 0, true);
    for i in 0..s4.len() {
        if s4[i].len() < 50 {
            let str = s4[i].to_string();
            let mut contain = false;
            if str.to_string() == "main".to_string() || str.to_string() == "untrusted".to_string() || str.to_uppercase() == prog_name.to_uppercase() {
                contain = true;
            }

            if contain == false {
                let mut x = bytesops::index_of(&str.as_bytes().to_vec(), &"\\".to_string().as_bytes().to_vec(), 0, true);
                let y = bytesops::index_of(&str.as_bytes().to_vec(), &"/".to_string().as_bytes().to_vec(), 0, true);
                if y != -1 && (y < x || x == -1) { x = y; }

                if x > 0 {
                    let mut lib_name = bytesops::substring(&str, x + 1, str.len() as i32 - x - 1);

                    let mut x2 = bytesops::index_of(&lib_name.as_bytes().to_vec(), &"\\".to_string().as_bytes().to_vec(), 0, true);
                    let y2 = bytesops::index_of(&lib_name.as_bytes().to_vec(), &"/".to_string().as_bytes().to_vec(), 0, true);
                    if y2 != -1 && (y2 < x2 || x2 == -1) { x2 = y2; }

                    if x2 > 0 {
                        lib_name = bytesops::substring(&lib_name, 0, x2);

                        lib_name = bytesops::trim(lib_name.to_string());
                        lib_name = bytesops::ascii(lib_name.to_string());
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
        result2.push(r.to_string());
    }
    
    return result2;
}

fn modules_list(buf: &Vec<u8>, start: String, end: String, append: String, mode: i32) -> Vec<String> {
    let mut prog_names: Vec<String> = std::env::args().collect();
    prog_names.remove(0);

    let mut prog_name = String::from("");
    if prog_names.len() > 0 {
        prog_name = bytesops::get_filename_without_extension(prog_names[0].to_string());
    }

    let mut result = vec![];
    let s4 = bytesops::between(&buf, start, end, 0, true);
    for i in 0..s4.len() {
        if s4[i].len() < 50 {
            let str = s4[i].to_string();
            let mut contain = false;
            let mut lib_name = str.to_string();
            
            let mut x = bytesops::index_of(&lib_name.as_bytes().to_vec(), &&"*".as_bytes().to_vec(), 0, false);
            let y = bytesops::index_of(&lib_name.as_bytes().to_vec(), &"(".as_bytes().to_vec(), 0, false);
            let z = bytesops::index_of(&lib_name.as_bytes().to_vec(), &",".as_bytes().to_vec(), 0, false);
            let w = bytesops::index_of(&lib_name.as_bytes().to_vec(), &"+".as_bytes().to_vec(), 0, false);
            let v = bytesops::index_of(&lib_name.as_bytes().to_vec(), &"#".as_bytes().to_vec(), 0, false);
            let u = bytesops::index_of(&lib_name.as_bytes().to_vec(), &"-".as_bytes().to_vec(), 0, false);
            let t = bytesops::index_of_non_fn(&lib_name.as_bytes().to_vec(), 0);
            let s = bytesops::index_of(&lib_name.as_bytes().to_vec(), &"@".as_bytes().to_vec(), 0, false);
            if y != -1 && (y < x || x == -1) { x = y; }
            if z != -1 && (z < x || x == -1) { x = z; }
            if w != -1 && (w < x || x == -1) { x = w; }
            if v != -1 && (v < x || x == -1) { x = v; }
            if u != -1 && (u < x || x == -1) { x = u; }
            if t != -1 && (t < x || x == -1) { x = t; }
            if s != -1 && (s < x || x == -1) { x = s; }
            
            if x != -1 {
                lib_name = bytesops::substring(&lib_name, 0, x);
            }
            
            let mut x = bytesops::index_of_non_fn(&lib_name.as_bytes().to_vec(), 0);
            if x != -1 {
                lib_name = bytesops::substring(&lib_name, 0, x);
            }

            if mode == 2 {
                let mut x = bytesops::index_of(&lib_name.as_bytes().to_vec(), &&"/".as_bytes().to_vec(), 0, false);
                if x != -1 {
                    x = bytesops::index_of(&lib_name.as_bytes().to_vec(), &&"/".as_bytes().to_vec(), x + 1, false);
                    if x != -1 {
                        lib_name = bytesops::substring(&lib_name, 0, x);
                    }
                }
            }
            lib_name = bytesops::trim(lib_name.to_string());
            lib_name = bytesops::ascii(lib_name.to_string());

            if lib_name.to_string() == "go" { continue; }
            if result.contains(&lib_name) == false {
                result.push(lib_name.to_string());
            }
        }
    }

    result.sort();
    let mut result2 = vec![];
    for r in result {
        let r2 = format!("{}{}", append.to_string(), r.to_string());
        result2.push(r2.to_string());
    }
    
    return result2;
}

fn print_help() {
    println!("{}", "rexetool 0.3.0");
    println!("");
    println!("{}", "USAGE");
    println!("{}", " <Binary File path>               to check compiled Rust/Go binary and the crates/modules it used");
    println!("");
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum BinaryType {
    Unknown, Rust, NET, GO, CPP
}

pub fn binary_type(buf: &Vec<u8>) -> BinaryType {
    let s1 = bytesops::between(&buf, ".cargo/registry".to_string(), "/".to_string(), 1, true);
    if s1.len() > 0 {
        return BinaryType::Rust;
    } else {
        let s1 = bytesops::between(&buf, ".cargo\\registry".to_string(), "\\".to_string(), 1, true);
        if s1.len() > 0 {
            return BinaryType::Rust;
        } else {
            let mut marker_wide = IndexMap::new();
            marker_wide.insert("Installing .NET", BinaryType::NET);
            marker_wide.insert("mscorlib", BinaryType::NET);
            marker_wide.insert("hostfxr", BinaryType::NET);
            
            let mut marker_ascii = IndexMap::new();
            marker_ascii.insert("mscorlib", BinaryType::NET);
            marker_ascii.insert("Go build ID", BinaryType::GO);
            marker_ascii.insert("go.buildid", BinaryType::GO);
            marker_ascii.insert("std@@", BinaryType::CPP);
            marker_ascii.insert("msvcrt.dll", BinaryType::CPP);
            marker_ascii.insert("memcpy", BinaryType::CPP);

            for (x, y) in marker_wide.iter() {
                let w = bytesops::wide_char(x.to_string());
                let i = bytesops::index_of(buf, &w, 0, false);
                if i >= 0 { return y.clone(); }
            }

            for (x, y) in marker_ascii.iter() {
                let w = x.to_string().as_bytes().to_vec();
                let i = bytesops::index_of(buf, &w, 0, false);
                if i >= 0 { return y.clone(); }
            }
        }
    }
    return BinaryType::Unknown;
}

pub fn rexetool() {
    let mut  s: Vec<String> = std::env::args().collect();

    if s.len() <= 1 {
        print_help();
        return;
    }
    
    if s.len() >= 2 { s.remove(0); }

    let f = File::open(s[0].to_string());
    match f {
        Ok(mut file) => {
            let mut vec1 = vec![];
            let mut buf = vec![];
            let x = file.read_to_end(&mut buf);
            match x {
                Ok(x) => {
                    let mut f = x as f64;
                    f = f / 1024. / 1024.;
                    println!("Loaded : {:.2} MB", f);

                    let btype = binary_type(&buf);
                    if btype == BinaryType::Rust {
                        println!("Program '{}' : Rust binary", bytesops::get_filename(s[0].to_string()));

                        println!("Crates used :");
                        vec1 = crates_list(&buf, "crates.io-".to_string(), ".rs".to_string());    
                    } else if btype == BinaryType::CPP {
                        println!("Program '{}' : C++ binary", bytesops::get_filename(s[0].to_string()));
                    } else if btype == BinaryType::NET {
                        println!("Program '{}' : NET binary", bytesops::get_filename(s[0].to_string()));
                    } else if btype == BinaryType::GO {
                        println!("Program '{}' : Go binary", bytesops::get_filename(s[0].to_string()));

                        println!("Modules used :");
                        vec1 = modules_list(&buf, "golang_org/x/".to_string(), ".".to_string(), String::from(""), 1);
                        let mut vec2 = modules_list(&buf, "golang.org/x/".to_string(), ".".to_string(), String::from(""), 1);
                        let mut vec3 = modules_list(&buf, "github.com/".to_string(), ".".to_string(), "github.com/".to_string(), 2);
                        vec1.append(&mut vec2);
                        vec1.append(&mut vec3);
                    } else if btype == BinaryType::Unknown {
                        println!("Program '{}' : Unknown binary", bytesops::get_filename(s[0].to_string()));
                    }

                    for x in vec1 {
                        println!("- {}", x);
                    }
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
