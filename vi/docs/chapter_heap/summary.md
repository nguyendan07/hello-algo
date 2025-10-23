---
comments: true
---

# 8.4 &nbsp; Tóm tắt

### 1. &nbsp; Tổng quan chính

- Một heap là một complete binary tree có thể được phân loại thành max heap
  hoặc min heap dựa trên tính chất xây dựng của nó, trong đó phần tử
  trên cùng của một max heap là lớn nhất và phần tử trên cùng của một
  min heap là nhỏ nhất.
- Một priority queue được định nghĩa là một queue có ưu tiên khi lấy ra,
  thường được triển khai bằng cách sử dụng một heap.
- Các thao tác phổ biến của một heap và time complexity tương ứng của
  chúng bao gồm: chèn phần tử vào heap $O(\log n)$, loại bỏ phần tử trên
  cùng khỏi heap $O(\log n)$, và truy cập phần tử trên cùng của heap $O(1)$.
- Một complete binary tree rất phù hợp để được biểu diễn bằng một array,
  do đó các heap thường được lưu trữ bằng cách sử dụng các array.
- Các thao tác Heapify được sử dụng để duy trì các tính chất của heap
  và được dùng trong cả thao tác chèn và loại bỏ của heap.
- time complexity của việc xây dựng một heap với đầu vào gồm $n$ phần tử
  có thể được tối ưu hóa thành $O(n)$, đây là một độ phức tạp rất hiệu quả.
- Top-k là một bài toán thuật toán kinh điển có thể được giải quyết hiệu quả
  bằng cách sử dụng cấu trúc dữ liệu heap, với time complexity là $O(n \log k)$.

### 2. &nbsp; Hỏi & Đáp

**Hỏi**: "heap" trong cấu trúc dữ liệu có phải là cùng một khái niệm với
  "heap" trong quản lý bộ nhớ không?

Hai khái niệm này không giống nhau, mặc dù cả hai đều được gọi là "heap".
Heap trong bộ nhớ hệ thống máy tính là một phần của cấp phát bộ nhớ động,
nơi chương trình có thể sử dụng nó để lưu trữ dữ liệu trong quá trình
thực thi. Chương trình có thể yêu cầu một lượng bộ nhớ heap nhất định
để lưu trữ các cấu trúc phức tạp như đối tượng và array. Khi dữ liệu đã
cấp phát không còn cần thiết, chương trình cần giải phóng bộ nhớ này
để ngăn chặn rò rỉ bộ nhớ. So với bộ nhớ stack, việc quản lý và sử dụng
bộ nhớ heap đòi hỏi nhiều sự cẩn trọng hơn, vì việc sử dụng không đúng
cách có thể dẫn đến rò rỉ bộ nhớ và con trỏ treo.
