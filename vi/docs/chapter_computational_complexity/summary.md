# Tóm tắt

### Ôn tập chính

**Đánh giá hiệu quả thuật toán**

- Hiệu quả về thời gian và hiệu quả về bộ nhớ là hai tiêu chí chính để đánh giá chất lượng của một thuật toán.
- Ta có thể đánh giá hiệu quả thuật toán bằng cách kiểm thử thực tế, nhưng khó loại bỏ ảnh hưởng của môi trường kiểm thử và tốn nhiều tài nguyên tính toán.
- Phân tích độ phức tạp giúp khắc phục nhược điểm của kiểm thử thực tế. Kết quả phân tích áp dụng cho mọi nền tảng và cho thấy hiệu quả của thuật toán ở các quy mô dữ liệu khác nhau.

**Độ phức tạp thời gian**

- Độ phức tạp thời gian đo xu hướng thời gian chạy của thuật toán khi khối lượng dữ liệu tăng lên, giúp đánh giá hiệu quả thuật toán. Tuy nhiên, nó có thể không chính xác trong một số trường hợp, ví dụ khi dữ liệu đầu vào nhỏ hoặc khi các thuật toán có cùng độ phức tạp thời gian, khiến việc so sánh hiệu quả trở nên khó khăn.
- Độ phức tạp thời gian trường hợp xấu nhất được ký hiệu bằng big-$O$, biểu thị giới hạn trên tiệm cận, phản ánh mức tăng số phép toán $T(n)$ khi $n$ tiến tới vô cùng.
- Để tính độ phức tạp thời gian, ta thực hiện hai bước: đếm số phép toán, sau đó xác định giới hạn trên tiệm cận.
- Các độ phức tạp thời gian phổ biến, sắp xếp từ thấp đến cao, gồm $O(1)$, $O(\log n)$, $O(n)$, $O(n \log n)$, $O(n^2)$, $O(2^n)$ và $O(n!)$, v.v.
- Độ phức tạp thời gian của một số thuật toán không cố định, phụ thuộc vào phân bố dữ liệu đầu vào. Độ phức tạp thời gian được chia thành trường hợp xấu nhất, tốt nhất và trung bình. Trường hợp tốt nhất ít được dùng vì dữ liệu đầu vào thường phải đáp ứng điều kiện rất đặc biệt.
- Độ phức tạp thời gian trung bình phản ánh hiệu quả của thuật toán khi dữ liệu đầu vào ngẫu nhiên, gần giống với hiệu suất thực tế. Để tính độ phức tạp trung bình, cần xét đến phân bố dữ liệu đầu vào và kỳ vọng toán học.

**Độ phức tạp bộ nhớ**

- Độ phức tạp bộ nhớ, giống như độ phức tạp thời gian, đo xu hướng bộ nhớ mà thuật toán sử dụng khi khối lượng dữ liệu tăng lên.
- Bộ nhớ sử dụng khi thuật toán chạy gồm bộ nhớ đầu vào, bộ nhớ tạm thời và bộ nhớ đầu ra. Thông thường, bộ nhớ đầu vào không tính vào độ phức tạp bộ nhớ. Bộ nhớ tạm thời gồm dữ liệu tạm, bộ nhớ khung ngăn xếp và bộ nhớ lệnh; trong đó bộ nhớ khung ngăn xếp thường chỉ ảnh hưởng đến độ phức tạp bộ nhớ trong hàm đệ quy.
- Ta thường chỉ quan tâm đến độ phức tạp bộ nhớ trường hợp xấu nhất, tức là tính độ phức tạp bộ nhớ của thuật toán với dữ liệu đầu vào xấu nhất và tại thời điểm sử dụng bộ nhớ nhiều nhất.
- Các độ phức tạp bộ nhớ phổ biến, sắp xếp từ thấp đến cao, gồm $O(1)$, $O(\log n)$, $O(n)$, $O(n^2)$ và $O(2^n)$, v.v.

### Hỏi & Đáp

**Hỏi**: Độ phức tạp bộ nhớ của đệ quy đuôi có phải là $O(1)$ không?

Về lý thuyết, độ phức tạp bộ nhớ của hàm đệ quy đuôi có thể tối ưu về $O(1)$. Tuy nhiên, hầu hết các ngôn ngữ lập trình (như Java, Python, C++, Go, C#) không tự động tối ưu đệ quy đuôi, nên thường được coi là có độ phức tạp bộ nhớ $O(n)$.

**Hỏi**: Sự khác biệt giữa "hàm" và "phương thức" là gì?

<u>Hàm</u> có thể thực thi độc lập, với tất cả tham số được truyền rõ ràng. <u>Phương thức</u> gắn với một đối tượng và được truyền ngầm đối tượng gọi nó, có thể thao tác trên dữ liệu của một đối tượng thuộc lớp.

Một số ví dụ từ các ngôn ngữ lập trình phổ biến:

- C là ngôn ngữ lập trình thủ tục, không có khái niệm hướng đối tượng, nên chỉ có hàm. Tuy nhiên, ta có thể mô phỏng lập trình hướng đối tượng bằng cách tạo cấu trúc (struct), và các hàm liên kết với cấu trúc này tương đương với phương thức ở các ngôn ngữ khác.
- Java và C# là ngôn ngữ hướng đối tượng, nơi các khối mã (phương thức) thường nằm trong lớp. Phương thức tĩnh hoạt động giống như hàm vì nó gắn với lớp và không truy cập được biến của đối tượng cụ thể.
- C++ và Python hỗ trợ cả lập trình thủ tục (hàm) và lập trình hướng đối tượng (phương thức).

**Hỏi**: Hình "Các loại độ phức tạp bộ nhớ phổ biến" có phản ánh kích thước bộ nhớ tuyệt đối không?

Không, hình chỉ thể hiện độ phức tạp bộ nhớ, tức là xu hướng tăng trưởng, chứ không phải kích thước bộ nhớ tuyệt đối.

Nếu bạn lấy $n = 8$, có thể thấy giá trị của mỗi đường cong không khớp với hàm của nó. Đó là vì mỗi đường cong đều có một hằng số, nhằm nén phạm vi giá trị lại cho dễ nhìn.

Trong thực tế, vì ta thường không biết "hằng số" độ phức tạp của từng phương pháp, nên không thể chọn giải pháp tốt nhất cho $n = 8$ chỉ dựa vào độ phức tạp. Tuy nhiên, với $n = 8^5$, việc lựa chọn sẽ dễ hơn nhiều vì xu hướng tăng trưởng sẽ chiếm ưu thế.
