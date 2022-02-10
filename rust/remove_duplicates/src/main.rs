fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0
    }

    let mut i = 0;
    for j in 1..nums.len() {
        if nums[j] != nums[i] {
            i += 1;
            nums[i] = nums[j];
        }
    }
    (i + 1) as i32
}

fn main() {
    println!("Hello, world!");
    let mut v: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let i = remove_duplicates(&mut v);
    println!("len: {}", i);
    println!("nums: {:?}", &v[..i as usize]);
}
