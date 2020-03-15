extern crate libc;
use std::collections::HashSet;
use std::ffi::CString;
use libc::c_char;
use std::ffi::CStr;
use log::*;

//extern crate log;

//mod ffi;

#[derive(Default,Debug)] //(Debug是为了方便打印） {:?}
struct LongText {
    pub text: String,
    pub size: usize,
    //pub sents: Vec<String>,
}

impl LongText {
    fn bytes_count(&self) -> usize {
        return self.text.len();
    }

    fn chars_count(&self) -> usize {
        return self.text.chars().count();
    }

    fn set_text(&mut self, text: String) {
        self.text = text;
        self.size = self.text.chars().count();
    }

    fn split(&self, delims: String, second_delims: String, max_sub_wcount: usize) -> Vec<String> {
        return long_text_split(self.text.to_string(), delims, second_delims, max_sub_wcount);
    }
}

#[no_mangle]
pub extern fn long_text_split_ffi(strs: *const c_char, delims: *const c_char, second_delims: *const c_char, max_sub_wcount: usize) -> Vec<String> {
    info!("long_text_split_ffi...");
    let c_str: &CStr = unsafe { CStr::from_ptr(strs) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf: String = str_slice.to_owned();  // if necessary

    let c_str: &CStr = unsafe { CStr::from_ptr(delims) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf1: String = str_slice.to_owned();  // if necessary

    let c_str: &CStr = unsafe { CStr::from_ptr(second_delims) };
    let str_slice: &str = c_str.to_str().unwrap();
    let str_buf2: String = str_slice.to_owned();  // if necessary
    return long_text_split(str_buf, str_buf1, str_buf2, max_sub_wcount);
}

pub fn long_text_split(strs: String, delims: String, second_delims: String, max_sub_wcount: usize) -> Vec<String> {
    info!("long_text_split...");
    let mut sents: Vec<String> = Vec::new();
//    let mut sent: String = String::new();
    let mut delims_set: HashSet<char> = HashSet::new();
    let mut second_delims_set: HashSet<char> = HashSet::new();
    let mut vec_chars: Vec<char> = Vec::new();
    for c in strs.chars() {
        vec_chars.push(c);
    }
    for c in delims.chars() {
        delims_set.insert(c);
    }
    for c in second_delims.chars() {
        second_delims_set.insert(c);
    }
    let UNINIT_I = 99999999usize;
    let mut start = 0usize;
    let mut last_second_delim = UNINIT_I;
    let mut i = 0usize;
//    let chars = strs.chars();
    for c in &vec_chars {
        if second_delims_set.contains(&c) {
            last_second_delim = i;
        }
        if i - start > max_sub_wcount {
            if UNINIT_I != last_second_delim {
                let mut sent = String::new();
                for subi in start..last_second_delim + 1 {
                    sent.push(vec_chars[subi]);
                }
                sents.push(sent);
                start = last_second_delim + 1;
            }
            last_second_delim = UNINIT_I;
        }
        if delims_set.contains(&c) {
            let mut sent = String::new();
            for subi in start..i + 1 {
                sent.push(vec_chars[subi]);
            }
            sents.push(sent);
            start = i + 1;
        }
        i += 1;
    }
    /*
    for c in text.chars() {
        //sent += &c.to_string();
        sent.push(c);
        if delims_set.contains(&c) // c.to_string() == delim
        {
            sents.push(sent);
            sent = "".to_string();
        }
    }
    */
    info!("DONE {}", &sents.len());
    return sents;
}