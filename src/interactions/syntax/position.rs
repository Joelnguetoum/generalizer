use std::fmt;
use std::fmt::{Debug, Formatter};
#[allow(dead_code)]
pub type Position = String; //Simply String for now, it might change later
#[allow(dead_code)]
pub fn push_end(position: &Position,c: char) -> Position {
    let mut p = position.clone();
    p.push(c);
    p
}
#[allow(dead_code)]
pub fn is_prefix(p: &Position, prefix: &Position) -> bool {
    p.starts_with(prefix)
}



//********************************

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Pos {
    Epsilon(Option<usize>),
    Left(Box<Pos>),
    Right(Box<Pos>),
    #[allow(dead_code)]
    Both(Box<Pos>,Box<Pos>)
}

impl Debug for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Pos::Epsilon(sub_pos) => {
                match sub_pos {
                    None => {
                        write!(f,"o")
                    },
                    Some(sbp_idx) => {
                        write!(f,"s{:}",sbp_idx)
                    }
                }
            },
            Pos::Left(ref in_self) => {
                write!(f,"1{:}",in_self)
            },
            Pos::Right(ref in_self) => {
                write!(f,"2{:}",in_self)
            },
            Pos::Both(ref sub1, ref sub2) => {
                write!(f,"H1{:}_2{:}H",sub1,sub2)
            }
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pos::Epsilon(sub_pos) => {
                match sub_pos {
                    None => {
                        write!(f,"")
                    },
                    Some(sbp_idx) => {
                        write!(f,"s{:}",sbp_idx)
                    }
                }
            },
            Pos::Left(ref in_self) => {
                write!(f,"1{:}",in_self)
            },
            Pos::Right(ref in_self) => {
                write!(f,"2{:}",in_self)
            },
            Pos::Both(ref sub1, ref sub2) => {
                write!(f,"(1{:},2{:})",sub1,sub2)
            }
        }
    }
}





