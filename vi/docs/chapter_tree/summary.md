# Tóm tắt

### Ôn tập chính

- Cây nhị phân là một cấu trúc dữ liệu phi tuyến thể hiện tư duy "chia để trị" khi tách một phần tử thành hai. Mỗi nút của cây nhị phân chứa một giá trị và hai con trỏ, trỏ đến nút con bên trái và bên phải tương ứng.
- Đối với một nút trong cây nhị phân, nút con bên trái (bên phải) và cây được tạo ra dưới nó được gọi chung là cây con bên trái (bên phải) của nút đó.
- Các thuật ngữ liên quan đến cây nhị phân bao gồm nút gốc, nút lá, mức (level), bậc (degree), cạnh, chiều cao và độ sâu.
- Các thao tác khởi tạo cây nhị phân, chèn nút và xóa nút tương tự như các thao tác trên danh sách liên kết.
- Các loại cây nhị phân phổ biến bao gồm cây nhị phân hoàn hảo, cây nhị phân đầy đủ (complete), cây nhị phân đầy (full) và cây nhị phân cân bằng. Cây nhị phân hoàn hảo là trạng thái lý tưởng, trong khi danh sách liên kết là trạng thái tệ nhất khi suy thoái.
- Cây nhị phân có thể được biểu diễn bằng mảng bằng cách sắp xếp các giá trị nút và các ô trống theo thứ tự duyệt theo tầng (level-order) và triển khai các con trỏ dựa trên quan hệ ánh xạ chỉ số giữa nút cha và nút con.
- Duyệt theo tầng (level-order) của cây nhị phân là phương pháp tìm kiếm theo chiều rộng (breadth-first), thể hiện cách duyệt theo lớp "mở rộng vòng này sang vòng khác". Thường được triển khai bằng hàng đợi (queue).
- Duyệt trước (pre-order), duyệt giữa (in-order) và duyệt sau (post-order) đều là các phương pháp tìm kiếm theo chiều sâu (depth-first), thể hiện cách duyệt "đi tới tận cùng trước, rồi quay lui để tiếp tục". Thường được triển khai bằng đệ quy.
- Cây tìm kiếm nhị phân (binary search tree) là cấu trúc dữ liệu hiệu quả cho việc tìm kiếm phần tử, với độ phức tạp thời gian cho các thao tác tìm kiếm, chèn và xóa đều là $O(\log n)$. Khi cây tìm kiếm nhị phân suy thoái thành danh sách liên kết, các độ phức tạp này giảm xuống $O(n)$.
- Cây AVL, còn gọi là cây tìm kiếm nhị phân cân bằng, đảm bảo cây luôn cân bằng sau nhiều lần chèn và xóa nút bằng các phép quay (rotation).
- Các phép quay trong cây AVL bao gồm phép quay phải, phép quay trái, quay phải-trái và quay trái-phải. Sau khi chèn hoặc xóa nút, cây AVL cân bằng lại bằng cách thực hiện các phép quay này theo chiều từ dưới lên.

### Hỏi & Đáp

**Q**: Với một cây nhị phân chỉ có một nút, cả chiều cao của cây và độ sâu của nút gốc có phải là $0$ không?

Có, vì chiều cao và độ sâu thường được định nghĩa là "số cạnh đã đi qua."

**Q**: Việc chèn và xóa trong cây nhị phân thường thực hiện bằng một tập hợp các thao tác. "Một tập hợp các thao tác" ở đây đề cập đến gì? Có hàm ý giải phóng tài nguyên của các nút con không?

Lấy cây tìm kiếm nhị phân làm ví dụ, thao tác xóa một nút cần xử lý ở ba trường hợp khác nhau, mỗi trường hợp cần thực hiện nhiều bước thao tác trên các nút.

**Q**: Tại sao có ba thứ tự: pre-order, in-order và post-order cho việc duyệt DFS của cây nhị phân, và mục đích của chúng là gì?

Tương tự như duyệt tuần tự và duyệt ngược của mảng, pre-order, in-order và post-order là ba phương pháp duyệt cây nhị phân, cho phép ta thu được kết quả duyệt theo một thứ tự nhất định. Ví dụ, trong cây tìm kiếm nhị phân, vì kích thước các nút thỏa mãn `left child node value < root node value < right child node value`, ta có thể thu được một dãy các nút có thứ tự bằng cách duyệt cây theo ưu tiên "trái $\rightarrow$ gốc $\rightarrow$ phải".

**Q**: Trong phép quay phải xử lý mối quan hệ giữa các nút mất cân bằng `node`, `child`, `grand_child`, phải chăng sau khi quay phải sẽ làm mất mối liên kết giữa `node` và nút cha của nó cũng như liên kết gốc của `node`?

Cần nhìn vấn đề này dưới góc độ đệ quy. Thao tác `right_rotate(root)` nhận nút gốc của một cây con và cuối cùng trả về nút gốc của cây con sau khi quay bằng `return child`. Việc liên kết giữa nút gốc của cây con đó và nút cha của nó được thiết lập sau khi hàm này trả về, điều đó nằm ngoài phạm vi bảo trì của phép quay phải.

**Q**: Trong C++, các hàm được chia thành phần `private` và `public`. Cần cân nhắc gì cho việc này? Tại sao hàm `height()` và hàm `updateHeight()` lại được đặt lần lượt ở `public` và `private`?

Điều đó phụ thuộc vào phạm vi sử dụng của phương thức. Nếu một phương thức chỉ dùng trong lớp thì nên đặt là `private`. Ví dụ, người dùng không nên tự gọi `updateHeight()`, vì nó chỉ là một bước trong thao tác chèn hoặc xóa. Ngược lại, `height()` dùng để truy xuất chiều cao của nút, tương tự `vector.size()`, nên được đặt `public` để sử dụng.

**Q**: Làm thế nào để xây dựng một cây tìm kiếm nhị phân từ một tập dữ liệu đầu vào? Việc chọn nút gốc có quan trọng không?

Có, phương pháp xây dựng cây được cung cấp trong hàm `build_tree()` trong mã của cây tìm kiếm nhị phân. Về việc chọn nút gốc, ta thường sắp xếp dữ liệu đầu vào rồi chọn phần tử ở giữa làm gốc, sau đó đệ quy xây dựng các cây con trái và phải. Cách này giúp cây cân bằng tối đa.

**Q**: Trong Java, có phải luôn luôn phải dùng phương thức `equals()` để so sánh chuỗi không?

Trong Java, với kiểu dữ liệu nguyên thủy (primitive), dùng `==` để so sánh xem giá trị của hai biến có bằng nhau không. Với kiểu tham chiếu, nguyên lý hoạt động của hai ký hiệu này khác nhau.

- `==`: Dùng để so sánh hai biến có cùng trỏ tới một đối tượng hay không, tức là vị trí trong bộ nhớ có giống nhau hay không.
- `equals()`: Dùng để so sánh giá trị của hai đối tượng có bằng nhau hay không.

Do đó, để so sánh giá trị, ta nên dùng `equals()`. Tuy nhiên, các chuỗi khởi tạo như `String a = "hi"; String b = "hi";` được lưu trong pool hằng chuỗi và trỏ tới cùng một đối tượng, nên `a == b` cũng có thể dùng để so sánh nội dung trong trường hợp đó.

**Q**: Trước khi tới tầng đáy, số nút trong hàng đợi có phải là $2^h$ trong duyệt theo chiều rộng không?

Có, ví dụ một cây nhị phân đầy đủ với chiều cao $h = 2$ có tổng số nút $n = 7$, thì tầng đáy có $4 = 2^h = (n + 1) / 2$ nút.
