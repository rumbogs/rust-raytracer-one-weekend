fn partition<T, F>(v: &mut [T], f: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    let pivot_index = len / 2;

    v.swap(pivot_index, len - 1);

    let mut store_index = 0;
    for i in 0..len - 1 {
        if f(&v[i], &v[len - 1]) {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index, len - 1);
    store_index
}

pub fn quick_sort<T, F>(v: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = v.len();
    if len >= 2 {
        let pivot_index = partition(v, f);
        quick_sort(&mut v[0..pivot_index], f);
        quick_sort(&mut v[pivot_index + 1..len], f);
    }
}