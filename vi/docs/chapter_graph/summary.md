# 9.4 &nbsp; Tóm tắt

### 1. &nbsp; Tổng quan chính

- Một graph được tạo thành từ các đỉnh và các cạnh. Nó có thể được mô tả
    như một tập hợp các đỉnh và một tập hợp các cạnh.
- So với các mối quan hệ tuyến tính (như linked lists) và mối quan hệ
    phân cấp (như trees), mối quan hệ mạng (graphs) mang lại sự linh hoạt
    lớn hơn, khiến chúng phức tạp hơn.
- Trong một directed graph, các cạnh có hướng. Trong một connected graph,
    bất kỳ đỉnh nào cũng có thể được truy cập từ bất kỳ đỉnh nào khác.
    Trong một weighted graph, mỗi cạnh có một biến trọng số liên kết.
- Một adjacency matrix là một cách để biểu diễn một graph bằng cách sử dụng
    matrix (array 2D). Các hàng và cột đại diện cho các đỉnh. Giá trị
    phần tử matrix cho biết liệu có một cạnh giữa hai đỉnh hay không,
    sử dụng $1$ cho cạnh hoặc $0$ cho không có cạnh. Adjacency matrix
    rất hiệu quả cho các thao tác như thêm, xóa hoặc kiểm tra cạnh,
    nhưng chúng đòi hỏi nhiều không gian hơn.
- Một adjacency list là một cách phổ biến khác để biểu diễn một graph
    bằng cách sử dụng một tập hợp các linked lists. Mỗi đỉnh trong graph
    có một list chứa tất cả các đỉnh kề của nó. List thứ $i$ đại diện cho
    đỉnh thứ $i$. Adjacency lists sử dụng ít không gian hơn so với
    adjacency matrices. Tuy nhiên, vì nó đòi hỏi duyệt list để tìm cạnh,
    hiệu quả thời gian thấp hơn.
- Khi các linked lists trong một adjacency list đủ dài, chúng có thể
    được chuyển đổi thành red-black trees hoặc hash tables để cải thiện
    hiệu quả tìm kiếm.
- Từ góc độ thiết kế thuật toán, một adjacency matrix phản ánh khái niệm
    "đánh đổi không gian lấy thời gian", trong khi một adjacency list phản
    ánh "đánh đổi thời gian lấy không gian".
- Graphs có thể được sử dụng để mô hình hóa nhiều hệ thống thực tế,
    chẳng hạn như social networks, subway routes.
- Một tree là một trường hợp đặc biệt của một graph, và tree traversal
    cũng là một trường hợp đặc biệt của graph traversal.
- Breadth-first traversal của một graph là một phương pháp tìm kiếm
    mở rộng từng lớp từ gần đến xa, thường sử dụng một queue.
- Depth-first traversal của một graph là một phương pháp tìm kiếm
    ưu tiên đi đến cuối trước khi backtracking khi không có đường dẫn
    nào tiếp theo. Nó thường được triển khai bằng cách sử dụng recursion.

### 2. &nbsp; Hỏi & Đáp

**Q**: Một path được định nghĩa là một chuỗi các đỉnh hay một chuỗi
các cạnh?

Trong graph theory, một path trong một graph là một chuỗi hữu hạn hoặc
vô hạn các cạnh nối một chuỗi các đỉnh.

Trong tài liệu này, một path được coi là một chuỗi các cạnh, hơn là
một chuỗi các đỉnh. Điều này là do có thể có nhiều cạnh nối hai đỉnh,
trong trường hợp đó mỗi cạnh tương ứng với một path.

**Q**: Trong một disconnected graph, có những đỉnh không thể được duyệt
không?

Trong một disconnected graph, có ít nhất một đỉnh không thể được truy cập
từ một điểm cụ thể. Để duyệt một disconnected graph, bạn cần đặt nhiều
điểm bắt đầu để duyệt tất cả các connected components của graph.

**Q**: Trong một adjacency list, thứ tự của "tất cả các đỉnh nối với
đỉnh đó" có quan trọng không?

Các đỉnh có thể được lưu trữ theo bất kỳ thứ tự nào. Tuy nhiên, trong các ứng dụng
thực tế, có thể cần sắp xếp chúng theo những quy tắc nhất định, chẳng hạn
như thứ tự các đỉnh được thêm vào hoặc thứ tự giá trị của các đỉnh. Điều này
có thể giúp chúng ta tìm kiếm các đỉnh một cách nhanh chóng, đặc biệt là
các đỉnh có giá trị cực trị nhất định.
