# 10.6 &nbsp; Tóm tắt

- Binary search phụ thuộc vào thứ tự của dữ liệu và thực hiện tìm kiếm bằng
  cách lặp đi lặp lại chia đôi khoảng tìm kiếm. Nó yêu cầu dữ liệu đầu vào
  phải được sắp xếp và chỉ áp dụng được cho arrays hoặc các cấu trúc dữ liệu
  dựa trên array.
- Brute force search có thể cần thiết để định vị một phần tử trong một tập
  dữ liệu chưa được sắp xếp. Các thuật toán tìm kiếm khác nhau có thể được
  áp dụng tùy thuộc vào cấu trúc dữ liệu: Linear search phù hợp cho arrays
  và linked lists, trong khi breadth-first search (BFS) và depth-first search
  (DFS) phù hợp cho graphs và trees. Các thuật toán này rất linh hoạt,
  không yêu cầu tiền xử lý dữ liệu, nhưng chúng có time complexity cao hơn
  là $O(n)$.
- Hash search, tree search và binary search là các phương pháp tìm kiếm hiệu
  quả có thể nhanh chóng định vị các phần tử mục tiêu trong các cấu trúc dữ
  liệu cụ thể. Các thuật toán này rất hiệu quả, với time complexity đạt
  $O(\log n)$ hoặc thậm chí $O(1)$, nhưng chúng thường yêu cầu không gian
  bổ sung để chứa các cấu trúc dữ liệu bổ sung.
- Trong thực tế, chúng ta cần phân tích các yếu tố như khối lượng dữ liệu,
  yêu cầu hiệu suất tìm kiếm, tần suất truy vấn và cập nhật dữ liệu, v.v.,
  để lựa chọn một phương pháp tìm kiếm phù hợp.
- Linear search lý tưởng cho dữ liệu nhỏ hoặc được cập nhật thường xuyên
  (thay đổi liên tục). Binary search hoạt động tốt cho dữ liệu lớn và đã
  được sắp xếp. Hash search phù hợp cho dữ liệu yêu cầu hiệu quả truy vấn
  cao và không cần truy vấn theo phạm vi. Tree search phù hợp nhất cho dữ
  liệu động lớn yêu cầu duy trì thứ tự và cần hỗ trợ truy vấn theo phạm vi.
- Thay thế linear search bằng hash search là một chiến lược phổ biến để tối
  ưu hóa hiệu suất runtime, giảm time complexity từ $O(n)$ xuống $O(1)$.
