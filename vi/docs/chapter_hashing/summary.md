---
comments: true
---

# 6.4 &nbsp; Tóm tắt

### 1 &nbsp; Ôn tập kiến thức

- Với một `key` đầu vào, bảng băm có thể truy xuất `value` tương ứng trong thời gian $O(1)$, rất hiệu quả.
- Các thao tác phổ biến trên bảng băm bao gồm truy vấn, thêm cặp key-value, xóa cặp key-value và duyệt bảng băm.
- Hàm băm ánh xạ một `key` thành một chỉ mục mảng, cho phép truy cập vào bucket tương ứng và truy xuất `value`.
- Hai key khác nhau có thể có cùng chỉ mục mảng sau khi băm, dẫn đến kết quả truy vấn sai lệch. Hiện tượng này được gọi là xung đột băm.
- Dung lượng của bảng băm càng lớn thì xác suất xung đột băm càng thấp. Do đó, thay đổi kích thước bảng băm có thể giảm thiểu xung đột băm. Tương tự như thay đổi kích thước mảng, thay đổi kích thước bảng băm tốn kém.
- Hệ số tải, được định nghĩa là số lượng phần tử chia cho số lượng bucket, phản ánh mức độ nghiêm trọng của xung đột băm và thường được sử dụng làm điều kiện để kích hoạt thay đổi kích thước bảng băm.
- Chaining giải quyết xung đột băm bằng cách chuyển đổi mỗi phần tử thành một danh sách liên kết, lưu trữ tất cả các phần tử xung đột trong cùng một danh sách. Tuy nhiên, danh sách quá dài có thể làm giảm hiệu quả truy vấn, có thể được cải thiện bằng cách chuyển đổi danh sách thành cây đỏ-đen.
- Open addressing xử lý xung đột băm thông qua nhiều lần thăm dò. Linear probing sử dụng kích thước bước cố định nhưng không thể xóa các phần tử và dễ bị clustering. Multiple hashing sử dụng một số hàm băm để thăm dò, giúp giảm clustering so với linear probing nhưng làm tăng chi phí tính toán.
- Các ngôn ngữ lập trình khác nhau áp dụng các cách triển khai bảng băm khác nhau. Ví dụ: `HashMap` của Java sử dụng chaining, trong khi `dict` của Python sử dụng open addressing.
- Trong bảng băm, chúng ta mong muốn các thuật toán băm có tính xác định, hiệu quả cao và phân phối đồng đều. Trong mật mã, các thuật toán băm cũng phải có khả năng chống xung đột và hiệu ứng thác lũ.
- Các thuật toán băm thường sử dụng các số nguyên tố lớn làm moduli để đảm bảo phân phối đồng đều các giá trị băm và giảm xung đột băm.
- Các thuật toán băm phổ biến bao gồm MD5, SHA-1, SHA-2 và SHA-3. MD5 thường được sử dụng để kiểm tra tính toàn vẹn của tệp, trong khi SHA-2 thường được sử dụng trong các ứng dụng và giao thức an toàn.
- Các ngôn ngữ lập trình thường cung cấp các thuật toán băm tích hợp cho các kiểu dữ liệu để tính toán chỉ mục bucket trong bảng băm. Nói chung, chỉ các đối tượng bất biến mới có thể băm được.

### 2 &nbsp; Hỏi & Đáp

**H**: Khi nào độ phức tạp thời gian của bảng băm giảm xuống $O(n)$?

Độ phức tạp thời gian của bảng băm có thể giảm xuống $O(n)$ khi xung đột băm nghiêm trọng. Khi hàm băm được thiết kế tốt, dung lượng được đặt phù hợp và các xung đột được phân phối đều, độ phức tạp thời gian là $O(1)$. Chúng ta thường coi độ phức tạp thời gian là $O(1)$ khi sử dụng bảng băm tích hợp trong các ngôn ngữ lập trình.

**H**: Tại sao không sử dụng hàm băm $f(x) = x$? Điều này sẽ loại bỏ các xung đột.

Với hàm băm $f(x) = x$, mỗi phần tử tương ứng với một chỉ mục bucket duy nhất, tương đương với một mảng. Tuy nhiên, không gian đầu vào thường lớn hơn nhiều so với không gian đầu ra (chiều dài mảng), vì vậy bước cuối cùng của hàm băm thường là lấy modulo của chiều dài mảng. Nói cách khác, mục tiêu của bảng băm là ánh xạ một không gian trạng thái lớn hơn sang một không gian nhỏ hơn trong khi cung cấp hiệu quả truy vấn $O(1)$.

**H**: Tại sao bảng băm có thể hiệu quả hơn mảng, danh sách liên kết hoặc cây nhị phân, mặc dù bảng băm được triển khai bằng các cấu trúc này?

Thứ nhất, bảng băm có hiệu quả thời gian cao hơn nhưng hiệu quả không gian thấp hơn. Một phần đáng kể bộ nhớ trong bảng băm vẫn không được sử dụng.

Thứ hai, bảng băm chỉ hiệu quả hơn về thời gian trong các trường hợp sử dụng cụ thể. Nếu một tính năng có thể được triển khai với cùng độ phức tạp thời gian bằng cách sử dụng một mảng hoặc một danh sách liên kết, thì thường nhanh hơn so với sử dụng một bảng băm. Điều này là do việc tính toán hàm băm phát sinh chi phí, làm cho hệ số không đổi trong độ phức tạp thời gian lớn hơn.

Cuối cùng, độ phức tạp thời gian của bảng băm có thể giảm xuống. Ví dụ: trong chaining, chúng ta thực hiện các thao tác tìm kiếm trong một danh sách liên kết hoặc cây đỏ-đen, điều này vẫn có nguy cơ giảm xuống thời gian $O(n)$.

**H**: Multiple hashing có nhược điểm là không thể xóa trực tiếp các phần tử không? Không gian được đánh dấu là đã xóa có thể được sử dụng lại không?

Multiple hashing là một dạng của open addressing và tất cả các phương pháp open addressing đều có nhược điểm là không thể xóa trực tiếp các phần tử; chúng yêu cầu đánh dấu các phần tử là đã xóa. Các không gian được đánh dấu có thể được sử dụng lại. Khi chèn các phần tử mới vào bảng băm và hàm băm trỏ đến một vị trí được đánh dấu là đã xóa, vị trí đó có thể được sử dụng bởi phần tử mới. Điều này duy trì chuỗi thăm dò của bảng băm đồng thời đảm bảo sử dụng không gian hiệu quả.

**H**: Tại sao xung đột băm xảy ra trong quá trình tìm kiếm trong linear probing?

Trong quá trình tìm kiếm, hàm băm trỏ đến bucket và cặp key-value tương ứng. Nếu `key` không khớp, điều đó cho thấy có xung đột băm. Do đó, linear probing sẽ tìm kiếm xuống dưới với một kích thước bước được xác định trước cho đến khi tìm thấy cặp key-value chính xác hoặc tìm kiếm không thành công.

**H**: Tại sao thay đổi kích thước bảng băm có thể giảm bớt xung đột băm?

Bước cuối cùng của hàm băm thường liên quan đến việc lấy modulo của chiều dài mảng $n$, để giữ cho đầu ra nằm trong phạm vi chỉ mục mảng. Khi thay đổi kích thước, chiều dài mảng $n$ thay đổi và các chỉ mục tương ứng với các key cũng có thể thay đổi. Các key trước đây được ánh xạ tới cùng một bucket có thể được phân phối trên nhiều bucket sau khi thay đổi kích thước, do đó giảm thiểu xung đột băm.
