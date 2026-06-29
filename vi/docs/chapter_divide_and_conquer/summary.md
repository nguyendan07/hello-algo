# Tóm tắt

### Ôn tập trọng tâm

- Chia để trị là một chiến lược thiết kế thuật toán phổ biến bao gồm hai giai đoạn: chia (phân chia) và trị (hợp nhất), và thường được triển khai bằng đệ quy.
- Tiêu chí để xác định một bài toán có phải là bài toán chia để trị hay không bao gồm: bài toán có thể phân rã hay không, các bài toán con có độc lập với nhau hay không, và lời giải của các bài toán con có thể hợp nhất hay không.
- Sắp xếp trộn là một ứng dụng điển hình của chiến lược chia để trị. Nó chia đệ quy một mảng thành hai mảng con có độ dài bằng nhau cho đến khi chỉ còn một phần tử, sau đó hợp nhất chúng lại từng tầng một để hoàn thành việc sắp xếp.
- Việc áp dụng chiến lược chia để trị thường có thể cải thiện hiệu suất thuật toán. Một mặt, nó làm giảm số lượng phép tính; mặt khác, nó giúp cho việc tối ưu hóa song song bởi hệ thống trở nên dễ dàng hơn.
- Chia để trị có thể giải quyết nhiều bài toán thuật toán và cũng được sử dụng rộng rãi trong thiết kế cấu trúc dữ liệu và thuật toán, khiến nó hiện diện khắp nơi.
- So với tìm kiếm vét cạn, tìm kiếm thích ứng có hiệu suất cao hơn. Các thuật toán tìm kiếm có độ phức tạp thời gian $O(\log n)$ thường được triển khai dựa trên chiến lược chia để trị.
- Tìm kiếm nhị phân là một ứng dụng điển hình khác của chia để trị. Nó không bao gồm bước hợp nhất lời giải của các bài toán con. Chúng ta có thể triển khai tìm kiếm nhị phân thông qua chia để trị bằng đệ quy.
- Trong bài toán dựng cây nhị phân, việc dựng cây (bài toán ban đầu) có thể được chia thành việc dựng cây con trái và cây con phải (các bài toán con), điều này có thể đạt được bằng cách chia các khoảng chỉ số của thứ tự duyệt tiền thứ và trung thứ.
- Trong bài toán tháp Hà Nội, một bài toán kích thước $n$ có thể được chia thành hai bài toán con kích thước $n-1$ và một bài toán con kích thước $1$. Sau khi giải ba bài toán con này theo thứ tự, bài toán ban đầu sẽ được giải quyết.
