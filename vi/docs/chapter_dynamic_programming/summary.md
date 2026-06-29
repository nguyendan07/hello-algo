# Tóm tắt

### Ôn tập trọng tâm

- Quy hoạch động phân rã các bài toán và tránh việc tính toán dư thừa bằng cách lưu trữ lời giải cho các bài toán con, từ đó cải thiện đáng kể hiệu suất tính toán.
- Nếu không xem xét giới hạn thời gian, tất cả các bài toán quy hoạch động đều có thể được giải bằng cách sử dụng quay lui (tìm kiếm vét cạn), nhưng cây đệ quy chứa một số lượng lớn các bài toán con chồng chéo, dẫn đến hiệu suất cực kỳ thấp. Bằng cách đưa vào một danh sách ghi nhớ, chúng ta có thể lưu trữ lời giải cho tất cả các bài toán con đã được tính toán, đảm bảo rằng các bài toán con chồng chéo chỉ được tính toán một lần.
- Ghi nhớ là một giải pháp đệ quy từ trên xuống, trong khi quy hoạch động tương ứng là một giải pháp lặp từ dưới lên, tương tự như việc "điền vào bảng". Vì trạng thái hiện tại chỉ phụ thuộc vào một số trạng thái cục bộ nhất định, chúng ta có thể loại bỏ một chiều của bảng $dp$ để giảm độ phức tạp không gian.
- Phân rã bài toán con là một phương pháp thuật toán tổng quát, với các tính chất khác nhau trong chia để trị, quy hoạch động và quay lui.
- Các bài toán quy hoạch động có ba đặc điểm lớn: các bài toán con chồng chéo, cấu trúc con tối ưu, và không có tác dụng phụ.
- Nếu lời giải tối ưu cho bài toán ban đầu có thể được xây dựng từ các lời giải tối ưu cho các bài toán con, thì nó có cấu trúc con tối ưu.
- Không có tác dụng phụ có nghĩa là đối với một trạng thái cho trước, sự phát triển trong tương lai của nó chỉ liên quan đến trạng thái đó và không liên quan gì đến tất cả các trạng thái trong quá khứ. Nhiều bài toán tối ưu hóa tổ hợp không thỏa mãn tính chất này và không thể giải quyết hiệu quả bằng quy hoạch động.

**Bài toán cái túi**

- Bài toán cái túi là một trong những bài toán quy hoạch động điển hình nhất, với các biến thể như cái túi 0-1, cái túi hoàn toàn, và đa cái túi.
- Định nghĩa trạng thái cho cái túi 0-1 là giá trị tối đa có thể đạt được bằng cách sử dụng $i$ đồ vật đầu tiên với sức chứa cái túi là $c$. Dựa trên hai quyết định là không cho đồ vật vào cái túi và cho nó vào, cấu trúc con tối ưu có thể được xác định và phương trình chuyển trạng thái được xây dựng. Trong tối ưu hóa không gian, vì mỗi trạng thái phụ thuộc vào trạng thái trực tiếp phía trên và phía trên bên trái, danh sách cần được duyệt theo thứ tự ngược để tránh ghi đè lên trạng thái phía trên bên trái.
- Bài toán cái túi hoàn toàn không có giới hạn về số lượng chọn của từng loại đồ vật, vì vậy sự chuyển trạng thái cho việc chọn cho đồ vật vào khác với bài toán cái túi 0-1. Vì trạng thái phụ thuộc vào trạng thái trực tiếp phía trên và trực tiếp bên trái, tối ưu hóa không gian nên sử dụng duyệt xuôi.
- Bài toán đổi tiền lẻ là một biến thể của bài toán cái túi hoàn toàn. Nó thay đổi từ việc tìm kiếm giá trị "lớn nhất" sang tìm kiếm số lượng đồng xu "nhỏ nhất", vì vậy toán tử $\max()$ trong phương trình chuyển trạng thái nên được đổi thành $\min()$. Nó thay đổi từ việc tìm kiếm "không vượt quá" sức chứa cái túi sang tìm kiếm việc tạo thành "chính xác" số tiền mục tiêu, vì vậy $amt + 1$ được sử dụng để đại diện cho lời giải không hợp lệ "không thể tạo thành số tiền mục tiêu".
- Bài toán đổi tiền lẻ II thay đổi từ việc tìm kiếm "số lượng đồng xu tối thiểu" sang tìm kiếm "số lượng tổ hợp đồng xu", vì vậy phương trình chuyển trạng thái tương ứng thay đổi từ $\min()$ sang một toán tử lấy tổng.

**Bài toán khoảng cách chỉnh sửa**

- Khoảng cách chỉnh sửa (khoảng cách Levenshtein) được sử dụng để đo lường độ tương đồng giữa hai chuỗi, được định nghĩa là số bước chỉnh sửa tối thiểu từ một chuỗi này sang một chuỗi khác, với các thao tác chỉnh sửa bao gồm chèn, xóa và thay thế.
- Định nghĩa trạng thái cho bài toán khoảng cách chỉnh sửa là số bước chỉnh sửa tối thiểu cần thiết để thay đổi $i$ ký tự đầu tiên của $s$ thành $j$ ký tự đầu tiên của $t$. Khi $s[i] \ne t[j]$, có ba quyết định: chèn, xóa, thay thế, mỗi quyết định có các bài toán con còn lại tương ứng. Từ đây, cấu trúc con tối ưu có thể được xác định và phương trình chuyển trạng thái được xây dựng. Khi $s[i] = t[j]$, không cần chỉnh sửa cho ký tự hiện tại.
- Trong khoảng cách chỉnh sửa, trạng thái phụ thuộc vào trạng thái trực tiếp phía trên, trực tiếp bên trái, và phía trên bên trái, vì vậy sau khi tối ưu hóa không gian, cả duyệt xuôi lẫn duyệt ngược đều không thể thực hiện chính xác việc chuyển trạng thái. Vì lý do này, chúng ta sử dụng một biến để tạm thời lưu trữ trạng thái phía trên bên trái, từ đó chuyển đổi sang một tình huống tương đương với bài toán cái túi hoàn toàn, cho phép duyệt xuôi sau khi tối ưu hóa không gian.
