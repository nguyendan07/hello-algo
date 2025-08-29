# Tóm tắt

- Thuật toán xuất hiện ở khắp nơi trong cuộc sống hàng ngày và không hề khó tiếp cận hay phức tạp như chúng ta nghĩ. Thực tế, chúng ta đã vô thức học được nhiều thuật toán để giải quyết các vấn đề khác nhau trong cuộc sống.
- Nguyên tắc tra cứu một từ trong từ điển giống với thuật toán tìm kiếm nhị phân. Thuật toán tìm kiếm nhị phân thể hiện ý tưởng quan trọng của thuật toán là chia để trị.
- Quá trình sắp xếp các lá bài giống với thuật toán sắp xếp chèn. Thuật toán sắp xếp chèn phù hợp để sắp xếp các bộ dữ liệu nhỏ.
- Các bước đổi tiền lẻ thực chất tuân theo thuật toán tham lam, mỗi bước đều chọn phương án tốt nhất tại thời điểm đó.
- Thuật toán là tập hợp các bước hướng dẫn cụ thể để giải quyết một vấn đề trong thời gian hữu hạn, còn cấu trúc dữ liệu là cách tổ chức và lưu trữ dữ liệu trong máy tính.
- Cấu trúc dữ liệu và thuật toán có mối liên hệ chặt chẽ. Cấu trúc dữ liệu là nền tảng của thuật toán, còn thuật toán là nơi phát huy chức năng của cấu trúc dữ liệu.
- Chúng ta có thể so sánh cấu trúc dữ liệu và thuật toán với việc lắp ráp các khối xây dựng. Các khối là dữ liệu, hình dạng và cách kết nối các khối là cấu trúc dữ liệu, còn các bước lắp ráp chính là thuật toán.

### Hỏi & Đáp

**Hỏi**: Là một lập trình viên, tôi hiếm khi phải tự cài đặt thuật toán trong công việc hàng ngày. Hầu hết các thuật toán thường dùng đều đã có sẵn trong ngôn ngữ lập trình và thư viện, chỉ cần dùng là được. Điều này có nghĩa là các vấn đề trong công việc chưa đủ phức tạp để cần tự thiết kế thuật toán riêng?

Nếu các kỹ năng làm việc cụ thể giống như "chiêu thức" trong võ thuật, thì các môn học nền tảng giống như "nội công".

Tôi cho rằng ý nghĩa của việc học thuật toán (và các môn nền tảng khác) không nhất thiết là để tự cài đặt lại từ đầu khi làm việc, mà là giúp chúng ta đưa ra quyết định và giải quyết vấn đề chuyên nghiệp hơn dựa trên hiểu biết vững chắc về các khái niệm. Nhờ đó, chất lượng công việc cũng được nâng cao. Ví dụ, mọi ngôn ngữ lập trình đều cung cấp hàm sắp xếp có sẵn:

- Nếu chưa học về cấu trúc dữ liệu và thuật toán, khi có dữ liệu cần sắp xếp, chúng ta chỉ đơn giản đưa vào hàm sắp xếp có sẵn. Hàm này chạy mượt, hiệu năng tốt, tưởng như không có vấn đề gì.
- Tuy nhiên, nếu đã học thuật toán, chúng ta biết rằng độ phức tạp thời gian của hàm sắp xếp có sẵn thường là $O(n \log n)$. Hơn nữa, nếu dữ liệu là các số nguyên có số chữ số cố định (như mã số sinh viên), ta có thể dùng cách hiệu quả hơn như sắp xếp theo cơ số (radix sort), giảm độ phức tạp xuống O(nk), với k là số chữ số. Khi xử lý lượng dữ liệu lớn, thời gian tiết kiệm được sẽ mang lại giá trị lớn — giảm chi phí, nâng cao trải nghiệm người dùng và hiệu suất hệ thống.

Trong kỹ thuật, nhiều vấn đề rất khó giải quyết tối ưu; đa số được xử lý bằng giải pháp "gần tối ưu". Độ khó của một vấn đề không chỉ phụ thuộc vào bản chất của nó mà còn vào kiến thức và kinh nghiệm của người giải quyết. Càng hiểu sâu và có nhiều kinh nghiệm, chúng ta càng phân tích kỹ và giải quyết vấn đề một cách hiệu quả, sáng tạo hơn.

