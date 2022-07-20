use std::fmt::Debug;

pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut[T]){
    for i in 0..v.len(){       
        let mut sorted = true;
        for j in 0..(v.len() - 1) - i{
            if v[j] > v[j+1]{
                v.swap(j,j+1);
                sorted = false
            }
        }
        println!("{:?}", v);
        if sorted{
            return;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut v = vec![2,4,1,5,9,3,15];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,2,3,4,5,9,15]);
    }
}
