use std::collections::HashSet;
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

fn long_text_split(strs:String,delims:String,second_delims:String,max_sub_wcount:usize) -> Vec<String> {
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
    let mut start = 0usize;
    let mut last_second_delim = 999999usize;
    let mut i = 0usize;
//    let chars = strs.chars();
    for c in &vec_chars {
        if second_delims_set.contains(&c) {
            last_second_delim = i;
        }
        if i - start > max_sub_wcount {
            if 999999 != last_second_delim {
                let mut sent = String::new();
                for subi in start..last_second_delim + 1 {
                    sent.push(vec_chars[subi]);
                }
                sents.push(sent);
                start = last_second_delim + 1;
            }
            last_second_delim = 999999;
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
    return sents;
}
