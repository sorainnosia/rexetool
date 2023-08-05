use std::slice::Iter;


pub fn index_of(str: &String, sub: String, start_index: i32, ignore_case: bool) -> i32 {
    let sc = sub.chars().count();
    let stc = str.chars().count();
    if sc > stc { return -1; }
    if sc <= 0 { return 0; }
    if start_index + (sc as i32) > (stc as i32) { return -1; }

    let mut subs= sub.chars();
    let mut i:i32 = 0;
    let mut cnt:i32 = 0;

    let mut cc:i32 = start_index;
    let mut opt = subs.next();
    
    for c in str.chars() {
        if cc > 0 {
            cc -= 1;
            continue;
        }
        
        match opt {
            Some(d) => { 
                if (ignore_case && c.to_uppercase().cmp(d.to_uppercase()) == std::cmp::Ordering::Equal) || c == d {
                    i += 1;
                    cnt += 1;
                    if cnt == sc as i32 {
                        break;
                    }
                    opt = subs.next();
                } else {
                    i += 1;
                    cnt = 0;
                    subs = sub.chars();
                    opt = subs.next();
                    match opt {
                        Some(e) => {
                            if (ignore_case && c.to_uppercase().cmp(e.to_uppercase()) == std::cmp::Ordering::Equal) || c == e {
                                cnt += 1;
                                if cnt == sc as i32 {
                                    break;
                                }
                                opt = subs.next();
                            }
                        },
                        None => { }
                    }
                }
            },
            None => { break; }
        }
    }
    if cnt == sc as i32 {
        return i - cnt + start_index;
    }
    return -1;
}

pub fn substring(str: &String, i: i32, mut count: i32) -> String {
    let mut result = String::new();

    let stc = str.chars().count();
    if i + count > stc as i32 { panic!("Substring index or count is longer than string length count") }

    let mut strs = str.chars();
    let mut j = i;
    while j > 0 {
        strs.next();
        j -= 1;
    }

    while count > 0 {
        let opt= strs.next();
        match opt {
            Some(c) => result.push_str(&c.to_string()),
            None => { }
        }
        count -= 1;
    }
    return result;
} 

fn same_char_m(ignore_case: bool, opt: Option<&u8>, c: u8, m: &mut i32, sc: usize, started: &mut bool) -> bool {
    let mut reset = false;
    match opt {
        Some(e) => {
            if (ignore_case && c.cmp(&e) == std::cmp::Ordering::Equal) || c == *e {
                *m += 1;
                if *m == sc as i32 {
                    *started = true;
                    reset = true;
                }
            }
        }, 
        None => {}
    }
    return reset;
}

pub fn last_index_of(str: &String, sub: String, ignore_case: bool) -> i32 {
    let sc = sub.len();
    let stc = str.len();
    if sc > stc { return -1; }
    if sc <= 0 { return 0; }

    let mut subs= sub.chars().rev();
    let mut i:i32 = stc as i32;
    let mut cnt:i32 = 0;

    let mut opt = subs.next();
    for c in str.chars().rev() {
        match opt {
            Some(d) => { 
                if (ignore_case && c.to_uppercase().cmp(d.to_uppercase()) == std::cmp::Ordering::Equal) || c == d {
                    i -= 1;
                    cnt += 1;
                    if cnt == sc as i32 {
                        break;
                    }
                    opt = subs.next();
                } else {
                    i -= 1;
                    cnt = 0;
                    subs = sub.chars().rev(); 
                    opt = subs.next();
                    match opt {
                        Some(e) => {
                            if (ignore_case && c.to_uppercase().cmp(e.to_uppercase()) == std::cmp::Ordering::Equal) || c == e {
                                cnt += 1;
                                if cnt == sc as i32 {
                                    break;
                                }
                                opt = subs.next();
                            }
                        },
                        None => { }
                    }
                }
            },
            None => { break; }
        }
    }
    if cnt == sc as i32 {
        return i;
    }
    return -1;
}

pub fn get_filename_without_extension(file: String) -> String {
    let mut i = last_index_of(&file, "/".to_string(), true);
    let j = last_index_of(&file, "\\".to_string(), true);
    let c = file.len() as i32;

    if j > i && j != -1 {
        i = j;
    }
    if i == -1 {
        let ii = last_index_of(&file, ".".to_string(), true);
        if ii > 0 {
            return substring(&file, 0, ii);
        }
        return file;
    }

    let result = substring(&file, i + 1, c - i - 1);

    let ii = last_index_of(&result, ".".to_string(), true);
    if ii > 0 {
        return substring(&result, 0, ii);
    }

    return result;
}

fn between_inner(ignore_case: bool, c: u8, d: u8, m: &mut i32, sc: usize, starts: &mut Iter<u8>, started: &mut bool) -> bool {
    let mut reset = false;
    if (ignore_case && c.cmp(&d) == std::cmp::Ordering::Equal) || c == d  {
        *m += 1;
        if *m == sc as i32 {
            *started = true;
            reset = true;
        } 
    } else {
        *m = 0;
        reset = true;
        let opt = starts.next();
        if same_char_m(ignore_case, opt, c, m, sc, started) {
            reset = true;
        }
    }
    return reset;
}

fn between_inner2(ignore_case: bool, c: u8, e: u8, end: &String, temp: &mut String, temp2: &mut String, m: &mut i32, ec: i32, started: &mut bool) -> bool {
    let mut reset = false;
    if (ignore_case && c.cmp(&e) == std::cmp::Ordering::Equal) || c == e  {
        *m += 1;
        let s = String::from_utf8(vec![e]);
        match s {
            Ok(t) => {
                (*temp2).push_str(t.as_str());
            },
            Err(x) => {}
        }
    }
    else {
        *m = 0;
        (*temp).push_str(&temp2.to_string());
        *temp2 = String::from("") ;

        let mut ends = end.as_bytes().iter();
        let opt = ends.next();
        match opt {
            Some(f) => {
                if (ignore_case && c.cmp(&f) == std::cmp::Ordering::Equal) || c == *f  {
                    *m += 1;
                    let s = String::from_utf8(vec![*f]);
                    match s {
                        Ok(t) => {
                            (*temp2).push_str(t.as_str());
                        },
                        Err(x) => {}
                    }
                } else {
                    let s = String::from_utf8(vec![c]);
                    match s {
                        Ok(t) => {
                            (*temp).push_str(t.as_str());
                        },
                        Err(x) => {}
                    }
                    reset = true;
                }
            },
            None => {}
        }
    }
    return reset;
}

pub fn between(str: &Vec<u8>, start: String, end: String, limit: i32, ignore_case: bool) -> Vec<String> {
    let mut result:Vec<String> = vec![];
    let strs = str.iter();
    let mut starts = start.as_bytes().iter();
    let mut ends = end.as_bytes().iter();
    
    let sc = start.as_bytes().len();
    let ec = end.as_bytes().len();
   
    let mut m = 0;
    let mut started:bool = false;
   
    let mut temp = String::from("");
    let mut temp2 = String::from("");
    for c in strs {
        if started == false {
            
            let opt = starts.next();
            match opt {
                Some(d) => {
                    if between_inner(ignore_case, *c, *d, &mut m, sc, &mut starts, &mut started) {
                        starts = start.as_bytes().iter();
                        m = 0;
                    }
                },
                None => {
                    starts = start.as_bytes().iter();
                    let opt = starts.next();
                    if same_char_m(ignore_case, opt, *c, &mut m, sc, &mut started) {
                        starts = start.as_bytes().iter();
                        m = 0;
                    }
                } 
            }
        }
        else if started == true {
            let opt = ends.next();
            match opt {
                Some(e) => {
                    if between_inner2(ignore_case, *c, *e, &end, &mut temp, &mut temp2, &mut m, ec as i32, &mut started) {
                        ends = end.as_bytes().iter();
                    }
                },
                None => {
                    ends = end.as_bytes().iter();
                    let opt = ends.next();
                    match opt {
                        Some(e) => {
                            if between_inner2(ignore_case, *c, *e, &end, &mut temp, &mut temp2, &mut m, ec as i32, &mut started) {
                                ends = end.as_bytes().iter();
                            }
                        },
                        None => { }
                    }
                }
            }
            
            if m == ec as i32 {
                ends = end.as_bytes().iter();
                m = 0;
                temp2 = String::from("");
                result.push(temp);
                if limit > 0 && (result.len() as i32) >= limit { return result; } 
                started = false;
                temp = String::from("") ;
            }
        }
    }
    return result;
}