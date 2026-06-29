# Cùng nhau đóng góp

Do năng lực và thời gian có hạn, cuốn sách này khó tránh khỏi những thiếu sót và sai sót. Chúng tôi rất mong nhận được sự thông cảm và vô cùng biết ơn sự giúp đỡ của bạn trong việc chỉnh sửa. Nếu bạn phát hiện ra lỗi chính tả, liên kết hỏng, nội dung còn thiếu, cách diễn đạt mơ hồ, giải thích chưa rõ ràng hoặc các vấn đề về cấu trúc, xin hãy giúp chúng tôi điều chỉnh để mang lại nguồn tài liệu học tập chất lượng cao hơn cho độc giả.

ID GitHub của tất cả những người [đóng góp (contributors)](https://github.com/krahets/hello-algo/graphs/contributors) sẽ được hiển thị trên trang chủ của kho lưu trữ sách, phiên bản web và phiên bản PDF nhằm ghi nhận những đóng góp vô tư của họ cho cộng đồng mã nguồn mở.

!!! success "Sức hút của Mã nguồn mở"

    Khoảng thời gian giữa hai lần in của một cuốn sách giấy thường khá dài, khiến việc cập nhật nội dung trở nên rất bất tiện.

    Trong cuốn sách mã nguồn mở này, thời gian cập nhật nội dung đã được rút ngắn chỉ còn vài ngày, thậm chí vài giờ.

### Điều chỉnh nội dung nhỏ

Như được hiển thị trong hình bên dưới, có một "biểu tượng chỉnh sửa" ở góc trên bên phải của mỗi trang. Bạn có thể sửa đổi văn bản hoặc mã nguồn theo các bước sau.

1. Nhấp vào "biểu tượng chỉnh sửa". Nếu bạn gặp thông báo yêu cầu "Fork this repository", vui lòng chấp nhận thao tác này.
2. Sửa đổi nội dung của tập tin nguồn Markdown, xác minh tính đúng đắn của nội dung và giữ nguyên định dạng nhất quán nhất có thể.
3. Điền mô tả về các thay đổi của bạn ở cuối trang, sau đó nhấp vào nút "Propose file change". Sau khi trang mới tải xong, nhấp vào nút "Create pull request" để gửi yêu cầu kéo (pull request).

![Nút chỉnh sửa trang](contribution.assets/edit_markdown.png)

Hình ảnh không thể sửa đổi trực tiếp. Vui lòng mô tả vấn đề bằng cách tạo một [Issue](https://github.com/krahets/hello-algo/issues) mới hoặc để lại bình luận. Chúng tôi sẽ nhanh chóng vẽ lại và thay thế hình ảnh.

### Sáng tạo nội dung

Nếu bạn quan tâm đến việc đóng góp cho dự án mã nguồn mở này, bao gồm dịch mã nguồn sang các ngôn ngữ lập trình khác hoặc mở rộng nội dung bài viết, bạn sẽ cần tuân theo quy trình Pull Request bên dưới.

1. Đăng nhập vào GitHub và Fork [kho lưu trữ mã nguồn](https://github.com/krahets/hello-algo) của sách về tài khoản cá nhân của bạn.
2. Truy cập trang kho lưu trữ đã fork của bạn và sử dụng lệnh `git clone` để sao chép kho lưu trữ về máy cục bộ.
3. Tạo nội dung cục bộ và tiến hành kiểm thử toàn diện để xác minh tính đúng đắn của mã nguồn.
4. Cam kết (commit) các thay đổi cục bộ của bạn và đẩy (push) chúng lên kho lưu trữ từ xa.
5. Làm mới trang web kho lưu trữ và nhấp vào nút "Create pull request" để gửi yêu cầu kéo.

### Triển khai Docker

Từ thư mục gốc của `hello-algo`, chạy lệnh Docker sau để truy cập dự án tại địa chỉ `http://localhost:8000`:

```shell
docker-compose up -d
```

Sử dụng lệnh sau để gỡ bỏ triển khai:

```shell
docker-compose down
```
