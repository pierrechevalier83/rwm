use std;

pub struct Cmd {
    pub name: std::string::String,
    pub args: std::vec::Vec<std::string::String>,
}
impl Cmd {
    pub fn new(slice: &[&str]) -> Cmd {
        if let Some((name, args)) = slice.split_first() {
            return Cmd {
                name: name.to_string(),
                args: args.iter().map(|x| x.to_string()).collect::<std::vec::Vec<_>>(),
            }
        }
        panic!("Config: invalid process name!");
    }
}
