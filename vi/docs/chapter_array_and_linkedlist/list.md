# List

<u>List</u> là một khái niệm cấu trúc dữ liệu trừu tượng, đại diện cho một tập hợp các phần tử có thứ tự, hỗ trợ các thao tác như truy cập phần tử, sửa đổi, thêm, xóa và duyệt, mà không yêu cầu người dùng phải xem xét giới hạn về dung lượng. List có thể được triển khai dựa trên linked list hoặc array.

- Bản thân linked list đã là một list, hỗ trợ các thao tác thêm, xóa, tìm kiếm và sửa đổi phần tử, với sự linh hoạt để điều chỉnh kích thước một cách động.
- Array cũng hỗ trợ các thao tác này, nhưng do độ dài cố định của chúng, chúng có thể được coi là một list có giới hạn về độ dài.

Khi triển khai list bằng array, **tính bất biến về độ dài làm giảm tính thực tế của list**. Điều này là do việc dự đoán lượng dữ liệu cần lưu trữ trước là rất khó, khiến cho việc chọn độ dài list phù hợp trở nên khó khăn. Nếu độ dài quá nhỏ, nó có thể không đáp ứng được yêu cầu; nếu quá lớn, nó có thể lãng phí không gian bộ nhớ.

Để giải quyết vấn đề này, chúng ta có thể triển khai list bằng <u>dynamic array</u>. Nó kế thừa những ưu điểm của array và có thể mở rộng một cách động trong quá trình thực thi chương trình.

Trên thực tế, **thư viện chuẩn của nhiều ngôn ngữ lập trình triển khai list bằng dynamic array**, chẳng hạn như `list` của Python, `ArrayList` của Java, `vector` của C++ và `List` của C#. Trong các thảo luận sau đây, chúng ta sẽ coi "list" và "dynamic array" là các khái niệm đồng nghĩa.

## Các thao tác list phổ biến

### Khởi tạo list

Chúng ta thường sử dụng hai phương pháp khởi tạo: "không có giá trị ban đầu" và "có giá trị ban đầu".

=== "Python"

    ```python title="list.py"
    # Khởi tạo list
    # Không có giá trị ban đầu
    nums1: list[int] = []
    # Có giá trị ban đầu
    nums: list[int] = [1, 3, 2, 5, 4]
    ```

=== "C++"

    ```cpp title="list.cpp"
    /* Khởi tạo list */
    // Lưu ý, trong C++ vector tương đương với nums được mô tả ở đây
    // Không có giá trị ban đầu
    vector<int> nums1;
    // Có giá trị ban đầu
    vector<int> nums = { 1, 3, 2, 5, 4 };
    ```

=== "Java"

    ```java title="list.java"
    /* Khởi tạo list */
    // Không có giá trị ban đầu
    List<Integer> nums1 = new ArrayList<>();
    // Có giá trị ban đầu (lưu ý kiểu phần tử phải là lớp wrapper Integer[] cho int[])
    Integer[] numbers = new Integer[] { 1, 3, 2, 5, 4 };
    List<Integer> nums = new ArrayList<>(Arrays.asList(numbers));
    ```

=== "C#"

    ```csharp title="list.cs"
    /* Khởi tạo list */
    // Không có giá trị ban đầu
    List<int> nums1 = [];
    // Có giá trị ban đầu
    int[] numbers = [1, 3, 2, 5, 4];
    List<int> nums = [.. numbers];
    ```

=== "Go"

    ```go title="list_test.go"
    /* Khởi tạo list */
    // Không có giá trị ban đầu
    nums1 := []int{}
    // Có giá trị ban đầu
    nums := []int{1, 3, 2, 5, 4}
    ```

=== "Swift"

    ```swift title="list.swift"
    /* Khởi tạo list */
    // Không có giá trị ban đầu
    let nums1: [Int] = []
    // Có giá trị ban đầu
    var nums = [1, 3, 2, 5, 4]
    ```

=== "JS"

    ```javascript title="list.js"
    /* Khởi tạo list */
    // Không có giá trị ban đầu
    const nums1 = [];
    // Có giá trị ban đầu
    const nums = [1, 3, 2, 5, 4];
    ```

=== "TS"

    ```typescript title="list.ts"
    /* Khởi tạo list */
    // Không có giá trị ban đầu
    const nums1: number[] = [];
    // Có giá trị ban đầu
    const nums: number[] = [1, 3, 2, 5, 4];
    ```

=== "Dart"

    ```dart title="list.dart"
    /* Khởi tạo list */
    // Không có giá trị ban đầu
    List<int> nums1 = [];
    // Có giá trị ban đầu
    List<int> nums = [1, 3, 2, 5, 4];
    ```

=== "Rust"

    ```rust title="list.rs"
    /* Khởi tạo list */
    // Không có giá trị ban đầu
    let nums1: Vec<i32> = Vec::new();
    // Có giá trị ban đầu
    let nums: Vec<i32> = vec![1, 3, 2, 5, 4];
    ```

=== "C"

    ```c title="list.c"
    // C không cung cấp dynamic array tích hợp sẵn
    ```

=== "Kotlin"

    ```kotlin title="list.kt"

    ```

=== "Zig"

    ```zig title="list.zig"
    // Khởi tạo list
    var nums = std.ArrayList(i32).init(std.heap.page_allocator);
    defer nums.deinit();
    try nums.appendSlice(&[_]i32{ 1, 3, 2, 5, 4 });
    ```

### Truy cập các phần tử

List về cơ bản là array, do đó chúng có thể truy cập và cập nhật các phần tử trong thời gian $O(1)$, rất hiệu quả.

=== "Python"

    ```python title="list.py"
    # Truy cập các phần tử
    num: int = nums[1]  # Truy cập phần tử tại chỉ số 1

    # Cập nhật các phần tử
    nums[1] = 0    # Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "C++"

    ```cpp title="list.cpp"
    /* Truy cập các phần tử */
    int num = nums[1];  // Truy cập phần tử tại chỉ số 1

    /* Cập nhật các phần tử */
    nums[1] = 0;  // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "Java"

    ```java title="list.java"
    /* Truy cập các phần tử */
    int num = nums.get(1);  // Truy cập phần tử tại chỉ số 1

    /* Cập nhật các phần tử */
    nums.set(1, 0);  // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "C#"

    ```csharp title="list.cs"
    /* Truy cập các phần tử */
    int num = nums[1];  // Truy cập phần tử tại chỉ số 1

    /* Cập nhật các phần tử */
    nums[1] = 0;  // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "Go"

    ```go title="list_test.go"
    /* Truy cập các phần tử */
    num := nums[1]  // Truy cập phần tử tại chỉ số 1

    /* Cập nhật các phần tử */
    nums[1] = 0     // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "Swift"

    ```swift title="list.swift"
    /* Truy cập các phần tử */
    let num = nums[1] // Truy cập phần tử tại chỉ số 1

    /* Cập nhật các phần tử */
    nums[1] = 0 // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "JS"

    ```javascript title="list.js"
    /* Truy cập các phần tử */
    const num = nums[1];  // Truy cập phần tử tại chỉ số 1

    /* Cập nhật các phần tử */
    nums[1] = 0;  // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "TS"

    ```typescript title="list.ts"
    /* Truy cập các phần tử */
    const num: number = nums[1];  // Truy cập phần tử tại chỉ số 1

    /* Cập nhật các phần tử */
    nums[1] = 0;  // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "Dart"

    ```dart title="list.dart"
    /* Truy cập các phần tử */
    int num = nums[1];  // Truy cập phần tử tại chỉ số 1

    /* Cập nhật các phần tử */
    nums[1] = 0;  // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "Rust"

    ```rust title="list.rs"
    /* Truy cập các phần tử */
    let num: i32 = nums[1];  // Truy cập phần tử tại chỉ số 1
    /* Cập nhật các phần tử */
    nums[1] = 0;             // Cập nhật phần tử tại chỉ số 1 thành 0
    ```

=== "C"

    ```c title="list.c"
    // C không cung cấp dynamic array tích hợp sẵn
    ```

=== "Kotlin"

    ```kotlin title="list.kt"

    ```

=== "Zig"

    ```zig title="list.zig"
    // Truy cập các phần tử
    var num = nums.items[1]; // Truy cập phần tử tại chỉ số 1

    // Cập nhật các phần tử
    nums.items[1] = 0; // Cập nhật phần tử tại chỉ số 1 thành 0  
    ```

### Chèn và xóa các phần tử

So với array, list cung cấp sự linh hoạt hơn trong việc thêm và xóa các phần tử. Mặc dù việc thêm các phần tử vào cuối list là một thao tác $O(1)$, hiệu quả của việc chèn và xóa các phần tử ở những nơi khác trong list vẫn giống như trong array, với độ phức tạp thời gian là $O(n)$.

=== "Python"

    ```python title="list.py"
    # Xóa list
    nums.clear()

    # Thêm các phần tử vào cuối
    nums.append(1)
    nums.append(3)
    nums.append(2)
    nums.append(5)
    nums.append(4)

    # Chèn phần tử vào giữa
    nums.insert(3, 6)  # Chèn số 6 vào chỉ số 3

    # Xóa các phần tử
    nums.pop(3)        # Xóa phần tử tại chỉ số 3
    ```

=== "C++"

    ```cpp title="list.cpp"
    /* Xóa list */
    nums.clear();

    /* Thêm các phần tử vào cuối */
    nums.push_back(1);
    nums.push_back(3);
    nums.push_back(2);
    nums.push_back(5);
    nums.push_back(4);

    /* Chèn phần tử vào giữa */
    nums.insert(nums.begin() + 3, 6);  // Chèn số 6 vào chỉ số 3

    /* Xóa các phần tử */
    nums.erase(nums.begin() + 3);      // Xóa phần tử tại chỉ số 3
    ```

=== "Java"

    ```java title="list.java"
    /* Xóa list */
    nums.clear();

    /* Thêm các phần tử vào cuối */
    nums.add(1);
    nums.add(3);
    nums.add(2);
    nums.add(5);
    nums.add(4);

    /* Chèn phần tử vào giữa */
    nums.add(3, 6);  // Chèn số 6 vào chỉ số 3

    /* Xóa các phần tử */
    nums.remove(3);  // Xóa phần tử tại chỉ số 3
    ```

=== "C#"

    ```csharp title="list.cs"
    /* Xóa list */
    nums.Clear();

    /* Thêm các phần tử vào cuối */
    nums.Add(1);
    nums.Add(3);
    nums.Add(2);
    nums.Add(5);
    nums.Add(4);

    /* Chèn phần tử vào giữa */
    nums.Insert(3, 6);

    /* Xóa các phần tử */
    nums.RemoveAt(3);
    ```

=== "Go"

    ```go title="list_test.go"
    /* Xóa list */
    nums = nil

    /* Thêm các phần tử vào cuối */
    nums = append(nums, 1)
    nums = append(nums, 3)
    nums = append(nums, 2)
    nums = append(nums, 5)
    nums = append(nums, 4)

    /* Chèn phần tử vào giữa */
    nums = append(nums[:3], append([]int{6}, nums[3:]...)...) // Chèn số 6 vào chỉ số 3

    /* Xóa các phần tử */
    nums = append(nums[:3], nums[4:]...) // Xóa phần tử tại chỉ số 3
    ```

=== "Swift"

    ```swift title="list.swift"
    /* Xóa list */
    nums.removeAll()

    /* Thêm các phần tử vào cuối */
    nums.append(1)
    nums.append(3)
    nums.append(2)
    nums.append(5)
    nums.append(4)

    /* Chèn phần tử vào giữa */
    nums.insert(6, at: 3) // Chèn số 6 vào chỉ số 3

    /* Xóa các phần tử */
    nums.remove(at: 3) // Xóa phần tử tại chỉ số 3
    ```

=== "JS"

    ```javascript title="list.js"
    /* Xóa list */
    nums.length = 0;

    /* Thêm các phần tử vào cuối */
    nums.push(1);
    nums.push(3);
    nums.push(2);
    nums.push(5);
    nums.push(4);

    /* Chèn phần tử vào giữa */
    nums.splice(3, 0, 6);

    /* Xóa các phần tử */
    nums.splice(3, 1);
    ```

=== "TS"

    ```typescript title="list.ts"
    /* Xóa list */
    nums.length = 0;

    /* Thêm các phần tử vào cuối */
    nums.push(1);
    nums.push(3);
    nums.push(2);
    nums.push(5);
    nums.push(4);

    /* Chèn phần tử vào giữa */
    nums.splice(3, 0, 6);

    /* Xóa các phần tử */
    nums.splice(3, 1);
    ```

=== "Dart"

    ```dart title="list.dart"
    /* Xóa list */
    nums.clear();

    /* Thêm các phần tử vào cuối */
    nums.add(1);
    nums.add(3);
    nums.add(2);
    nums.add(5);
    nums.add(4);

    /* Chèn phần tử vào giữa */
    nums.insert(3, 6); // Chèn số 6 vào chỉ số 3

    /* Xóa các phần tử */
    nums.removeAt(3); // Xóa phần tử tại chỉ số 3
    ```

=== "Rust"

    ```rust title="list.rs"
    /* Xóa list */
    nums.clear();

    /* Thêm các phần tử vào cuối */
    nums.push(1);
    nums.push(3);
    nums.push(2);
    nums.push(5);
    nums.push(4);

    /* Chèn phần tử vào giữa */
    nums.insert(3, 6);  // Chèn số 6 vào chỉ số 3

    /* Xóa các phần tử */
    nums.remove(3);    // Xóa phần tử tại chỉ số 3
    ```

=== "C"

    ```c title="list.c"
    // C không cung cấp dynamic array tích hợp sẵn
    ```

=== "Kotlin"

    ```kotlin title="list.kt"

    ```

=== "Zig"

    ```zig title="list.zig"
    // Xóa list
    nums.clearRetainingCapacity();

    // Thêm các phần tử vào cuối
    try nums.append(1);
    try nums.append(3);
    try nums.append(2);
    try nums.append(5);
    try nums.append(4);

    // Chèn phần tử vào giữa
    try nums.insert(3, 6); // Chèn số 6 vào chỉ số 3

    // Xóa các phần tử
    _ = nums.orderedRemove(3); // Xóa phần tử tại chỉ số 3
    ```

### Duyệt list

Tương tự như array, list có thể được duyệt bằng cách sử dụng chỉ số hoặc bằng cách duyệt trực tiếp qua từng phần tử.

=== "Python"

    ```python title="list.py"
    # Duyệt list bằng chỉ số
    count = 0
    for i in range(len(nums)):
        count += nums[i]

    # Duyệt trực tiếp qua các phần tử list
    for num in nums:
        count += num
    ```

=== "C++"

    ```cpp title="list.cpp"
    /* Duyệt list bằng chỉ số */
    int count = 0;
    for (int i = 0; i < nums.size(); i++) {
        count += nums[i];
    }

    /* Duyệt trực tiếp qua các phần tử list */
    count = 0;
    for (int num : nums) {
        count += num;
    }
    ```

=== "Java"

    ```java title="list.java"
    /* Duyệt list bằng chỉ số */
    int count = 0;
    for (int i = 0; i < nums.size(); i++) {
        count += nums.get(i);
    }

    /* Duyệt trực tiếp qua các phần tử list */
    for (int num : nums) {
        count += num;
    }
    ```

=== "C#"

    ```csharp title="list.cs"
    /* Duyệt list bằng chỉ số */
    int count = 0;
    for (int i = 0; i < nums.Count; i++) {
        count += nums[i];
    }

    /* Duyệt trực tiếp qua các phần tử list */
    count = 0;
    foreach (int num in nums) {
        count += num;
    }
    ```

=== "Go"

    ```go title="list_test.go"
    /* Duyệt list bằng chỉ số */
    count := 0
    for i := 0; i < len(nums); i++ {
        count += nums[i]
    }

    /* Duyệt trực tiếp qua các phần tử list */
    count = 0
    for _, num := range nums {
        count += num
    }
    ```

=== "Swift"

    ```swift title="list.swift"
    /* Duyệt list bằng chỉ số */
    var count = 0
    for i in nums.indices {
        count += nums[i]
    }

    /* Duyệt trực tiếp qua các phần tử list */
    count = 0
    for num in nums {
        count += num
    }
    ```

=== "JS"

    ```javascript title="list.js"
    /* Duyệt list bằng chỉ số */
    let count = 0;
    for (let i = 0; i < nums.length; i++) {
        count += nums[i];
    }

    /* Duyệt trực tiếp qua các phần tử list */
    count = 0;
    for (const num of nums) {
        count += num;
    }
    ```

=== "TS"

    ```typescript title="list.ts"
    /* Duyệt list bằng chỉ số */
    let count = 0;
    for (let i = 0; i < nums.length; i++) {
        count += nums[i];
    }

    /* Duyệt trực tiếp qua các phần tử list */
    count = 0;
    for (const num of nums) {
        count += num;
    }
    ```

=== "Dart"

    ```dart title="list.dart"
    /* Duyệt list bằng chỉ số */
    int count = 0;
    for (var i = 0; i < nums.length; i++) {
        count += nums[i];
    }
    
    /* Duyệt trực tiếp qua các phần tử list */
    count = 0;
    for (var num in nums) {
        count += num;
    }
    ```

=== "Rust"

    ```rust title="list.rs"
    // Duyệt list bằng chỉ số
    let mut _count = 0;
    for i in 0..nums.len() {
        _count += nums[i];
    }

    // Duyệt trực tiếp qua các phần tử list
    _count = 0;
    for num in &nums {
        _count += num;
    }
    ```

=== "C"

    ```c title="list.c"
    // C không cung cấp dynamic array tích hợp sẵn
    ```

=== "Kotlin"

    ```kotlin title="list.kt"

    ```

=== "Zig"

    ```zig title="list.zig"
    // Duyệt list bằng chỉ số
    var count: i32 = 0;
    var i: i32 = 0;
    while (i < nums.items.len) : (i += 1) {
        count += nums[i];
    }

    // Duyệt trực tiếp qua các phần tử list
    count = 0;
    for (nums.items) |num| {
        count += num;
    }
    ```

### Nối các list

Cho một list mới `nums1`, chúng ta có thể nối nó vào cuối list ban đầu.

=== "Python"

    ```python title="list.py"
    # Nối hai list
    nums1: list[int] = [6, 8, 7, 10, 9]
    nums += nums1  # Nối nums1 vào cuối nums
    ```

=== "C++"

    ```cpp title="list.cpp"
    /* Nối hai list */
    vector<int> nums1 = { 6, 8, 7, 10, 9 };
    // Nối nums1 vào cuối nums
    nums.insert(nums.end(), nums1.begin(), nums1.end());
    ```

=== "Java"

    ```java title="list.java"
    /* Nối hai list */
    List<Integer> nums1 = new ArrayList<>(Arrays.asList(new Integer[] { 6, 8, 7, 10, 9 }));
    nums.addAll(nums1);  // Nối nums1 vào cuối nums
    ```

=== "C#"

    ```csharp title="list.cs"
    /* Nối hai list */
    List<int> nums1 = [6, 8, 7, 10, 9];
    nums.AddRange(nums1);  // Nối nums1 vào cuối nums
    ```

=== "Go"

    ```go title="list_test.go"
    /* Nối hai list */
    nums1 := []int{6, 8, 7, 10, 9}
    nums = append(nums, nums1...)  // Nối nums1 vào cuối nums
    ```

=== "Swift"

    ```swift title="list.swift"
    /* Nối hai list */
    let nums1 = [6, 8, 7, 10, 9]
    nums.append(contentsOf: nums1) // Nối nums1 vào cuối nums
    ```

=== "JS"

    ```javascript title="list.js"
    /* Nối hai list */
    const nums1 = [6, 8, 7, 10, 9];
    nums.push(...nums1);  // Nối nums1 vào cuối nums
    ```

=== "TS"

    ```typescript title="list.ts"
    /* Nối hai list */
    const nums1: number[] = [6, 8, 7, 10, 9];
    nums.push(...nums1);  // Nối nums1 vào cuối nums
    ```

=== "Dart"

    ```dart title="list.dart"
    /* Nối hai list */
    List<int> nums1 = [6, 8, 7, 10, 9];
    nums.addAll(nums1);  // Nối nums1 vào cuối nums
    ```

=== "Rust"

    ```rust title="list.rs"
    /* Nối hai list */
    let nums1: Vec<i32> = vec![6, 8, 7, 10, 9];
    nums.extend(nums1);
    ```

=== "C"

    ```c title="list.c"
    // C không cung cấp dynamic array tích hợp sẵn
    ```

=== "Kotlin"

    ```kotlin title="list.kt"

    ```

=== "Zig"

    ```zig title="list.zig"
    // Nối hai list
    var nums1 = std.ArrayList(i32).init(std.heap.page_allocator);
    defer nums1.deinit();
    try nums1.appendSlice(&[_]i32{ 6, 8, 7, 10, 9 });
    try nums.insertSlice(nums.items.len, nums1.items); // Nối nums1 vào cuối nums
    ```

### Sắp xếp list

Sau khi list được sắp xếp, chúng ta có thể sử dụng các thuật toán thường được sử dụng trong các bài toán thuật toán liên quan đến array, chẳng hạn như thuật toán "tìm kiếm nhị phân" và "two-pointer".

=== "Python"

    ```python title="list.py"
    # Sắp xếp list
    nums.sort()  # Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "C++"

    ```cpp title="list.cpp"
    /* Sắp xếp list */
    sort(nums.begin(), nums.end());  // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "Java"

    ```java title="list.java"
    /* Sắp xếp list */
    Collections.sort(nums);  // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "C#"

    ```csharp title="list.cs"
    /* Sắp xếp list */
    nums.Sort(); // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "Go"

    ```go title="list_test.go"
    /* Sắp xếp list */
    sort.Ints(nums)  // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "Swift"

    ```swift title="list.swift"
    /* Sắp xếp list */
    nums.sort() // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "JS"

    ```javascript title="list.js"
    /* Sắp xếp list */  
    nums.sort((a, b) => a - b);  // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "TS"

    ```typescript title="list.ts"
    /* Sắp xếp list */
    nums.sort((a, b) => a - b);  // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "Dart"

    ```dart title="list.dart"
    /* Sắp xếp list */
    nums.sort(); // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "Rust"

    ```rust title="list.rs"
    /* Sắp xếp list */
    nums.sort(); // Sau khi sắp xếp, các phần tử list theo thứ tự tăng dần
    ```

=== "C"

    ```c title="list.c"
    // C không cung cấp dynamic array tích hợp sẵn
    ```

=== "Kotlin"

    ```kotlin title="list.kt"

    ```

=== "Zig"

    ```zig title="list.zig"
    // Sắp xếp list
    std.sort.sort(i32, nums.items, {}, comptime std.sort.asc(i32));
    ```

## Triển khai list

Nhiều ngôn ngữ lập trình đi kèm với list tích hợp sẵn, bao gồm Java, C++, Python, v.v. Việc triển khai của chúng có xu hướng phức tạp, có các cài đặt được cân nhắc cẩn thận cho các tham số khác nhau, như dung lượng ban đầu và hệ số mở rộng. Độc giả nào tò mò có thể đi sâu vào mã nguồn để tìm hiểu thêm.

Để nâng cao hiểu biết của chúng ta về cách list hoạt động, chúng ta sẽ thử triển khai một phiên bản đơn giản của list, tập trung vào ba khía cạnh thiết kế quan trọng:

- **Dung lượng ban đầu**: Chọn dung lượng ban đầu hợp lý cho array. Trong ví dụ này, chúng ta chọn 10 làm dung lượng ban đầu.
- **Ghi lại kích thước**: Khai báo một biến `size` để ghi lại số lượng phần tử hiện tại trong list, cập nhật theo thời gian thực khi chèn và xóa phần tử. Với biến này, chúng ta có thể xác định vị trí cuối list và xác định xem có cần mở rộng hay không.
- **Cơ chế mở rộng**: Nếu list đạt đến dung lượng tối đa khi chèn một phần tử, cần có một quy trình mở rộng. Điều này bao gồm việc tạo một array lớn hơn dựa trên hệ số mở rộng, và sau đó chuyển tất cả các phần tử từ array hiện tại sang array mới. Trong ví dụ này, chúng ta quy định rằng kích thước array sẽ tăng gấp đôi sau mỗi lần mở rộng.

=== "Python"

    ```python title="my_list.py"
    class MyList:
        """List class"""

        def __init__(self):
            """Constructor"""
            self._capacity: int = 10  # List capacity
            self._arr: list[int] = [0] * self._capacity  # Array (stores list elements)
            self._size: int = 0  # List length (current number of elements)
            self._extend_ratio: int = 2  # Multiple for each list expansion

        def size(self) -> int:
            """Get list length (current number of elements)"""
            return self._size

        def capacity(self) -> int:
            """Get list capacity"""
            return self._capacity

        def get(self, index: int) -> int:
            """Access element"""
            # If the index is out of bounds, throw an exception, as below
            if index < 0 or index >= self._size:
                raise IndexError("Index out of bounds")
            return self._arr[index]

        def set(self, num: int, index: int):
            """Update element"""
            if index < 0 or index >= self._size:
                raise IndexError("Index out of bounds")
            self._arr[index] = num

        def add(self, num: int):
            """Add element at the end"""
            # When the number of elements exceeds capacity, trigger the expansion mechanism
            if self.size() == self.capacity():
                self.extend_capacity()
            self._arr[self._size] = num
            self._size += 1

        def insert(self, num: int, index: int):
            """Insert element in the middle"""
            if index < 0 or index >= self._size:
                raise IndexError("Index out of bounds")
            # When the number of elements exceeds capacity, trigger the expansion mechanism
            if self._size == self.capacity():
                self.extend_capacity()
            # Move all elements after `index` one position backward
            for j in range(self._size - 1, index - 1, -1):
                self._arr[j + 1] = self._arr[j]
            self._arr[index] = num
            # Update the number of elements
            self._size += 1

        def remove(self, index: int) -> int:
            """Remove element"""
            if index < 0 or index >= self._size:
                raise IndexError("Index out of bounds")
            num = self._arr[index]
            # Move all elements after `index` one position forward
            for j in range(index, self._size - 1):
                self._arr[j] = self._arr[j + 1]
            # Update the number of elements
            self._size -= 1
            # Return the removed element
            return num

        def extend_capacity(self):
            """Extend list"""
            # Create a new array of _extend_ratio times the length of the original array and copy the original array to the new array
            self._arr = self._arr + [0] * self.capacity() * (self._extend_ratio - 1)
            # Update list capacity
            self._capacity = len(self._arr)

        def to_array(self) -> list[int]:
            """Return a list of valid lengths"""
            return self._arr[: self._size]
    ```

=== "C++"

    ```cpp title="my_list.cpp"
    /* List class */
    class MyList {
      private:
        int *arr;             // Array (stores list elements)
        int arrCapacity = 10; // List capacity
        int arrSize = 0;      // List length (current number of elements)
        int extendRatio = 2;   // Multiple for each list expansion

      public:
        /* Constructor */
        MyList() {
            arr = new int[arrCapacity];
        }

        /* Destructor */
        ~MyList() {
            delete[] arr;
        }

        /* Get list length (current number of elements)*/
        int size() {
            return arrSize;
        }

        /* Get list capacity */
        int capacity() {
            return arrCapacity;
        }

        /* Access element */
        int get(int index) {
            // If the index is out of bounds, throw an exception, as below
            if (index < 0 || index >= size())
                throw out_of_range("Index out of bounds");
            return arr[index];
        }

        /* Update element */
        void set(int index, int num) {
            if (index < 0 || index >= size())
                throw out_of_range("Index out of bounds");
            arr[index] = num;
        }

        /* Add element at the end */
        void add(int num) {
            // When the number of elements exceeds capacity, trigger the expansion mechanism
            if (size() == capacity())
                extendCapacity();
            arr[size()] = num;
            // Update the number of elements
            arrSize++;
        }

        /* Insert element in the middle */
        void insert(int index, int num) {
            if (index < 0 || index >= size())
                throw out_of_range("Index out of bounds");
            // When the number of elements exceeds capacity, trigger the expansion mechanism
            if (size() == capacity())
                extendCapacity();
            // Move all elements after `index` one position backward
            for (int j = size() - 1; j >= index; j--) {
                arr[j + 1] = arr[j];
            }
            arr[index] = num;
            // Update the number of elements
            arrSize++;
        }

        /* Remove element */
        int remove(int index) {
            if (index < 0 || index >= size())
                throw out_of_range("Index out of bounds");
            int num = arr[index];
            // Move all elements after `index` one position forward
            for (int j = index; j < size() - 1; j++) {
                arr[j] = arr[j + 1];
            }
            // Update the number of elements
            arrSize--;
            // Return the removed element
            return num;
        }

        /* Extend list */
        void extendCapacity() {
            // Create a new array with a length multiple of the original array by extendRatio
            int newCapacity = capacity() * extendRatio;
            int *tmp = arr;
            arr = new int[newCapacity];
            // Copy all elements from the original array to the new array
            for (int i = 0; i < size(); i++) {
                arr[i] = tmp[i];
            }
            // Free memory
            delete[] tmp;
            arrCapacity = newCapacity;
        }

        /* Convert the list to a Vector for printing */
        vector<int> toVector() {
            // Only convert elements within valid length range
            vector<int> vec(size());
            for (int i = 0; i < size(); i++) {
                vec[i] = arr[i];
            }
            return vec;
        }
    };
    ```

=== "Java"

    ```java title="my_list.java"
    /* List class */
    class MyList {
        private int[] arr; // Array (stores list elements)
        private int capacity = 10; // List capacity
        private int size = 0; // List length (current number of elements)
        private int extendRatio = 2; // Multiple for each list expansion

        /* Constructor */
        public MyList() {
            arr = new int[capacity];
        }

        /* Get list length (current number of elements) */
        public int size() {
            return size;
        }

        /* Get list capacity */
        public int capacity() {
            return capacity;
        }

        /* Access element */
        public int get(int index) {
            // If the index is out of bounds, throw an exception, as below
            if (index < 0 || index >= size)
                throw new IndexOutOfBoundsException("Index out of bounds");
            return arr[index];
        }

        /* Update element */
        public void set(int index, int num) {
            if (index < 0 || index >= size)
                throw new IndexOutOfBoundsException("Index out of bounds");
            arr[index] = num;
        }

        /* Add element at the end */
        public void add(int num) {
            // When the number of elements exceeds capacity, trigger the expansion mechanism
            if (size == capacity())
                extendCapacity();
            arr[size] = num;
            // Update the number of elements
            size++;
        }

        /* Insert element in the middle */
        public void insert(int index, int num) {
            if (index < 0 || index >= size)
                throw new IndexOutOfBoundsException("Index out of bounds");
            // When the number of elements exceeds capacity, trigger the expansion mechanism
            if (size == capacity())
                extendCapacity();
            // Move all elements after `index` one position backward
            for (int j = size - 1; j >= index; j--) {
                arr[j + 1] = arr[j];
            }
            arr[index] = num;
            // Update the number of elements
            size++;
        }

        /* Remove element */
        public int remove(int index) {
            if (index < 0 || index >= size)
                throw new IndexOutOfBoundsException("Index out of bounds");
            int num = arr[index];
            // Move all elements after `index` one position forward
            for (int j = index; j < size - 1; j++) {
                arr[j] = arr[j + 1];
            }
            // Update the number of elements
            size--;
            // Return the removed element
            return num;
        }

        /* Extend list */
        public void extendCapacity() {
            // Create a new array with a length multiple of the original array by extendRatio, and copy the original array to the new array
            arr = Arrays.copyOf(arr, capacity() * extendRatio);
            // Update list capacity
            capacity = arr.length;
        }

        /* Convert the list to an array */
        public int[] toArray() {
            int size = size();
            // Only convert elements within valid length range
            int[] arr = new int[size];
            for (int i = 0; i < size; i++) {
                arr[i] = get(i);
            }
            return arr;
        }
    }
    ```

=== "C#"

    ```csharp title="my_list.cs"
    [class]{MyList}-[func]{}
    ```

=== "Go"

    ```go title="my_list.go"
    [class]{myList}-[func]{}
    ```

=== "Swift"

    ```swift title="my_list.swift"
    [class]{MyList}-[func]{}
    ```

=== "JS"

    ```javascript title="my_list.js"
    [class]{MyList}-[func]{}
    ```

=== "TS"

    ```typescript title="my_list.ts"
    [class]{MyList}-[func]{}
    ```

=== "Dart"

    ```dart title="my_list.dart"
    [class]{MyList}-[func]{}
    ```

=== "Rust"

    ```rust title="my_list.rs"
    [class]{MyList}-[func]{}
    ```

=== "C"

    ```c title="my_list.c"
    [class]{MyList}-[func]{}
    ```

=== "Kotlin"

    ```kotlin title="my_list.kt"
    [class]{MyList}-[func]{}
    ```

=== "Ruby"

    ```ruby title="my_list.rb"
    [class]{MyList}-[func]{}
    ```

=== "Zig"

    ```zig title="my_list.zig"
    [class]{MyList}-[func]{}
    ```
