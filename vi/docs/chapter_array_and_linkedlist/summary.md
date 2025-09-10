# Tóm tắt

### Ôn tập các điểm chính

- Mảng (Array) và danh sách liên kết (Linked List) là hai cấu trúc dữ liệu cơ bản, đại diện cho hai phương pháp lưu trữ trong bộ nhớ máy tính: lưu trữ không gian liền kề và lưu trữ không gian không liền kề. Đặc điểm của chúng bổ sung cho nhau.
- Mảng hỗ trợ truy cập ngẫu nhiên và sử dụng ít bộ nhớ hơn; tuy nhiên, chúng kém hiệu quả trong việc chèn và xóa các phần tử và có độ dài cố định sau khi khởi tạo.
- Danh sách liên kết thực hiện chèn và xóa nút hiệu quả thông qua thay đổi tham chiếu (con trỏ) và có thể điều chỉnh độ dài một cách linh hoạt; tuy nhiên, chúng có hiệu quả truy cập nút thấp hơn và tiêu thụ nhiều bộ nhớ hơn.
- Các loại danh sách liên kết phổ biến bao gồm danh sách liên kết đơn, danh sách liên kết vòng và danh sách liên kết đôi, mỗi loại có các tình huống ứng dụng riêng.
- Danh sách (List) là các tập hợp các phần tử có thứ tự, hỗ trợ thêm, xóa và sửa đổi, thường được triển khai dựa trên mảng động, giữ lại những ưu điểm của mảng đồng thời cho phép điều chỉnh độ dài linh hoạt.
- Sự ra đời của danh sách đã cải thiện đáng kể tính thực tế của mảng nhưng có thể dẫn đến lãng phí một số không gian bộ nhớ.
- Trong quá trình thực thi chương trình, dữ liệu chủ yếu được lưu trữ trong bộ nhớ. Mảng cung cấp hiệu quả không gian bộ nhớ cao hơn, trong khi danh sách liên kết linh hoạt hơn trong việc sử dụng bộ nhớ.
- Bộ nhớ cache (Cache) cung cấp khả năng truy cập dữ liệu nhanh chóng cho CPU thông qua các cơ chế như dòng cache, tìm nạp trước, tính cục bộ không gian và tính cục bộ thời gian, giúp tăng cường đáng kể hiệu quả thực thi chương trình.
- Do tỷ lệ命中cache cao hơn, mảng thường hiệu quả hơn danh sách liên kết. Khi chọn cấu trúc dữ liệu, nên đưa ra lựa chọn phù hợp dựa trên nhu cầu và tình huống cụ thể.

### Hỏi & Đáp

**H**: Việc lưu trữ mảng trên stack so với heap có ảnh hưởng đến hiệu quả thời gian và không gian không?

Mảng được lưu trữ trên cả stack và heap đều được lưu trữ trong không gian bộ nhớ liền kề và hiệu quả hoạt động dữ liệu về cơ bản là giống nhau. Tuy nhiên, stack và heap có những đặc điểm riêng, dẫn đến những khác biệt sau.

1. Hiệu quả cấp phát và giải phóng: Stack là một khối bộ nhớ nhỏ hơn, được trình biên dịch tự động cấp phát; bộ nhớ heap tương đối lớn hơn và có thể được cấp phát động trong code, dễ bị phân mảnh hơn. Do đó, các hoạt động cấp phát và giải phóng trên heap thường chậm hơn so với trên stack.
2. Giới hạn kích thước: Bộ nhớ stack tương đối nhỏ, trong khi kích thước heap thường bị giới hạn bởi bộ nhớ khả dụng. Do đó, heap phù hợp hơn để lưu trữ các mảng lớn.
3. Tính linh hoạt: Kích thước của mảng trên stack cần được xác định tại thời điểm biên dịch, trong khi kích thước của mảng trên heap có thể được xác định động tại thời điểm chạy.

**H**: Tại sao mảng yêu cầu các phần tử cùng loại, trong khi danh sách liên kết không nhấn mạnh các phần tử cùng loại?

Danh sách liên kết bao gồm các nút được kết nối bằng các tham chiếu (con trỏ) và mỗi nút có thể lưu trữ dữ liệu thuộc các loại khác nhau, chẳng hạn như int, double, string, object, v.v.

Ngược lại, các phần tử mảng phải cùng loại, cho phép tính toán offset để truy cập các vị trí phần tử tương ứng. Ví dụ: một mảng chứa cả kiểu int và long, với các phần tử đơn chiếm lần lượt 4 byte và 8 byte, không thể sử dụng công thức sau để tính toán offset, vì mảng chứa các phần tử có hai độ dài khác nhau.

```shell
# Địa chỉ bộ nhớ phần tử = địa chỉ bộ nhớ mảng + độ dài phần tử * chỉ số phần tử
```

**H**: Sau khi xóa một nút, có cần thiết phải đặt `P.next` thành `None` không?

Không sửa đổi `P.next` cũng có thể chấp nhận được. Từ góc độ của danh sách liên kết, việc duyệt từ nút đầu đến nút cuối sẽ không còn gặp `P`. Điều này có nghĩa là nút `P` đã được loại bỏ khỏi danh sách một cách hiệu quả và nơi `P` trỏ đến không còn ảnh hưởng đến danh sách nữa.

Từ góc độ thu gom rác, đối với các ngôn ngữ có cơ chế thu gom rác tự động như Java, Python và Go, việc nút `P` có được thu gom hay không phụ thuộc vào việc có còn tham chiếu nào trỏ đến nó hay không, chứ không phải vào giá trị của `P.next`. Trong các ngôn ngữ như C và C++, chúng ta cần giải phóng bộ nhớ của nút theo cách thủ công.

**H**: Trong danh sách liên kết, độ phức tạp thời gian cho các thao tác chèn và xóa là `O(1)`. Nhưng việc tìm kiếm phần tử trước khi chèn hoặc xóa mất thời gian `O(n)`, vậy tại sao độ phức tạp thời gian không phải là `O(n)`?

Nếu một phần tử được tìm kiếm trước và sau đó bị xóa, thì độ phức tạp thời gian thực sự là `O(n)`. Tuy nhiên, lợi thế `O(1)` của danh sách liên kết trong việc chèn và xóa có thể được nhận ra trong các ứng dụng khác. Ví dụ: trong việc triển khai hàng đợi hai đầu bằng danh sách liên kết, chúng ta duy trì các con trỏ luôn trỏ đến các nút đầu và cuối, làm cho mỗi thao tác chèn và xóa là `O(1)`.

**H**: Trong hình "Định nghĩa danh sách liên kết và phương pháp lưu trữ", các nút lưu trữ màu xanh lam nhạt có chiếm một địa chỉ bộ nhớ duy nhất hay chúng chia sẻ một nửa với giá trị nút?

Hình ảnh chỉ là một biểu diễn định tính; phân tích định lượng phụ thuộc vào các tình huống cụ thể.

- Các loại giá trị nút khác nhau chiếm dung lượng khác nhau, chẳng hạn như int, long, double và các thể hiện đối tượng.
- Dung lượng bộ nhớ mà các biến con trỏ chiếm phụ thuộc vào hệ điều hành và môi trường biên dịch được sử dụng, thường là 8 byte hoặc 4 byte.

**H**: Việc thêm các phần tử vào cuối danh sách có luôn là `O(1)` không?

Nếu việc thêm một phần tử vượt quá độ dài danh sách, danh sách cần được mở rộng trước. Hệ thống sẽ yêu cầu một khối bộ nhớ mới và di chuyển tất cả các phần tử của danh sách ban đầu sang, trong trường hợp đó độ phức tạp thời gian trở thành `O(n)`.

**H**: Câu nói "Sự ra đời của danh sách cải thiện đáng kể tính thực tế của mảng, nhưng có thể dẫn đến lãng phí một số không gian bộ nhớ" - điều này có đề cập đến bộ nhớ bị chiếm bởi các biến bổ sung như capacity, length và hệ số mở rộng không?

Sự lãng phí không gian ở đây chủ yếu đề cập đến hai khía cạnh: một mặt, danh sách được đặt với độ dài ban đầu, mà chúng ta có thể không phải lúc nào cũng cần; mặt khác, để ngăn chặn việc mở rộng thường xuyên, việc mở rộng thường nhân với một hệ số, chẳng hạn như $\times 1.5$. Điều này dẫn đến nhiều vị trí trống mà chúng ta thường không thể lấp đầy hoàn toàn.

**H**: Trong Python, sau khi khởi tạo `n = [1, 2, 3]`, địa chỉ của 3 phần tử này là liền kề, nhưng việc khởi tạo `m = [2, 1, 3]` cho thấy rằng `id` của mỗi phần tử không liên tiếp mà giống với các phần tử trong `n`. Nếu địa chỉ của các phần tử này không liền kề, thì `m` có còn là một mảng không?

Nếu chúng ta thay thế các phần tử danh sách bằng các nút danh sách liên kết `n = [n1, n2, n3, n4, n5]`, thì 5 đối tượng nút này cũng thường được phân tán khắp bộ nhớ. Tuy nhiên, với một chỉ số danh sách, chúng ta vẫn có thể truy cập địa chỉ bộ nhớ của nút trong thời gian `O(1)`, do đó truy cập nút tương ứng. Điều này là do mảng lưu trữ các tham chiếu đến các nút, chứ không phải chính các nút.

Không giống như nhiều ngôn ngữ, trong Python, các số cũng được bao bọc dưới dạng đối tượng và danh sách lưu trữ các tham chiếu đến các số này, chứ không phải chính các số đó. Do đó, chúng ta thấy rằng cùng một số trong hai mảng có cùng `id` và địa chỉ bộ nhớ của các số này không cần phải liền kề.

**H**: `std::list` trong C++ STL đã triển khai danh sách liên kết đôi, nhưng có vẻ như một số sách thuật toán không sử dụng trực tiếp nó. Có bất kỳ hạn chế nào không?

Một mặt, chúng ta thường thích sử dụng mảng để triển khai các thuật toán, chỉ sử dụng danh sách liên kết khi cần thiết, chủ yếu vì hai lý do.

- Chi phí không gian: Vì mỗi phần tử yêu cầu hai con trỏ bổ sung (một cho phần tử trước và một cho phần tử tiếp theo), `std::list` thường chiếm nhiều không gian hơn `std::vector`.
- Không thân thiện với bộ nhớ cache: Vì dữ liệu không được lưu trữ liên tục, `std::list` có tỷ lệ sử dụng bộ nhớ cache thấp hơn. Nói chung, `std::vector` hoạt động tốt hơn.

Mặt khác, danh sách liên kết chủ yếu cần thiết cho cây nhị phân và đồ thị. Stack và queue thường được triển khai bằng các lớp `stack` và `queue` của ngôn ngữ lập trình, thay vì danh sách liên kết.

**H**: Việc khởi tạo một danh sách `res = [0] * self.size()` có dẫn đến việc mỗi phần tử của `res` tham chiếu đến cùng một địa chỉ không?

Không. Tuy nhiên, vấn đề này phát sinh với mảng hai chiều, ví dụ: việc khởi tạo một danh sách hai chiều `res = [[0]] * self.size()` sẽ tham chiếu đến cùng một danh sách `[0]` nhiều lần.

**H**: Khi xóa một nút, có cần thiết phải phá vỡ tham chiếu đến nút kế nhiệm của nó không?

Từ góc độ cấu trúc dữ liệu và thuật toán (giải quyết vấn đề), việc không phá vỡ liên kết là ổn, miễn là logic của chương trình là chính xác. Từ góc độ của các thư viện tiêu chuẩn, việc phá vỡ liên kết an toàn hơn và rõ ràng hơn về mặt logic. Nếu liên kết không bị phá vỡ và nút bị xóa không được tái chế đúng cách, nó có thể ảnh hưởng đến việc tái chế bộ nhớ của nút kế nhiệm.
