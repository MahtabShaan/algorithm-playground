use std::fmt::Debug;
use std::mem;

fn merge<T>(array: &mut [T], aux: &mut [T], lo: usize, mid: usize, hi: usize)
    where T: Ord + Debug + Clone {

    let mut i = lo;
    let mut j = mid + 1;

    for x in lo..=hi {
        aux[x] = array[x].clone();
    }

    for x in lo..=hi {

        if i > mid {
            mem::swap(&mut array[x], &mut aux[j]);
            j+=1;
        } else if j > hi {
            mem::swap(&mut array[x], &mut aux[i]);
            i+=1;
        } else if aux[j] > aux[i] {
            mem::swap(&mut array[x], &mut aux[i]);
            i+=1;
        } else {
            mem::swap(&mut array[x], &mut aux[j]);
            j+=1;
        }
    }
    //println!("merge: {}, {} = {:?}", lo, hi, &array[lo..=hi]);
}

fn sort_recurse<T>(array: &mut [T], aux: &mut [T], lo: usize, hi: usize)
    where T: Ord + Debug + Clone {
    //println!("sort: {}, {}", lo, hi);
    if hi <= lo { return; }

    let mid = lo + (hi - lo) / 2;
    sort_recurse(array, aux, lo, mid);
    sort_recurse(array, aux, mid + 1, hi);
    merge(array, aux, lo, mid, hi);
}

pub fn sort<T: Ord + Debug + Clone>(array: &mut [T]) {

    let mut aux = Vec::new();

    for i in 0..array.len() {
        aux.push(array[i].clone());
    }

    let hi = array.len() - 1;
    sort_recurse(array, &mut aux, 0, hi);
}

pub fn bottomsup_sort<T: Ord + Debug + Clone>(array: &mut [T]) {

    //println!("{:?}", array);

    let length = array.len();
    let mut aux = Vec::new();

    for i in 0..length {
        aux.push(array[i].clone());
    }


    let mut size = 2;

    loop {

        let mut i = 0;

        loop {

            if i >= length { break };
            let mut hi = i + size - 1;
            if hi >= length { hi = length - 1 }
            let mid = i + size / 2 - 1;
            merge(array, &mut aux, i, mid, hi);
            i += size;
        }
        //println!("{}: {:?}", size, array);
        if size > length { break; }
        size = 2 * size;
    }
}
