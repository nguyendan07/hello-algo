# Tóm tắt

### Ôn tập trọng tâm

- Thuật toán quay lui về bản chất là một phương pháp tìm kiếm vét cạn. Nó tìm kiếm các lời giải đáp ứng các điều kiện chỉ định bằng cách thực hiện duyệt theo chiều sâu không gian lời giải. Trong quá trình tìm kiếm, khi tìm thấy một lời giải thỏa mãn các điều kiện, nó sẽ được ghi nhận. Quá trình tìm kiếm kết thúc sau khi tìm thấy tất cả các lời giải hoặc khi quá trình duyệt hoàn tất.
- Quá trình tìm kiếm của thuật toán quay lui bao gồm hai phần: thử nghiệm và quay lui. Nó thử các lựa chọn khác nhau thông qua tìm kiếm theo chiều sâu. Khi gặp các tình huống vi phạm các ràng buộc, nó hoàn tác lựa chọn trước đó, quay trở lại trạng thái trước và tiếp tục khám phá các tùy chọn khác. Thử nghiệm và quay lui là các thao tác theo các hướng ngược nhau.
- Các bài toán quay lui thường chứa nhiều ràng buộc, những ràng buộc này có thể được tận dụng để triển khai các thao tác cắt tỉa. Cắt tỉa có thể chấm dứt sớm các nhánh tìm kiếm không cần thiết, giúp cải thiện đáng kể hiệu suất tìm kiếm.
- Thuật toán quay lui chủ yếu được sử dụng để giải quyết các bài toán tìm kiếm và các bài toán thỏa mãn ràng buộc. Mặc dù các bài toán tối ưu hóa tổ hợp có thể được giải bằng quay lui, thường có sẵn các giải pháp hiệu quả hơn hoặc cho hiệu suất tốt hơn.
- Bài toán hoán vị nhằm mục đích tìm tất cả các hoán vị có thể có của các phần tử trong một tập hợp cho trước. Chúng ta sử dụng một mảng để ghi lại xem mỗi phần tử đã được chọn hay chưa, từ đó cắt tỉa các nhánh tìm kiếm cố gắng chọn lặp lại cùng một phần tử, đảm bảo mỗi phần tử được chọn đúng một lần.
- Trong bài toán hoán vị, nếu tập hợp chứa các phần tử trùng lặp, kết quả cuối cùng sẽ chứa các hoán vị trùng lặp. Chúng ta cần áp đặt một ràng buộc sao cho các phần tử bằng nhau chỉ có thể được chọn một lần trong mỗi vòng, điều này thường đạt được bằng cách sử dụng một tập hợp băm (hash set).
- Bài toán tổng tập con nhằm mục đích tìm tất cả các tập con của một tập hợp cho trước có tổng bằng một giá trị mục tiêu. Vì tập hợp không có thứ tự nhưng quá trình tìm kiếm xuất kết quả theo mọi thứ tự, các tập con trùng lặp sẽ được tạo ra. Chúng ta sắp xếp dữ liệu trước khi quay lui và sử dụng một biến để chỉ ra điểm bắt đầu duyệt của mỗi vòng, từ đó cắt tỉa các nhánh tìm kiếm tạo ra các tập con trùng lặp.
- Đối với bài toán tổng tập con, các phần tử bằng nhau trong mảng tạo ra các tập con trùng lặp. Chúng ta tận dụng điều kiện tiên quyết là mảng đã được sắp xếp bằng cách kiểm tra xem các phần tử kề nhau có bằng nhau hay không để triển khai cắt tỉa, đảm bảo rằng các phần tử bằng nhau chỉ có thể được chọn một lần trong mỗi vòng.
- Bài toán $n$ quân hậu nhằm mục đích tìm các cách đặt $n$ quân hậu trên bàn cờ $n \times n$ sao cho không có hai quân hậu nào có thể tấn công lẫn nhau. Các ràng buộc của bài toán này bao gồm ràng buộc hàng, ràng buộc cột, và các ràng buộc đường chéo chính và phụ. Để thỏa mãn các ràng buộc hàng, chúng ta áp dụng chiến lược đặt theo từng hàng, đảm bảo chính xác một quân hậu được đặt ở mỗi hàng.
- Việc xử lý các ràng buộc cột và ràng buộc đường chéo là tương tự nhau. Đối với các ràng buộc cột, chúng ta sử dụng một mảng để ghi lại xem mỗi cột đã có quân hậu hay chưa, từ đó chỉ ra xem một ô được chọn có hợp lệ hay không. Đối với các ràng buộc đường chéo, chúng ta sử dụng hai mảng để ghi lại riêng biệt xem các quân hậu có tồn tại trên từng đường chéo chính hoặc phụ hay không. Thử thách nằm ở việc tìm ra quy luật chỉ số hàng-cột đặc trưng cho các ô trên cùng một đường chéo chính (phụ).

### Hỏi & Đáp

**Hỏi**: Làm thế nào chúng ta có thể hiểu mối quan hệ giữa quay lui và đệ quy?

Nói một cách tổng thể, quay lui là một chiến lược thuật toán, trong khi đệ quy tốt hơn nên được xem như một công cụ.

- Quay lui thường được triển khai bằng đệ quy. Tuy nhiên, quay lui chỉ là một ứng dụng của đệ quy, cụ thể là việc sử dụng nó trong các bài toán tìm kiếm.
- Cấu trúc của đệ quy phản ánh một mô hình giải quyết bài toán dựa trên việc phân rã một bài toán thành các bài toán con, và nó thường được sử dụng trong chia để trị, quay lui và quy hoạch động (đệ quy có ghi nhớ).
