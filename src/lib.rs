use std::fmt::Debug;

pub fn bubble_sort<T: PartialOrd + Debug>(v: &mut[T]){
    for i in 0..v.len(){       
        let mut sorted = true;
        //finds the n-th largest element and puts it into its final place
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
        let mut v = vec![5,1,4,2,8];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,4,5,8]);
    }
}
