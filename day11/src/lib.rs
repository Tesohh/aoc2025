use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash)]
pub struct Server(pub [char; 3]);

impl Server {
    pub fn new(s: &str) -> Server {
        Server(str_to_char3(s).unwrap())
    }
}

impl Debug for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

pub fn str_to_char3(s: &str) -> Option<[char; 3]> {
    let mut it = s.chars();
    Some((it.next()?, it.next()?, it.next()?).into())
}
