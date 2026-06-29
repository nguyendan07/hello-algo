# Tóm tắt

### Ôn tập trọng tâm

- Thuật toán tham ăn thường được sử dụng để giải quyết các bài toán tối ưu hóa. Nguyên tắc là đưa ra các quyết định tối ưu cục bộ ở mỗi giai đoạn quyết định với hy vọng thu được lời giải tối ưu toàn cục.
- Thuật toán tham ăn thực hiện lặp đi lặp lại từng lựa chọn tham ăn một, chuyển đổi bài toán thành một bài toán con nhỏ hơn trong mỗi vòng, cho đến khi bài toán được giải quyết.
- Thuật toán tham ăn không chỉ đơn giản trong việc triển khai mà còn có hiệu suất giải quyết bài toán cao. So với quy hoạch động, thuật toán tham ăn thường có độ phức tạp thời gian thấp hơn.
- Trong bài toán đổi tiền lẻ, đối với một số tổ hợp đồng xu nhất định, thuật toán tham ăn có thể đảm bảo tìm ra lời giải tối ưu; tuy nhiên đối với các tổ hợp đồng xu khác, thuật toán tham ăn có thể tìm ra các lời giải rất kém.
- Các bài toán phù hợp để giải bằng thuật toán tham ăn có hai tính chất lớn: tính chất lựa chọn tham ăn và cấu trúc con tối ưu. Tính chất lựa chọn tham ăn thể hiện hiệu quả của chiến lược tham ăn.
- Đối với một số bài toán phức tạp, việc chứng minh tính chất lựa chọn tham ăn không hề đơn giản. Tương đối mà nói, việc bác bỏ nó dễ dàng hơn, chẳng hạn như trong bài toán đổi tiền lẻ.
- Việc giải các bài toán tham ăn chủ yếu bao gồm ba bước: phân tích bài toán, xác định chiến lược tham ăn và chứng minh tính đúng đắn. Trong số đó, xác định chiến lược tham ăn là bước cốt lõi, và chứng minh tính đúng đắn thường là khó khăn chính.
- Bài toán cái túi phân số, dựa trên bài toán cái túi 0-1, cho phép chọn các phần nhỏ của đồ vật, và do đó có thể được giải bằng thuật toán tham ăn. Tính đúng đắn của chiến lược tham ăn có thể được chứng minh bằng phương pháp chứng minh phản chứng.
- Bài toán dung tích tối đa có thể được giải bằng liệt kê vét cạn với độ phức tạp thời gian $O(n^2)$. Bằng cách thiết kế một chiến lược tham ăn dịch chuyển cạnh ngắn hơn vào trong ở mỗi vòng, độ phức tạp thời gian có thể được tối ưu hóa thành $O(n)$.
- Trong bài toán tích lớn nhất khi cắt đoạn, chúng ta liên tiếp rút ra hai chiến lược tham ăn: các số nguyên $\geq 4$ đều nên tiếp tục được tách ra, và thừa số tách tối ưu là $3$. Mã nguồn bao gồm các thao tác tính lũy thừa, và độ phức tạp thời gian phụ thuộc vào phương pháp triển khai lũy thừa, thường là $O(1)$ hoặc $O(\log n)$.
