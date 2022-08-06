pub fn merge_sort(a: &mut [i32]) {
    let len = a.len();
    if len <= 1 {
        return;
    }

    let q = len / 2;
    if a[0..q].len() > 1 {
        merge_sort(&mut a[0..q]);
    }

    if a[q..len].len() > 1 {
        merge_sort(&mut a[q..len]);
    }

    merge(a, 0, q, len);
}

fn merge(a: &mut [i32], p: usize, q: usize, r: usize) {
    let mut t = vec![];

    let mut i=p;
    let mut j=q;

    while i<q && j<r {
        if a[i] > a[j] {
            t.push(a[j]);
            j = j + 1;
        } else {
            t.push(a[i]);
            i = i + 1;
        }
    }

    if i<q {
        for x in i..q {
            t.push(a[x]);
        }
    } else {
        for x in j..r {
            t.push(a[x]);
        }
    }

    for i in 0..t.len() {
        a[i] = t[i];
    }
}

pub fn insert_sort(a: &mut [i32]) {
    if a.len() < 2 {
        return;
    }

    let mut t;
    for i in 1..a.len() {
        for j in 0..i {
            if a[i] < a[j] {
                t = a[i];
                a[i] = a[j];
                a[j] = t;
            }
        }
    }
}

#[test]
fn merge_sort_test() {
    let mut test_cases = vec![
        (vec![], vec![]),
        (vec![1], vec![1]),
        (vec![0, 1], vec![0, 1]),
        (vec![1, 0], vec![0, 1]),
        (vec![0, 1, 2], vec![0, 1, 2]),
        (vec![0, 2, 1], vec![0, 1, 2]),
        (vec![1, 0, 2], vec![0, 1, 2]),
        (vec![1, 2, 0], vec![0, 1, 2]),
        (vec![2, 0, 1], vec![0, 1, 2]),
        (vec![2, 1, 0], vec![0, 1, 2]),
    ];

    for test_case in &mut test_cases {
        merge_sort(&mut test_case.0);
        assert_eq!(test_case.0, test_case.1);
    }
}

#[test]
fn insert_sort_test() {
    let mut test_cases = vec![
        (vec![], vec![]),
        (vec![1], vec![1]),
        (vec![0, 1], vec![0, 1]),
        (vec![1, 0], vec![0, 1]),
        (vec![0, 1, 2], vec![0, 1, 2]),
        (vec![0, 2, 1], vec![0, 1, 2]),
        (vec![1, 0, 2], vec![0, 1, 2]),
        (vec![1, 2, 0], vec![0, 1, 2]),
        (vec![2, 0, 1], vec![0, 1, 2]),
        (vec![2, 1, 0], vec![0, 1, 2]),
    ];

    for test_case in &mut test_cases {
        insert_sort(&mut test_case.0);
        assert_eq!(test_case.0, test_case.1);
    }
}