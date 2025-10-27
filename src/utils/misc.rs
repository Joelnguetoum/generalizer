

pub fn remove_first<T: PartialEq>(vec: Vec<T>, value: T) -> Vec<T> where T: Clone{
    let mut temp = vec.clone();
    if let Some(pos) = vec.iter().position(|x| *x == value) {
        temp.remove(pos);
    }
    temp
}