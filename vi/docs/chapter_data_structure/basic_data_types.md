# Các kiểu dữ liệu cơ bản

Khi nói về dữ liệu trong máy tính, chúng ta thường nghĩ đến các dạng như văn bản, hình ảnh, video, âm thanh và mô hình 3D. Dù có cách tổ chức khác nhau, tất cả đều được tạo thành từ các kiểu dữ liệu cơ bản.

**Kiểu dữ liệu cơ bản là những kiểu mà CPU có thể thao tác trực tiếp** và được sử dụng trực tiếp trong thuật toán, chủ yếu gồm các loại sau:

- Kiểu số nguyên: `byte`, `short`, `int`, `long`.
- Kiểu số thực: `float`, `double`, dùng để biểu diễn số thập phân.
- Kiểu ký tự: `char`, dùng để biểu diễn chữ cái, dấu câu, thậm chí là emoji ở nhiều ngôn ngữ.
- Kiểu boolean: `bool`, dùng để biểu diễn quyết định "đúng" hoặc "sai".

**Các kiểu dữ liệu cơ bản được lưu trữ trong máy tính dưới dạng nhị phân**. Một chữ số nhị phân là 1 bit. Trong hầu hết hệ điều hành hiện đại, 1 byte gồm 8 bit.

Giá trị mà kiểu dữ liệu cơ bản có thể biểu diễn phụ thuộc vào kích thước bộ nhớ mà nó chiếm. Dưới đây, chúng ta lấy Java làm ví dụ.

- Kiểu số nguyên `byte` chiếm 1 byte = 8 bit và có thể biểu diễn $2^8$ số.
- Kiểu số nguyên `int` chiếm 4 byte = 32 bit và có thể biểu diễn $2^{32}$ số.

Bảng sau liệt kê kích thước bộ nhớ, phạm vi giá trị và giá trị mặc định của các kiểu dữ liệu cơ bản trong Java. Bạn không cần phải ghi nhớ bảng này, chỉ cần hiểu tổng quát và tra cứu khi cần.

<p align="center"> Bảng <id> &nbsp; Kích thước bộ nhớ và phạm vi giá trị của các kiểu dữ liệu cơ bản </p>

| Kiểu     | Ký hiệu   | Kích thước     | Giá trị nhỏ nhất         | Giá trị lớn nhất         | Giá trị mặc định |
| -------- | --------- | -------------- | ------------------------ | ------------------------ | --------------- |
| Số nguyên| `byte`    | 1 byte         | $-2^7$ ($-128$)          | $2^7 - 1$ ($127$)        | 0               |
|          | `short`   | 2 byte         | $-2^{15}$                | $2^{15} - 1$             | 0               |
|          | `int`     | 4 byte         | $-2^{31}$                | $2^{31} - 1$             | 0               |
|          | `long`    | 8 byte         | $-2^{63}$                | $2^{63} - 1$             | 0               |
| Số thực  | `float`   | 4 byte         | $1.175 \times 10^{-38}$  | $3.403 \times 10^{38}$   | $0.0\text{f}$   |
|          | `double`  | 8 byte         | $2.225 \times 10^{-308}$ | $1.798 \times 10^{308}$  | 0.0             |
| Ký tự    | `char`    | 2 byte         | 0                        | $2^{16} - 1$             | 0               |
| Boolean  | `bool`    | 1 byte         | $\text{false}$           | $\text{true}$            | $\text{false}$  |

Lưu ý rằng bảng trên chỉ áp dụng cho các kiểu dữ liệu cơ bản của Java. Mỗi ngôn ngữ lập trình đều có định nghĩa kiểu dữ liệu riêng, có thể khác về kích thước, phạm vi giá trị và giá trị mặc định.

- Trong Python, kiểu số nguyên `int` có thể lớn tùy ý, chỉ giới hạn bởi bộ nhớ; kiểu số thực `float` là số thực 64 bit; không có kiểu `char`, một ký tự thực chất là chuỗi `str` có độ dài 1.
- C và C++ không quy định kích thước kiểu dữ liệu cơ bản, nó phụ thuộc vào nền tảng và trình biên dịch. Bảng trên theo mô hình dữ liệu LP64 [data model](https://en.cppreference.com/w/cpp/language/types#Properties), dùng cho hệ điều hành Unix 64-bit như Linux và macOS.
- Kích thước của `char` trong C và C++ là 1 byte, còn ở nhiều ngôn ngữ khác thì phụ thuộc vào phương pháp mã hóa ký tự, sẽ được nói rõ ở chương "Mã hóa ký tự".
- Dù kiểu boolean chỉ cần 1 bit (0 hoặc 1), nhưng thường được lưu trong bộ nhớ là 1 byte. Lý do là CPU hiện đại thường dùng 1 byte là đơn vị nhỏ nhất để truy cập bộ nhớ.

Vậy, kiểu dữ liệu cơ bản liên quan gì đến cấu trúc dữ liệu? Chúng ta biết rằng cấu trúc dữ liệu là cách tổ chức và lưu trữ dữ liệu trong máy tính. Ở đây, trọng tâm là "cấu trúc" chứ không phải "dữ liệu".

Nếu muốn biểu diễn "một dãy số", chúng ta thường nghĩ đến mảng. Vì cấu trúc tuyến tính của mảng giúp biểu diễn sự liền kề và thứ tự của các số, còn việc lưu trữ là số nguyên `int`, số thực `float` hay ký tự `char` thì không liên quan đến "cấu trúc dữ liệu".

Nói cách khác, **kiểu dữ liệu cơ bản cung cấp "kiểu nội dung" của dữ liệu, còn cấu trúc dữ liệu cung cấp "cách tổ chức" dữ liệu**. Ví dụ, trong đoạn mã dưới đây, chúng ta dùng cùng một cấu trúc dữ liệu (mảng) để lưu trữ và biểu diễn các kiểu dữ liệu cơ bản khác nhau như `int`, `float`, `char`, `bool`, v.v.

=== "Python"

    ```python title=""
    # Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    numbers: list[int] = [0] * 5
    decimals: list[float] = [0.0] * 5
    # Ký tự trong Python thực chất là chuỗi có độ dài 1
    characters: list[str] = ['0'] * 5
    bools: list[bool] = [False] * 5
    # List trong Python có thể lưu trữ tự do các kiểu dữ liệu cơ bản và tham chiếu đối tượng
    data = [0, 0.0, 'a', False, ListNode(0)]
    ```

=== "C++"

    ```cpp title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    int numbers[5];
    float decimals[5];
    char characters[5];
    bool bools[5];
    ```

=== "Java"

    ```java title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    int[] numbers = new int[5];
    float[] decimals = new float[5];
    char[] characters = new char[5];
    boolean[] bools = new boolean[5];
    ```

=== "C#"

    ```csharp title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    int[] numbers = new int[5];
    float[] decimals = new int[5];
    char[] characters = new char[5];
    bool[] bools = new bool[5];
    ```

=== "Go"

    ```go title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    var numbers = [5]int{}
    var decimals = [5]float64{}
    var characters = [5]byte{}
    var bools = [5]bool{}
    ```

=== "Swift"

    ```swift title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    let numbers = Array(repeating: 0, count: 5)
    let decimals = Array(repeating: 0.0, count: 5)
    let characters: [Character] = Array(repeating: "a", count: 5)
    let bools = Array(repeating: false, count: 5)
    ```

=== "JS"

    ```javascript title=""
    // Mảng trong JavaScript có thể lưu trữ tự do các kiểu dữ liệu cơ bản và đối tượng
    const array = [0, 0.0, 'a', false];
    ```

=== "TS"

    ```typescript title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    const numbers: number[] = [];
    const characters: string[] = [];
    const bools: boolean[] = [];
    ```

=== "Dart"

    ```dart title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    List<int> numbers = List.filled(5, 0);
    List<double> decimals = List.filled(5, 0.0);
    List<String> characters = List.filled(5, 'a');
    List<bool> bools = List.filled(5, false);
    ```

=== "Rust"

    ```rust title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    let numbers: Vec<i32> = vec![0; 5];
    let decimals: Vec<f32> = vec![0.0, 5];
    let characters: Vec<char> = vec!['0'; 5];
    let bools: Vec<bool> = vec![false; 5];
    ```

=== "C"

    ```c title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    int numbers[10];
    float decimals[10];
    char characters[10];
    bool bools[10];
    ```

=== "Kotlin"

    ```kotlin title=""

    ```

=== "Zig"

    ```zig title=""
    // Khởi tạo mảng với các kiểu dữ liệu cơ bản khác nhau
    var numbers: [5]i32 = undefined;
    var decimals: [5]f32 = undefined;
    var characters: [5]u8 = undefined;
    var bools: [5]bool = undefined;
    ```
