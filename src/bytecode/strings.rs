use std::collections::HashMap;
use std::rc::Rc;

pub type InternMap = HashMap<String, Rc<String>>;

pub fn get_or_insert_string(string: &str, strings: &mut InternMap) -> Rc<String> {
    if let Some(string) = strings.get(string) {
        string.clone()
    } else {
        let rc = Rc::new(string.to_owned());
        strings.insert(string.to_owned(), rc.clone());
        rc
    }
}
