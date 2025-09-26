---
comments: true
---

# 6.1 &nbsp; Bảng băm (Hash table)

Một <u>bảng băm</u>, còn được gọi là <u>bảng ánh xạ băm</u> (hash map), là một cấu trúc dữ liệu thiết lập mối quan hệ ánh xạ giữa các khóa (key) và giá trị (value), cho phép truy xuất phần tử hiệu quả. Cụ thể, khi chúng ta nhập một `key` vào bảng băm, chúng ta có thể truy xuất `value` tương ứng trong thời gian phức tạp $O(1)$.

Như hình dưới đây, cho $n$ học sinh, mỗi học sinh có hai trường dữ liệu: "Tên" và "Mã số học sinh". Nếu chúng ta muốn triển khai một hàm truy vấn nhận mã số học sinh làm đầu vào và trả về tên tương ứng, chúng ta có thể sử dụng bảng băm như trong hình dưới đây.

![Biểu diễn trừu tượng của một bảng băm](hash_map.assets/hash_table_lookup.png){ class="animation-figure" }

<p align="center"> Figure 6-1 &nbsp; Biểu diễn trừu tượng của một bảng băm</p>

Ngoài bảng băm, mảng và danh sách liên kết cũng có thể được sử dụng để triển khai chức năng truy vấn, nhưng độ phức tạp thời gian là khác nhau. Hiệu quả của chúng được so sánh trong bảng dưới đây:

- **Chèn một phần tử**: Chỉ cần thêm phần tử vào cuối mảng (hoặc danh sách liên kết). Độ phức tạp thời gian của thao tác này là $O(1)$.
- **Tìm kiếm một phần tử**: Vì mảng (hoặc danh sách liên kết) không được sắp xếp, việc tìm kiếm một phần tử yêu cầu duyệt qua tất cả các phần tử. Độ phức tạp thời gian của thao tác này là $O(n)$.
- **Xóa một phần tử**: Để xóa một phần tử, trước tiên chúng ta cần định vị nó. Sau đó, chúng ta xóa nó khỏi mảng (hoặc danh sách liên kết). Độ phức tạp thời gian của thao tác này là $O(n)$.

<p align="center"> Bảng <id> &nbsp; So sánh hiệu quả thời gian cho các thao tác phổ biến </p>

<div class="center-table" markdown>

|                | Mảng   | Danh sách liên kết | Bảng băm |
| -------------- | ------ | ----------- | ---------- |
| Tìm kiếm phần tử  | $O(n)$ | $O(n)$      | $O(1)$     |
| Chèn phần tử   | $O(1)$ | $O(1)$      | $O(1)$     |
| Xóa phần tử  | $O(n)$ | $O(n)$      | $O(1)$     |

</div>

Như đã quan sát, **độ phức tạp thời gian cho các thao tác (chèn, xóa, tìm kiếm và sửa đổi) trong bảng băm là $O(1)$**, rất hiệu quả.

## 6.1.1 &nbsp; Các thao tác phổ biến của bảng băm

Các thao tác phổ biến của bảng băm bao gồm: khởi tạo, truy vấn, thêm cặp key-value và xóa cặp key-value. Dưới đây là một ví dụ code:

=== "Python"

    ```python title="hash_map.py"
    # Khởi tạo bảng băm
    hmap: dict = {}

    # Thao tác thêm
    # Thêm cặp key-value (key, value) vào bảng băm
    hmap[12836] = "Xiao Ha"
    hmap[15937] = "Xiao Luo"
    hmap[16750] = "Xiao Suan"
    hmap[13276] = "Xiao Fa"
    hmap[10583] = "Xiao Ya"

    # Thao tác truy vấn
    # Nhập key vào bảng băm, lấy value
    name: str = hmap[15937]

    # Thao tác xóa
    # Xóa cặp key-value (key, value) khỏi bảng băm
    hmap.pop(10583)
    ```

=== "C++"

    ```cpp title="hash_map.cpp"
    /* Khởi tạo bảng băm */
    unordered_map<int, string> map;

    /* Thao tác thêm */
    // Thêm cặp key-value (key, value) vào bảng băm
    map[12836] = "Xiao Ha";
    map[15937] = "Xiao Luo";
    map[16750] = "Xiao Suan";
    map[13276] = "Xiao Fa";
    map[10583] = "Xiao Ya";

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    string name = map[15937];

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    map.erase(10583);
    ```

=== "Java"

    ```java title="hash_map.java"
    /* Khởi tạo bảng băm */
    Map<Integer, String> map = new HashMap<>();

    /* Thao tác thêm */
    // Thêm cặp key-value (key, value) vào bảng băm
    map.put(12836, "Xiao Ha");   
    map.put(15937, "Xiao Luo");   
    map.put(16750, "Xiao Suan");   
    map.put(13276, "Xiao Fa");
    map.put(10583, "Xiao Ya");

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    String name = map.get(15937);

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    map.remove(10583);
    ```

=== "C#"

    ```csharp title="hash_map.cs"
    /* Khởi tạo bảng băm */
    Dictionary<int, string> map = new() {
        /* Thao tác thêm */
        // Thêm cặp key-value (key, value) vào bảng băm
        { 12836, "Xiao Ha" },
        { 15937, "Xiao Luo" },
        { 16750, "Xiao Suan" },
        { 13276, "Xiao Fa" },
        { 10583, "Xiao Ya" }
    };

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    string name = map[15937];

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    map.Remove(10583);
    ```

=== "Go"

    ```go title="hash_map_test.go"
    /* Khởi tạo bảng băm */
    hmap := make(map[int]string)

    /* Thao tác thêm */
    // Thêm cặp key-value (key, value) vào bảng băm
    hmap[12836] = "Xiao Ha"
    hmap[15937] = "Xiao Luo"
    hmap[16750] = "Xiao Suan"
    hmap[13276] = "Xiao Fa"
    hmap[10583] = "Xiao Ya"

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    name := hmap[15937]

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    delete(hmap, 10583)
    ```

=== "Swift"

    ```swift title="hash_map.swift"
    /* Khởi tạo bảng băm */
    var map: [Int: String] = [:]

    /* Thao tác thêm */
    // Thêm cặp key-value (key, value) vào bảng băm
    map[12836] = "Xiao Ha"
    map[15937] = "Xiao Luo"
    map[16750] = "Xiao Suan"
    map[13276] = "Xiao Fa"
    map[10583] = "Xiao Ya"

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    let name = map[15937]!

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    map.removeValue(forKey: 10583)
    ```

=== "JS"

    ```javascript title="hash_map.js"
    /* Khởi tạo bảng băm */
    const map = new Map();
    /* Thao tác thêm */
    // Thêm cặp key-value (key, value) vào bảng băm
    map.set(12836, 'Xiao Ha');
    map.set(15937, 'Xiao Luo');
    map.set(16750, 'Xiao Suan');
    map.set(13276, 'Xiao Fa');
    map.set(10583, 'Xiao Ya');

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    let name = map.get(15937);

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    map.delete(10583);
    ```

=== "TS"

    ```typescript title="hash_map.ts"
    /* Khởi tạo bảng băm */
    const map = new Map<number, string>();
    /* Thao tác thêm */
    // Thêm cặp key-value (key, value) vào bảng băm
    map.set(12836, 'Xiao Ha');
    map.set(15937, 'Xiao Luo');
    map.set(16750, 'Xiao Suan');
    map.set(13276, 'Xiao Fa');
    map.set(10583, 'Xiao Ya');
    console.info('\nSau khi thêm, bảng băm là\nKey -> Value');
    console.info(map);

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    let name = map.get(15937);
    console.info('\nNhập số học sinh 15937, truy vấn tên ' + name);

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    map.delete(10583);
    console.info('\nSau khi xóa 10583, bảng băm là\nKey -> Value');
    console.info(map);
    ```

=== "Dart"

    ```dart title="hash_map.dart"
    /* Khởi tạo bảng băm */
    Map<int, String> map = {};

    /* Thao tác thêm */
    // Thêm cặp key-value (key, value) vào bảng băm
    map[12836] = "Xiao Ha";
    map[15937] = "Xiao Luo";
    map[16750] = "Xiao Suan";
    map[13276] = "Xiao Fa";
    map[10583] = "Xiao Ya";

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    String name = map[15937];

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    map.remove(10583);
    ```

=== "Rust"

    ```rust title="hash_map.rs"
    use std::collections::HashMap;

    /* Khởi tạo bảng băm */
    let mut map: HashMap<i32, String> = HashMap::new();

    /* Thao tác thêm */
    // Thêm cặp key-value (key, value) vào bảng băm
    map.insert(12836, "Xiao Ha".to_string());
    map.insert(15937, "Xiao Luo".to_string());
    map.insert(16750, "Xiao Suan".to_string());
    map.insert(13279, "Xiao Fa".to_string());
    map.insert(10583, "Xiao Ya".to_string());

    /* Thao tác truy vấn */
    // Nhập key vào bảng băm, lấy value
    let _name: Option<&String> = map.get(&15937);

    /* Thao tác xóa */
    // Xóa cặp key-value (key, value) khỏi bảng băm
    let _removed_value: Option<String> = map.remove(&10583);
    ```

=== "C"

    ```c title="hash_map.c"
    // C không cung cấp bảng băm tích hợp sẵn
    ```

=== "Kotlin"

    ```kotlin title="hash_map.kt"

    ```

=== "Zig"

    ```zig title="hash_map.zig"

    ```

??? pythontutor "Code Visualization"

    <div style="height: 549px; width: 100%;"><iframe class="pythontutor-iframe" src="https://pythontutor.com/iframe-embed.html#code=%22%22%22Driver%20Code%22%22%22%0Aif%20__name__%20%3D%3D%20%22__main__%22%3A%0A%20%20%20%20%23%20%E5%88%9D%E5%A7%8B%E5%8C%96%E5%93%88%E5%B8%8C%E8%A1%A8%0A%20%20%20%20hmap%20%3D%20%7B%7D%0A%20%20%20%20%0A%20%20%20%20%23%20%E6%B7%BB%E5%8A%A0%E6%93%8D%E4%BD%9C%0A%20%20%20%20%23%20%E5%9C%A8%E5%93%88%E5%B8%8C%E8%A1%A8%E4%B8%AD%E6%B7%BB%E5%8A%A0%E9%94%AE%E5%80%BC%E5%AF%B9%20%28key,%20value%29%0A%20%20%20%20hmap%5B12836%5D%20%3D%20%22%E5%B0%8F%E5%93%88%22%0A%20%20%20%20hmap%5B15937%5D%20%3D%20%22%E5%B0%8F%E5%95%B0%22%0A%20%20%20%20hmap%5B16750%5D%20%3D%20%22%E5%B0%8F%E7%AE%97%22%0A%20%20%20%20hmap%5B13276%5D%20%3D%20%22%E5%B0%8F%E6%B3%95%22%0A%20%20%20%20hmap%5B10583%5D%20%3D%20%22%E5%B0%8F%E9%B8%AD%22%0A%20%20%20%20%0A%20%20%20%20%23%20%E6%9F%A5%E8%AF%A2%E6%93%8D%E4%BD%9C%0A%20%20%20%20%23%20%E5%90%91%E5%93%88%E5%B8%8C%E8%A1%A8%E4%B8%AD%E8%BE%93%E5%85%A5%E9%94%AE%20key%20%EF%BC%8C%E5%BE%97%E5%88%B0%E5%80%BC%20value%0A%20%20%20%20name%20%3D%20hmap%5B15937%5D%0A%20%20%20%20%0A%20%20%20%20%23%20%E5%88%A0%E9%99%A4%E6%93%8D%E4%BD%9C%0A%20%20%20%20%23%20%E5%9C%A8%E5%93%88%E5%B8%8C%E8%A1%A8%E4%B8%AD%E5%88%A0%E9%99%A4%E9%94%AE%E5%80%BC%E5%AF%B9%20%28key,%20value%29%0A%20%20%20%20hmap.pop%2810583%29&codeDivHeight=472&codeDivWidth=350&cumulative=false&curInstr=2&heapPrimitives=nevernest&origin=opt-frontend.js&py=311&rawInputLstJSON=%5B%5D&textReferences=false"> </iframe></div>
    <div style="margin-top: 5px;"><a href="https://pythontutor.com/iframe-embed.html#code=%22%22%22Driver%20Code%22%22%22%0Aif%20__name__%20%3D%3D%20%22__main__%22%3A%0A%20%20%20%20%23%20%E5%88%9D%E5%A7%8B%E5%8C%96%E5%93%88%E5%B8%8C%E8%A1%A8%0A%20%20%20%20hmap%20%3D%20%7B%7D%0A%20%20%20%20%0A%20%20%20%20%23%20%E6%B7%BB%E5%8A%A0%E6%93%8D%E4%BD%9C%0A%20%20%20%20%23%20%E5%9C%A8%E5%93%88%E5%B8%8C%E8%A1%A8%E4%B8%AD%E6%B7%BB%E5%8A%A0%E9%94%AE%E5%80%BC%E5%AF%B9%20%28key,%20value%29%0A%20%20%20%20hmap%5B12836%5D%20%3D%20%22%E5%B0%8F%E5%93%88%22%0A%20%20%20%20hmap%5B15937%5D%20%3D%20%22%E5%B0%8F%E5%95%B0%22%0A%20%20%20%20hmap%5B16750%5D%20%3D%20%22%E5%B0%8F%E7%AE%97%22%0A%20%20%20%20hmap%5B13276%5D%20%3D%20%22%E5%B0%8F%E6%B3%95%22%0A%20%20%20%20hmap%5B10583%5D%20%3D%20%22%E5%B0%8F%E9%B8%AD%22%0A%20%20%20%20%0A%20%20%20%20%23%20%E6%9F%A5%E8%AF%A2%E6%93%8D%E4%BD%9C%0A%20%20%20%20%23%20%E5%90%91%E5%93%88%E5%B8%8C%E8%A1%A8%E4%B8%AD%E8%BE%93%E5%85%A5%E9%94%AE%20key%20%EF%BC%8C%E5%BE%97%E5%88%B0%E5%80%BC%20value%0A%20%20%20%20name%20%3D%20hmap%5B15937%5D%0A%20%20%20%20%0A%20%20%20%20%23%20%E5%88%A0%E9%99%A4%E6%93%8D%E4%BD%9C%0A%20%20%20%20%23%20%E5%9C%A8%E5%93%88%E5%B8%8C%E8%A1%A8%E4%B8%AD%E5%88%A0%E9%99%A4%E9%94%AE%E5%80%BC%E5%AF%B9%20%28key,%20value%29%0A%20%20%20%20hmap.pop%2810583%29&codeDivHeight=800&codeDivWidth=600&cumulative=false&curInstr=2&heapPrimitives=nevernest&origin=opt-frontend.js&py=311&rawInputLstJSON=%5B%5D&textReferences=false" target="_blank" rel="noopener noreferrer">Full Screen ></a></div>

Có ba cách phổ biến để duyệt một bảng băm: duyệt các cặp key-value, duyệt các key và duyệt các value. Dưới đây là một ví dụ code:

=== "Python"

    ```python title="hash_map.py"
    # Duyệt bảng băm
    # Duyệt các cặp key-value key->value
    for key, value in hmap.items():
        print(key, "->", value)
    # Chỉ duyệt các key
    for key in hmap.keys():
        print(key)
    # Chỉ duyệt các value
    for value in hmap.values():
        print(value)
    ```

=== "C++"

    ```cpp title="hash_map.cpp"
    /* Duyệt bảng băm */
    // Duyệt các cặp key-value key->value
    for (auto kv: map) {
        cout << kv.first << " -> " << kv.second << endl;
    }
    // Duyệt bằng iterator key->value
    for (auto iter = map.begin(); iter != map.end(); iter++) {
        cout << iter->first << "->" << iter->second << endl;
    }
    ```

=== "Java"

    ```java title="hash_map.java"
    /* Duyệt bảng băm */
    // Duyệt các cặp key-value key->value
    for (Map.Entry<Integer, String> kv: map.entrySet()) {
        System.out.println(kv.getKey() + " -> " + kv.getValue());
    }
    // Chỉ duyệt các key
    for (int key: map.keySet()) {
        System.out.println(key);
    }
    // Chỉ duyệt các value
    for (String val: map.values()) {
        System.out.println(val);
    }
    ```

=== "C#"

    ```csharp title="hash_map.cs"
    /* Duyệt bảng băm */
    // Duyệt các cặp key-value Key->Value
    foreach (var kv in map) {
        Console.WriteLine(kv.Key + " -> " + kv.Value);
    }
    // Chỉ duyệt các key
    foreach (int key in map.Keys) {
        Console.WriteLine(key);
    }
    // Chỉ duyệt các value
    foreach (string val in map.Values) {
        Console.WriteLine(val);
    }
    ```

=== "Go"

    ```go title="hash_map_test.go"
    /* Duyệt bảng băm */
    // Duyệt các cặp key-value key->value
    for key, value := range hmap {
        fmt.Println(key, "->", value)
    }
    // Chỉ duyệt các key
    for key := range hmap {
        fmt.Println(key)
    }
    // Chỉ duyệt các value
    for _, value := range hmap {
        fmt.Println(value)
    }
    ```

=== "Swift"

    ```swift title="hash_map.swift"
    /* Duyệt bảng băm */
    // Duyệt các cặp key-value Key->Value
    for (key, value) in map {
        print("\(key) -> \(value)")
    }
    // Chỉ duyệt các key
    for key in map.keys {
        print(key)
    }
    // Chỉ duyệt các value
    for value in map.values {
        print(value)
    }
    ```

=== "JS"

    ```javascript title="hash_map.js"
    /* Duyệt bảng băm */
    console.info('\nDuyệt các cặp key-value Key->Value');
    for (const [k, v] of map.entries()) {
        console.info(k + ' -> ' + v);
    }
    console.info('\nChỉ duyệt các key Key');
    for (const k of map.keys()) {
        console.info(k);
    }
    console.info('\nChỉ duyệt các value Value');
    for (const v of map.values()) {
        console.info(v);
    }
    ```

=== "TS"

    ```typescript title="hash_map.ts"
    /* Duyệt bảng băm */
    console.info('\nDuyệt các cặp key-value Key->Value');
    for (const [k, v] of map.entries()) {
        console.info(k + ' -> ' + v);
    }
    console.info('\nChỉ duyệt các key Key');
    for (const k of map.keys()) {
        console.info(k);
    }
    console.info('\nChỉ duyệt các value Value');
    for (const v of map.values()) {
        console.info(v);
    }
    ```

=== "Dart"

    ```dart title="hash_map.dart"
    /* Duyệt bảng băm */
    // Duyệt các cặp key-value Key->Value
    map.forEach((key, value) {
    print('$key -> $value');
    });

    // Chỉ duyệt các key Key
    map.keys.forEach((key) {
    print(key);
    });

    // Chỉ duyệt các value Value
    map.values.forEach((value) {
    print(value);
    });
    ```

=== "Rust"

    ```rust title="hash_map.rs"
    /* Duyệt bảng băm */
    // Duyệt các cặp key-value Key->Value
    for (key, value) in &map {
        println!("{key} -> {value}");
    }

    // Chỉ duyệt các key Key
    for key in map.keys() {
        println!("{key}"); 
    }

    // Chỉ duyệt các value Value
    for value in map.values() {
        println!("{value}");
    }
    ```

=== "C"

    ```c title="hash_map.c"
    // C không cung cấp bảng băm tích hợp sẵn
    ```

=== "Kotlin"

    ```kotlin title="hash_map.kt"

    ```

=== "Zig"

    ```zig title="hash_map.zig"
    // Zig example is not provided
    ```

??? pythontutor "Code Visualization"

    <div style="height: 549px; width: 100%;"><iframe class="pythontutor-iframe" src="https://pythontutor.com/iframe-embed.html#code=%22%22%22Driver%20Code%22%22%22%0Aif%20__name__%20%3D%3D%20%22__main__%22%3A%0A%20%20%20%20%23%20%E5%88%9D%E5%A7%8B%E5%8C%96%E5%93%88%E5%B8%8C%E8%A1%A8%0A%20%20%20%20hmap%20%3D%20%7B%7D%0A%20%20%20%20%0A%20%20%20%20%23%20%E6%B7%BB%E5%8A%A0%E6%93%8D%E4%BD%9C%0A%20%20%20%20%23%20%E5%9C%A8%E5%93%88%E5%B8%8C%E8%A1%A8%E4%B8%AD%E6%B7%BB%E5%8A%A0%E9%94%AE%E5%80%BC%E5%AF%B9%20%28key,%20value%29%0A%20%20%20%20hmap%5B12836%5D%20%3D%20%22%E5%B0%8F%E5%93%88%22%0A%20%20%20%20hmap%5B15937%5D%20%3D%20%22%E5%B0%8F%E5%95%B0%22%0A%20%20%20%20hmap%5B16750%5D%20%3D%20%22%E5%B0%8F%E7%AE%97%22%0A%20%20%20%20hmap%5B13276%5D%20%3D%20%22%E5%B0%8F%E6%B3%95%22%0A%20%20%20%20hmap%5B10583%5D%20%3D%20%22%E5%B0%8F%E9%B8%AD%22%0A%20%20%20%20%0A%20%20%20%20%23%20%E9%81%8D%E5%8E%86%E5%93%88%E5%B8%8C%E8%A1%A8%0A%20%20%20%20%23%20%E9%81%8D%E5%8E%86%E9%94%AE%E5%80%BC%E5%AF%B9%20key-%3Evalue%0A%20%20%20%20for%20key,%20value%20in%20hmap.items%28%29%3A%0A%20%20%20%20%20%20%20%20print%28key,%20%22-%3E%22,%20value%29%0A%20%20%20%20%23%20%E5%8D%95%E7%8B%AC%E9%81%8D%E5%8E%86%E9%94%AE%20key%0A%20%20%20%20for%20key%20in%20hmap.keys%28%29%3A%0A%20%20%20%20%20%20%20%20print%28key%29%0A%20%20%20%20%23%20%E5%8D%95%E7%8B%AC%E9%81%8D%E5%8E%86%E5%80%BC%20value%0A%20%20%20%20for%20value%20in%20hmap.values%28%29%3A%0A%20%20%20%20%20%20%20%20print%28value%29&codeDivHeight=472&codeDivWidth=350&cumulative=false&curInstr=8&heapPrimitives=nevernest&origin=opt-frontend.js&py=311&rawInputLstJSON=%5B%5D&textReferences=false"> </iframe></div>
    <div style="margin-top: 5px;"><a href="https://pythontutor.com/iframe-embed.html#code=%22%22%22Driver%20Code%22%22%22%0Aif%20__name__%20%3D%3D%20%22__main__%22%3A%0A%20%20%20%20%23%20%E5%88%9D%E5%A7%8B%E5%8C%96%E5%93%88%E5%B8%8C%E8%A1%A8%0A%20%20%20%20hmap%20%3D%20%7B%7D%0A%20%20%20%20%0A%20%20%20%20%23%20%E6%B7%BB%E5%8A%A0%E6%93%8D%E4%BD%9C%0A%20%20%20%20%23%20%E5%9C%A8%E5%93%88%E5%B8%8C%E8%A1%A8%E4%B8%AD%E6%B7%BB%E5%8A%A0%E9%94%AE%E5%80%BC%E5%AF%B9%20%28key,%20value%29%0A%20%20%20%20hmap%5B12836%5D%20%3D%20%22%E5%B0%8F%E5%93%88%22%0A%20%20%20%20hmap%5B15937%5D%20%3D%20%22%E5%B0%8F%E5%95%B0%22%0A%20%20%20%20hmap%5B16750%5D%20%3D%20%22%E5%B0%8F%E7%AE%97%22%0A%20%20%20%20hmap%5B13276%5D%20%3D%20%22%E5%B0%8F%E6%B3%95%22%0A%20%20%20%20hmap%5B10583%5D%20%3D%20%22%E5%B0%8F%E9%B8%AD%22%0A%20%20%20%20%0A%20%20%20%20%23%20%E9%81%8D%E5%8E%86%E5%93%88%E5%B8%8C%E8%A1%A8%0A%20%20%20%20%23%20%E9%81%8D%E5%8E%86%E9%94%AE%E5%80%BC%E5%AF%B9%20key-%3Evalue%0A%20%20%20%20for%20key,%20value%20in%20hmap.items%28%29%3A%0A%20%20%20%20%20%20%20%20print%28key,%20%22-%3E%22,%20value%29%0A%20%20%20%20%23%20%E5%8D%95%E7%8B%AC%E9%81%8D%E5%8E%86%E9%94%AE%20key%0A%20%20%20%20for%20key%20in%20hmap.keys%28%29%3A%0A%20%20%20%20%20%20%20%20print%28key%29%0A%20%20%20%20%23%20%E5%8D%95%E7%8B%AC%E9%81%8D%E5%8E%86%E5%80%BC%20value%0A%20%20%20%20for%20value%20in%20hmap.values%28%29%3A%0A%20%20%20%20%20%20%20%20print%28value%29&codeDivHeight=800&codeDivWidth=600&cumulative=false&curInstr=8&heapPrimitives=nevernest&origin=opt-frontend.js&py=311&rawInputLstJSON=%5B%5D&textReferences=false" target="_blank" rel="noopener noreferrer">Full Screen ></a></div>

## 6.1.2 &nbsp; Triển khai đơn giản của bảng băm

Đầu tiên, hãy xem xét trường hợp đơn giản nhất: **triển khai bảng băm chỉ bằng một mảng**. Trong bảng băm, mỗi vị trí trống trong mảng được gọi là một <u>bucket</u> (ô), và mỗi bucket có thể lưu trữ một cặp key-value. Do đó, thao tác truy vấn bao gồm việc tìm bucket tương ứng với `key` và truy xuất `value` từ đó.

Vậy, làm thế nào để chúng ta định vị bucket tương ứng dựa trên `key`? Điều này đạt được thông qua một <u>hàm băm</u> (hash function). Vai trò của hàm băm là ánh xạ một không gian đầu vào lớn hơn sang một không gian đầu ra nhỏ hơn. Trong một bảng băm, không gian đầu vào bao gồm tất cả các key, và không gian đầu ra bao gồm tất cả các bucket (chỉ số mảng). Nói cách khác, với một `key` cho trước, **chúng ta có thể sử dụng hàm băm để xác định vị trí lưu trữ của cặp key-value tương ứng trong mảng**.

Với một `key` cho trước, việc tính toán hàm băm bao gồm hai bước:

1. Tính giá trị băm bằng cách sử dụng một thuật toán băm nhất định `hash()`.
2. Lấy phần dư của giá trị băm với số lượng bucket (độ dài mảng) `capacity` để thu được `index` mảng tương ứng với key.

```shell
index = hash(key) % capacity
```

Sau đó, chúng ta có thể sử dụng `index` để truy cập bucket tương ứng trong bảng băm và do đó truy xuất `value`.

Giả sử độ dài mảng là `capacity = 100` và thuật toán băm được định nghĩa là `hash(key) = key`. Do đó, hàm băm có thể được biểu diễn là `key % 100`. Hình sau minh họa nguyên tắc hoạt động của hàm băm bằng cách sử dụng `key` làm mã số học sinh và `value` làm tên.

![Nguyên tắc hoạt động của hàm băm](hash_map.assets/hash_function.png){ class="animation-figure" }

<p align="center"> Figure 6-2 &nbsp; Nguyên tắc hoạt động của hàm băm</p>

Đoạn code sau triển khai một bảng băm đơn giản. Ở đây, chúng ta đóng gói `key` và `value` vào một class `Pair` để biểu diễn cặp key-value.

=== "Python"

    ```python title="array_hash_map.py"
    class Pair:
        """Key-value pair"""

        def __init__(self, key: int, val: str):
            self.key = key
            self.val = val

    class ArrayHashMap:
        """Hash table based on array implementation"""

        def __init__(self):
            """Constructor"""
            # Initialize an array, containing 100 buckets
            self.buckets: list[Pair | None] = [None] * 100

        def hash_func(self, key: int) -> int:
            """Hash function"""
            index = key % 100
            return index

        def get(self, key: int) -> str:
            """Query operation"""
            index: int = self.hash_func(key)
            pair: Pair = self.buckets[index]
            if pair is None:
                return None
            return pair.val

        def put(self, key: int, val: str):
            """Add operation"""
            pair = Pair(key, val)
            index: int = self.hash_func(key)
            self.buckets[index] = pair

        def remove(self, key: int):
            """Remove operation"""
            index: int = self.hash_func(key)
            # Set to None, representing removal
            self.buckets[index] = None

        def entry_set(self) -> list[Pair]:
            """Get all key-value pairs"""
            result: list[Pair] = []
            for pair in self.buckets:
                if pair is not None:
                    result.append(pair)
            return result

        def key_set(self) -> list[int]:
            """Get all keys"""
            result = []
            for pair in self.buckets:
                if pair is not None:
                    result.append(pair.key)
            return result

        def value_set(self) -> list[str]:
            """Get all values"""
            result = []
            for pair in self.buckets:
                if pair is not None:
                    result.append(pair.val)
            return result

        def print(self):
            """Print hash table"""
            for pair in self.buckets:
                if pair is not None:
                    print(pair.key, "->", pair.val)
    ```

=== "C++"

    ```cpp title="array_hash_map.cpp"
    /* Key-value pair */
    struct Pair {
      public:
        int key;
        string val;
        Pair(int key, string val) {
            this->key = key;
            this->val = val;
        }
    };

    /* Hash table based on array implementation */
    class ArrayHashMap {
      private:
        vector<Pair *> buckets;

      public:
        ArrayHashMap() {
            // Initialize an array, containing 100 buckets
            buckets = vector<Pair *>(100);
        }

        ~ArrayHashMap() {
            // Free memory
            for (const auto &bucket : buckets) {
                delete bucket;
            }
            buckets.clear();
        }

        /* Hash function */
        int hashFunc(int key) {
            int index = key % 100;
            return index;
        }

        /* Query operation */
        string get(int key) {
            int index = hashFunc(key);
            Pair *pair = buckets[index];
            if (pair == nullptr)
                return "";
            return pair->val;
        }

        /* Add operation */
        void put(int key, string val) {
            Pair *pair = new Pair(key, val);
            int index = hashFunc(key);
            buckets[index] = pair;
        }

        /* Remove operation */
        void remove(int key) {
            int index = hashFunc(key);
            // Free memory and set to nullptr
            delete buckets[index];
            buckets[index] = nullptr;
        }

        /* Get all key-value pairs */
        vector<Pair *> pairSet() {
            vector<Pair *> pairSet;
            for (Pair *pair : buckets) {
                if (pair != nullptr) {
                    pairSet.push_back(pair);
                }
            }
            return pairSet;
        }

        /* Get all keys */
        vector<int> keySet() {
            vector<int> keySet;
            for (Pair *pair : buckets) {
                if (pair != nullptr) {
                    keySet.push_back(pair->key);
                }
            }
            return keySet;
        }

        /* Get all values */
        vector<string> valueSet() {
            vector<string> valueSet;
            for (Pair *pair : buckets) {
                if (pair != nullptr) {
                    valueSet.push_back(pair->val);
                }
            }
            return valueSet;
        }

        /* Print hash table */
        void print() {
            for (Pair *kv : pairSet()) {
                cout << kv->key << " -> " << kv->val << endl;
            }
        }
    };
    ```

=== "Java"

    ```java title="array_hash_map.java"
    /* Key-value pair */
    class Pair {
        public int key;
        public String val;

        public Pair(int key, String val) {
            this.key = key;
            this.val = val;
        }
    }

    /* Hash table based on array implementation */
    class ArrayHashMap {
        private List<Pair> buckets;

        public ArrayHashMap() {
            // Initialize an array, containing 100 buckets
            buckets = new ArrayList<>();
            for (int i = 0; i < 100; i++) {
                buckets.add(null);
            }
        }

        /* Hash function */
        private int hashFunc(int key) {
            int index = key % 100;
            return index;
        }

        /* Query operation */
        public String get(int key) {
            int index = hashFunc(key);
            Pair pair = buckets.get(index);
            if (pair == null)
                return null;
            return pair.val;
        }

        /* Add operation */
        public void put(int key, String val) {
            Pair pair = new Pair(key, val);
            int index = hashFunc(key);
            buckets.set(index, pair);
        }

        /* Remove operation */
        public void remove(int key) {
            int index = hashFunc(key);
            // Set to null, indicating removal
            buckets.set(index, null);
        }

        /* Get all key-value pairs */
        public List<Pair> pairSet() {
            List<Pair> pairSet = new ArrayList<>();
            for (Pair pair : buckets) {
                if (pair != null)
                    pairSet.add(pair);
            }
            return pairSet;
        }

        /* Get all keys */
        public List<Integer> keySet() {
            List<Integer> keySet = new ArrayList<>();
            for (Pair pair : buckets) {
                if (pair != null)
                    keySet.add(pair.key);
            }
            return keySet;
        }

        /* Get all values */
        public List<String> valueSet() {
            List<String> valueSet = new ArrayList<>();
            for (Pair pair : buckets) {
                if (pair != null)
                    valueSet.add(pair.val);
            }
            return valueSet;
        }

        /* Print hash table */
        public void print() {
            for (Pair kv : pairSet()) {
                System.out.println(kv.key + " -> " + kv.val);
            }
        }
    }
    ```

=== "C#"

    ```csharp title="array_hash_map.cs"
    [class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

=== "Go"

    ```go title="array_hash_map.go"
    [class]{pair}-[func]{}

    [class]{arrayHashMap}-[func]{}
    ```

=== "Swift"

    ```swift title="array_hash_map.swift"
    [file]{utils/pair.swift}-[class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

=== "JS"

    ```javascript title="array_hash_map.js"
    [class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

=== "TS"

    ```typescript title="array_hash_map.ts"
    [class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

=== "Dart"

    ```dart title="array_hash_map.dart"
    [class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

=== "Rust"

    ```rust title="array_hash_map.rs"
        // Định nghĩa một cặp key-value. Đây là viên gạch xây dựng nên bảng băm của chúng ta.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Pair {
        pub key: i32,
        pub val: String,
    }
    // Bảng băm được triển khai dựa trên mảng (vector trong Rust).
    pub struct ArrayHashMap {
        buckets: Vec<Option<Pair>>,
    }

    impl ArrayHashMap {
        pub fn new() -> ArrayHashMap {
            // Khởi tạo mảng chứa 100 "bucket" (ngăn chứa). Mỗi bucket có thể chứa một cặp key-value.
            Self {
                buckets: vec![None; 100],
            }
        }

        // Hàm băm: Chuyển đổi key thành một chỉ số trong mảng buckets.
        fn hash_func(&self, key: i32) -> usize {
            key as usize % 100
        }

        // Thao tác truy vấn (get): Lấy giá trị (value) tương ứng với key.
        pub fn get(&self, key: i32) -> Option<&String> {
            let index = self.hash_func(key);
            self.buckets[index].as_ref().map(|pair| &pair.val)
        }

        // Thao tác thêm/cập nhật (put): Thêm một cặp key-value mới hoặc cập nhật giá trị nếu key đã tồn tại.
        pub fn put(&mut self, key: i32, val: &str) {
            let index = self.hash_func(key);
            self.buckets[index] = Some(Pair {
                key,
                val: val.to_string(),
            });
        }

        // Thao tác xóa (remove): Xóa một cặp key-value khỏi bảng băm.
        pub fn remove(&mut self, key: i32) {
            let index = self.hash_func(key);
            // Gán giá trị `None` cho bucket để đánh dấu là đã xóa.
            self.buckets[index] = None;
        }

        // Lấy tất cả các cặp key-value hiện có trong bảng băm.
        pub fn entry_set(&self) -> Vec<&Pair> {
            self.buckets
                .iter()
                .filter_map(|pair| pair.as_ref())
                .collect()
        }

        // Lấy tất cả các key hiện có.
        pub fn key_set(&self) -> Vec<&i32> {
            self.buckets
                .iter()
                .filter_map(|pair| pair.as_ref().map(|pair| &pair.key))
                .collect()
        }

        // Lấy tất cả các value hiện có.
        pub fn value_set(&self) -> Vec<&String> {
            self.buckets
                .iter()
                .filter_map(|pair| pair.as_ref().map(|pair| &pair.val))
                .collect()
        }

        // In toàn bộ nội dung của bảng băm để dễ dàng kiểm tra.
        pub fn print(&self) {
            for pair in self.entry_set() {
                println!("{} -> {}", pair.key, pair.val);
            }
        }
    }
    ```

=== "C"

    ```c title="array_hash_map.c"
    [class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

=== "Kotlin"

    ```kotlin title="array_hash_map.kt"
    [class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

=== "Ruby"

    ```ruby title="array_hash_map.rb"
    [class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

=== "Zig"

    ```zig title="array_hash_map.zig"
    [class]{Pair}-[func]{}

    [class]{ArrayHashMap}-[func]{}
    ```

## 6.1.3 &nbsp; Xung đột băm và thay đổi kích thước

Về bản chất, vai trò của hàm băm là ánh xạ toàn bộ không gian đầu vào của tất cả các key vào không gian đầu ra của tất cả các chỉ số mảng. Tuy nhiên, không gian đầu vào thường lớn hơn nhiều so với không gian đầu ra. Do đó, **về mặt lý thuyết, sẽ luôn có trường hợp "nhiều đầu vào tương ứng với cùng một đầu ra"**.

Trong ví dụ trên, với hàm băm đã cho, khi hai chữ số cuối của `key` đầu vào giống nhau, hàm băm sẽ tạo ra cùng một đầu ra. Ví dụ: khi truy vấn hai học sinh có mã số học sinh là 12836 và 20336, chúng ta thấy:

```shell
12836 % 100 = 36
20336 % 100 = 36
```

Như trong hình bên dưới, cả hai mã số học sinh đều trỏ đến cùng một tên, điều này rõ ràng là không chính xác. Tình huống mà nhiều đầu vào tương ứng với cùng một đầu ra này được gọi là <u>xung đột băm</u> (hash collision).

![Ví dụ về xung đột băm](hash_map.assets/hash_collision.png){ class="animation-figure" }

<p align="center"> Figure 6-3 &nbsp; Ví dụ về xung đột băm</p>

Dễ hiểu rằng khi dung lượng $n$ của bảng băm tăng lên, xác suất nhiều key được gán cho cùng một bucket giảm xuống, dẫn đến ít xung đột hơn. Do đó, **chúng ta có thể giảm xung đột băm bằng cách thay đổi kích thước bảng băm**.

Như trong hình bên dưới, trước khi thay đổi kích thước, các cặp key-value `(136, A)` và `(236, D)` xung đột. Tuy nhiên, sau khi thay đổi kích thước, xung đột được giải quyết.

![Thay đổi kích thước bảng băm](hash_map.assets/hash_table_reshash.png){ class="animation-figure" }

<p align="center"> Figure 6-4 &nbsp; Thay đổi kích thước bảng băm</p>

Tương tự như mở rộng mảng, việc thay đổi kích thước bảng băm yêu cầu di chuyển tất cả các cặp key-value từ bảng băm ban đầu sang bảng băm mới, điều này tốn thời gian. Hơn nữa, vì `capacity` của bảng băm thay đổi, chúng ta cần tính toán lại vị trí lưu trữ của tất cả các cặp key-value bằng hàm băm, làm tăng thêm chi phí tính toán của quá trình thay đổi kích thước. Do đó, các ngôn ngữ lập trình thường phân bổ một dung lượng đủ lớn cho bảng băm để ngăn chặn việc thay đổi kích thước thường xuyên.

<u>Hệ số tải</u> (load factor) là một khái niệm quan trọng trong bảng băm. Nó được định nghĩa là tỷ lệ giữa số lượng phần tử trong bảng băm và số lượng bucket. Nó được sử dụng để đo mức độ nghiêm trọng của xung đột băm và **thường đóng vai trò là yếu tố kích hoạt cho việc thay đổi kích thước bảng băm**. Ví dụ: trong Java, khi hệ số tải vượt quá $0.75$, hệ thống sẽ tăng kích thước bảng băm lên gấp đôi.
