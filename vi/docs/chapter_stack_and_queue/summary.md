# Tóm tắt

### Ôn tập kiến thức chính

- Stack (Ngăn xếp) là một cấu trúc dữ liệu tuân theo nguyên tắc Last-In-First-Out (LIFO - Vào sau ra trước) và có thể được triển khai bằng mảng hoặc danh sách liên kết.
- Về hiệu quả thời gian, việc triển khai stack bằng mảng có hiệu quả trung bình cao hơn. Tuy nhiên, trong quá trình mở rộng, độ phức tạp thời gian cho một thao tác push có thể giảm xuống $O(n)$. Ngược lại, việc triển khai stack bằng danh sách liên kết mang lại hiệu quả ổn định hơn.
- Về hiệu quả không gian, việc triển khai stack bằng mảng có thể dẫn đến một mức độ lãng phí không gian nhất định. Tuy nhiên, điều quan trọng cần lưu ý là không gian bộ nhớ chiếm bởi các node trong danh sách liên kết thường lớn hơn so với các phần tử trong một mảng.
- Queue (Hàng đợi) là một cấu trúc dữ liệu tuân theo nguyên tắc First-In-First-Out (FIFO - Vào trước ra trước), và nó cũng có thể được triển khai bằng mảng hoặc danh sách liên kết. Các kết luận về hiệu quả thời gian và không gian cho hàng đợi tương tự như đối với stack.
- Double-ended queue (deque - Hàng đợi hai đầu) là một loại hàng đợi linh hoạt hơn, cho phép thêm và xóa các phần tử ở cả hai đầu.

### Hỏi & Đáp

**H**: Chức năng tiến và lùi của trình duyệt có được triển khai bằng danh sách liên kết đôi không?

Việc điều hướng tiến và lùi của trình duyệt về cơ bản là một biểu hiện của khái niệm "stack". Khi người dùng truy cập một trang mới, trang đó được thêm vào đầu stack; khi họ nhấp vào nút quay lại, trang sẽ được lấy ra khỏi đầu stack. Một hàng đợi hai đầu (deque) có thể thuận tiện triển khai một số hoạt động bổ sung, như đã đề cập trong phần "Hàng đợi hai đầu".

**H**: Sau khi lấy một node ra khỏi stack, có cần giải phóng bộ nhớ của node đó không?

Nếu node đã lấy ra vẫn sẽ được sử dụng sau này, thì không cần thiết phải giải phóng bộ nhớ của nó. Trong các ngôn ngữ như Java và Python có tính năng thu gom rác tự động, việc giải phóng bộ nhớ thủ công là không cần thiết; trong C và C++, cần phải giải phóng bộ nhớ thủ công.

**H**: Hàng đợi hai đầu có vẻ giống như hai stack được nối với nhau. Nó có những ứng dụng gì?

Một hàng đợi hai đầu, là sự kết hợp của một stack và một queue hoặc hai stack được nối với nhau, thể hiện cả logic của stack và queue. Do đó, nó có thể triển khai tất cả các ứng dụng của stack và queue đồng thời mang lại sự linh hoạt hơn.

**H**: Chính xác thì undo và redo được triển khai như thế nào?

Các thao tác undo và redo được triển khai bằng hai stack: Stack `A` cho undo và Stack `B` cho redo.

1. Mỗi khi người dùng thực hiện một thao tác, nó sẽ được đẩy vào Stack `A` và Stack `B` sẽ bị xóa.
2. Khi người dùng thực hiện một "undo", thao tác gần đây nhất sẽ được lấy ra khỏi Stack `A` và đẩy vào Stack `B`.
3. Khi người dùng thực hiện một "redo", thao tác gần đây nhất sẽ được lấy ra khỏi Stack `B` và đẩy trở lại Stack `A`.
