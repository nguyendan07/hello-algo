# Tóm tắt

### Ôn tập chính

- Cấu trúc dữ liệu có hai loại: cấu trúc logic (cách dữ liệu liên kết với nhau) và cấu trúc vật lý (cách dữ liệu lưu trong bộ nhớ).
- Cấu trúc logic gồm tuyến tính (mảng, danh sách liên kết, ngăn xếp, hàng đợi) và phi tuyến tính (cây, đồ thị, heap). Bảng băm có thể dùng cả hai loại này.
- Khi chương trình chạy, dữ liệu lưu trong bộ nhớ, mỗi vùng nhớ có địa chỉ riêng để truy cập.
- Cấu trúc vật lý gồm lưu trữ liên tục (mảng) và lưu trữ rời rạc (danh sách liên kết). Các cấu trúc dữ liệu đều xây dựng từ mảng, danh sách liên kết hoặc kết hợp cả hai.
- Kiểu dữ liệu cơ bản: số nguyên (`byte`, `short`, `int`, `long`), số thực (`float`, `double`), ký tự (`char`), boolean (`bool`). Giá trị của kiểu dữ liệu phụ thuộc vào kích thước và cách biểu diễn.
- Có ba cách mã hóa số nguyên: dấu-chân, bù 1, bù 2. Bit đầu là bit dấu, các bit còn lại là giá trị số.
- Máy tính dùng bù 2 để mã hóa số nguyên. Ưu điểm: (i) cộng số dương và âm dễ dàng, (ii) không cần mạch phần cứng đặc biệt cho phép trừ, (iii) không có hai số 0 khác nhau.
- Số thực được mã hóa bằng 1 bit dấu, 8 bit số mũ, 23 bit phần thập phân. Nhờ số mũ, phạm vi số thực lớn hơn số nguyên, nhưng độ chính xác giảm.
- ASCII là bộ ký tự tiếng Anh, dài 1 byte, có 127 ký tự. GBK là bộ mã tiếng Trung, có hơn 20.000 ký tự. Unicode là chuẩn chung cho các ngôn ngữ, giúp tránh lỗi ký tự do mã hóa khác nhau.
- UTF-8 là cách mã hóa Unicode phổ biến nhất, có độ dài thay đổi, tiết kiệm không gian. UTF-16 và UTF-32 có độ dài cố định. Khi lưu ký tự Trung Quốc, UTF-16 tiết kiệm hơn UTF-8. Java và C# mặc định dùng UTF-16.

### Hỏi & Đáp

**Hỏi**: Tại sao bảng băm dùng cả cấu trúc tuyến tính và phi tuyến tính?

Bảng băm dùng mảng làm nền tảng. Để xử lý va chạm, mỗi ô mảng có thể trỏ đến danh sách liên kết hoặc cây. Vì vậy, bảng băm vừa dùng cấu trúc tuyến tính (mảng, danh sách liên kết) vừa dùng phi tuyến tính (cây).

**Hỏi**: Kiểu `char` có luôn dài 1 byte không?

Độ dài kiểu `char` phụ thuộc vào ngôn ngữ lập trình. Java, JavaScript, TypeScript, C# dùng UTF-16, nên `char` dài 2 byte.

**Hỏi**: Cấu trúc dữ liệu dựa trên mảng gọi là "tĩnh" có đúng không? Ngăn xếp vẫn thêm/xóa động mà.

Ngăn xếp thao tác động với dữ liệu, nhưng dung lượng mảng là cố định. Nếu vượt quá dung lượng, mảng sẽ được sao chép sang mảng mới lớn hơn.

**Hỏi**: Khi xây dựng ngăn xếp/hàng đợi, không chỉ định kích thước, sao vẫn gọi là "tĩnh"?

Trong ngôn ngữ cấp cao, dung lượng ban đầu được tự động cấp phát (ví dụ, `ArrayList` trong Java thường là 10). Khi cần, dung lượng sẽ tự động mở rộng.

**Hỏi**: Chuyển dấu-chân sang bù 2 là "đổi dấu rồi cộng 1", vậy chuyển ngược lại là "trừ 1 rồi đổi dấu". Nhưng bù 2 cũng có thể chuyển sang dấu-chân bằng "đổi dấu rồi cộng 1", tại sao?

**Đáp**: Chuyển đổi giữa dấu-chân và bù 2 thực chất là tính "bù". Nếu $a + b = c$, thì $a$ là bù của $b$ tới $c$.

Ví dụ, số nhị phân $0010$ (4 bit), chuyển dấu-chân sang bù 2 bằng "đổi dấu rồi cộng 1":

$$
0010 \rightarrow 1101 \rightarrow 1110
$$

Tổng $0010 + 1110 = 10000$, nghĩa là $1110$ là bù của $0010$ tới $10000$.

Chuyển ngược lại cũng dùng "đổi dấu rồi cộng 1":

$$
1110 \rightarrow 0001 \rightarrow 0010
$$

Vì vậy, cả hai chiều đều dùng được "đổi dấu rồi cộng 1". Ngoài ra, có thể dùng "trừ 1 rồi đổi dấu":

$$
1110 \rightarrow 1101 \rightarrow 0010
$$

Tóm lại, "đổi dấu rồi cộng 1" và "trừ 1 rồi đổi dấu" đều là tính bù tới $10000$, và đều đúng.

Thực chất, "đổi dấu" là tìm bù tới $1111$ (vì dấu-chân + bù 1 = $1111$), còn bù 1 cộng 1 thì ra bù 2 tới $10000$.

Ví dụ dùng 4 bit, nhưng áp dụng cho mọi số nhị phân.
