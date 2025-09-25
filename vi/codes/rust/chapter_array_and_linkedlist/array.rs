/*
 * Tệp: array.rs
 * Thời gian tạo: 2023-01-15
 * Tác giả: xBLACICEx (xBLACKICEx@outlook.com), codingonion (coderonion@gmail.com)
 */

use hello_algo_rust::include::print_util;
use rand::Rng;

/* Truy cập ngẫu nhiên phần tử */
fn random_access(nums: &[i32]) -> i32 {
    // Trong khoảng [0, nums.len()) ngẫu nhiên chọn một số
    let random_index = rand::thread_rng().gen_range(0..nums.len());
    // Lấy và trả về phần tử ngẫu nhiên
    let random_num = nums[random_index];
    random_num
}

/* Mở rộng độ dài mảng */
fn extend(nums: &[i32], enlarge: usize) -> Vec<i32> {
    // Khởi tạo một mảng với độ dài mở rộng
    let mut res: Vec<i32> = vec![0; nums.len() + enlarge];
    // Sao chép tất cả phần tử của mảng gốc vào mảng mới
    res[0..nums.len()].copy_from_slice(nums);

    // Trả về mảng mới đã mở rộng
    res
}

/* Chèn phần tử num tại chỉ mục index của mảng */
fn insert(nums: &mut [i32], num: i32, index: usize) {
    // Dịch chuyển tất cả phần tử từ chỉ mục index trở đi về sau một vị trí
    for i in (index + 1..nums.len()).rev() {
        nums[i] = nums[i - 1];
    }
    // Gán num cho phần tử tại vị trí index
    nums[index] = num;
}

/* Xóa phần tử tại chỉ mục index */
fn remove(nums: &mut [i32], index: usize) {
    // Dịch chuyển tất cả phần tử sau chỉ mục index về trước một vị trí
    for i in index..nums.len() - 1 {
        nums[i] = nums[i + 1];
    }
}

/* Duyệt mảng */
fn traverse(nums: &[i32]) {
    let mut _count = 0;
    // Duyệt mảng qua chỉ mục
    for i in 0..nums.len() {
        _count += nums[i];
    }
    // Duyệt trực tiếp các phần tử của mảng
    _count = 0;
    for &num in nums {
        _count += num;
    }
}

/* Tìm phần tử chỉ định trong mảng */
fn find(nums: &[i32], target: i32) -> Option<usize> {
    for i in 0..nums.len() {
        if nums[i] == target {
            return Some(i);
        }
    }
    None
}

/* Mã chính */
fn main() {
    /* Khởi tạo mảng */
    let arr: [i32; 5] = [0; 5];
    print!("Mảng arr = ");
    print_util::print_array(&arr);
    // Trong Rust, khi chỉ định độ dài ([i32; 5]) là mảng, không chỉ định độ dài (&[i32]) là lát cắt
    // Vì mảng Rust được thiết kế để xác định độ dài tại thời điểm biên dịch, nên chỉ có thể sử dụng hằng số để chỉ định độ dài
    // Vector là kiểu được sử dụng làm mảng động trong Rust nói chung
    // Để thuận tiện cho việc triển khai phương thức mở rộng extend(), dưới đây coi vector như mảng (array)
    let nums: Vec<i32> = vec![1, 3, 2, 5, 4];
    print!("\nMảng nums = ");
    print_util::print_array(&nums);

    // Truy cập ngẫu nhiên
    let random_num = random_access(&nums);
    println!("\nLấy phần tử ngẫu nhiên trong nums {}", random_num);

    // Mở rộng độ dài
    let mut nums: Vec<i32> = extend(&nums, 3);
    print!("Mở rộng độ dài mảng lên 8 , được nums = ");
    print_util::print_array(&nums);

    // Chèn phần tử
    insert(&mut nums, 6, 3);
    print!("\nChèn số 6 tại chỉ mục 3 , được nums = ");
    print_util::print_array(&nums);

    // Xóa phần tử
    remove(&mut nums, 2);
    print!("\nXóa phần tử tại chỉ mục 2, được nums = ");
    print_util::print_array(&nums);

    // Duyệt mảng
    traverse(&nums);

    // Tìm phần tử
    let index = find(&nums, 3).unwrap();
    println!("\nTìm phần tử 3 trong nums , được chỉ mục = {}", index);
}

