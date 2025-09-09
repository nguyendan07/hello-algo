# Đánh giá hiệu quả thuật toán

Khi thiết kế thuật toán, chúng ta theo đuổi hai mục tiêu sau theo thứ tự.

1. **Tìm ra giải pháp cho bài toán**: Thuật toán cần đảm bảo tìm được lời giải đúng trong phạm vi đầu vào đã cho.
2. **Tìm kiếm giải pháp tối ưu**: Với cùng một bài toán, có thể có nhiều cách giải khác nhau, và chúng ta hướng tới việc tìm ra thuật toán hiệu quả nhất.

Nói cách khác, khi đã có thể giải quyết được bài toán, hiệu quả của thuật toán trở thành tiêu chí chính để đánh giá, bao gồm hai khía cạnh sau:

- **Hiệu quả thời gian**: Tốc độ thực thi của thuật toán.
- **Hiệu quả không gian**: Lượng bộ nhớ mà thuật toán sử dụng.

Tóm lại, **mục tiêu của chúng ta là thiết kế cấu trúc dữ liệu và thuật toán vừa nhanh vừa tiết kiệm bộ nhớ**. Đánh giá hiệu quả thuật toán là rất quan trọng, vì nhờ đó chúng ta có thể so sánh các thuật toán khác nhau và định hướng quá trình thiết kế, tối ưu thuật toán.

Có hai phương pháp chính để đánh giá hiệu quả: kiểm thử thực tế và ước lượng lý thuyết.

## Kiểm thử thực tế

Giả sử chúng ta có hai thuật toán `A` và `B`, đều giải được cùng một bài toán, và cần so sánh hiệu quả của chúng. Cách trực tiếp nhất là chạy hai thuật toán này trên máy tính, theo dõi và ghi lại thời gian chạy cũng như lượng bộ nhớ sử dụng. Phương pháp này phản ánh thực tế, nhưng có nhiều hạn chế.

Một mặt, **rất khó loại bỏ ảnh hưởng từ môi trường kiểm thử**. Cấu hình phần cứng có thể ảnh hưởng đến hiệu năng thuật toán. Ví dụ, thuật toán có khả năng xử lý song song cao sẽ chạy tốt hơn trên CPU đa nhân, còn thuật toán thao tác nhiều với bộ nhớ sẽ chạy tốt hơn với RAM hiệu năng cao. Kết quả kiểm thử có thể khác nhau trên các máy khác nhau. Điều này khiến việc kiểm thử trên nhiều máy để tính trung bình hiệu quả trở nên không khả thi.

Mặt khác, **kiểm thử toàn diện rất tốn tài nguyên**. Hiệu quả thuật toán thay đổi theo kích thước dữ liệu đầu vào. Ví dụ, với dữ liệu nhỏ, thuật toán `A` có thể chạy nhanh hơn `B`, nhưng với dữ liệu lớn, kết quả có thể ngược lại. Vì vậy, để có kết luận thuyết phục, cần kiểm thử với nhiều kích thước dữ liệu, đòi hỏi rất nhiều tài nguyên tính toán.

## Ước lượng lý thuyết

Do kiểm thử thực tế có nhiều hạn chế, chúng ta có thể đánh giá hiệu quả thuật toán chỉ bằng cách tính toán. Phương pháp này gọi là <u>phân tích độ phức tạp tiệm cận</u>, hay đơn giản là <u>phân tích độ phức tạp</u>.

Phân tích độ phức tạp phản ánh mối quan hệ giữa tài nguyên thời gian và không gian cần thiết để thực thi thuật toán với kích thước dữ liệu đầu vào. **Nó mô tả xu hướng tăng lên của thời gian và bộ nhớ khi kích thước dữ liệu đầu vào tăng**. Định nghĩa này nghe có vẻ phức tạp, nhưng chúng ta có thể hiểu qua ba điểm chính sau:

- "Tài nguyên thời gian và không gian" tương ứng với <u>độ phức tạp thời gian</u> và <u>độ phức tạp không gian</u>.
- "Khi kích thước dữ liệu đầu vào tăng" nghĩa là độ phức tạp phản ánh mối quan hệ giữa hiệu quả thuật toán và lượng dữ liệu đầu vào.
- "Xu hướng tăng lên của thời gian và bộ nhớ" nghĩa là phân tích độ phức tạp không tập trung vào giá trị cụ thể của thời gian chạy hay bộ nhớ sử dụng, mà chú trọng vào "tốc độ" tăng lên của chúng.

**Phân tích độ phức tạp khắc phục được nhược điểm của kiểm thử thực tế**, thể hiện ở các điểm sau:

- Không cần chạy mã nguồn thực tế, tiết kiệm năng lượng và thân thiện với môi trường.
- Không phụ thuộc vào môi trường kiểm thử, áp dụng được trên mọi nền tảng.
- Phản ánh hiệu quả thuật toán với các kích thước dữ liệu khác nhau, đặc biệt là khi dữ liệu lớn.

!!! tip

    Nếu bạn vẫn còn bối rối về khái niệm độ phức tạp, đừng lo lắng. Chúng ta sẽ tìm hiểu kỹ hơn ở các chương sau.

Phân tích độ phức tạp cung cấp cho chúng ta một "thước đo" để đánh giá hiệu quả thuật toán, giúp đo lường tài nguyên thời gian và không gian cần thiết để thực thi, và so sánh hiệu quả giữa các thuật toán khác nhau.

Độ phức tạp là một khái niệm toán học, có thể khá trừu tượng và khó hiểu với người mới bắt đầu. Vì vậy, phân tích độ phức tạp có thể chưa phải là chủ đề phù hợp để học đầu tiên. Tuy nhiên, khi tìm hiểu về đặc điểm của một cấu trúc dữ liệu hay thuật toán cụ thể, chúng ta khó tránh khỏi việc phân tích tốc độ và bộ nhớ sử dụng của nó.

Tóm lại, bạn nên có hiểu biết cơ bản về phân tích độ phức tạp trước khi đi sâu vào cấu trúc dữ liệu và thuật toán, **để có thể tự phân tích độ phức tạp của các thuật toán đơn giản**.
