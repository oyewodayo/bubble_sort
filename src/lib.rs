pub fn bubble_sort<T: PartialOrd>(v: &mut[T]){
    for _ in 0..v.len(){
        for i in 0..v.len() - 1{
            if v[i] > v[i+1]{
                v.swap(i,i+1)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut v = vec![2,4,1,5,9,3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,2,3,4,9]);
    }
}
