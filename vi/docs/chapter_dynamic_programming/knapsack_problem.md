# Bài toán Cái túi 0-1 (0-1 Knapsack)

Bài toán cái túi là một bài toán nhập môn tuyệt vời cho quy hoạch động và là một trong những dạng bài toán phổ biến nhất trong quy hoạch động. Nó có nhiều biến thể, chẳng hạn như bài toán cái túi 0-1, bài toán cái túi hoàn toàn (unbounded knapsack), và bài toán đa cái túi (multiple knapsack).

Trong phần này, trước tiên chúng ta sẽ giải quyết bài toán cái túi 0-1 phổ biến nhất.

!!! question

    Cho $n$ đồ vật và một cái túi có sức chứa $cap$, trong đó trọng lượng và giá trị của đồ vật thứ $i$ lần lượt là $wgt[i-1]$ và $val[i-1]$. Mỗi đồ vật chỉ có thể được chọn tối đa một lần. Giá trị tối đa có thể cho vào cái túi dưới giới hạn sức chứa là bao nhiêu?

Quan sát hình bên dưới. Vì số thứ tự đồ vật $i$ bắt đầu đếm từ $1$ và các chỉ số mảng bắt đầu từ $0$, đồ vật $i$ tương ứng với trọng lượng $wgt[i-1]$ và giá trị $val[i-1]$.

![Dữ liệu ví dụ cho cái túi 0-1](knapsack_problem.assets/knapsack_example.png)

Chúng ta có thể xem bài toán cái túi 0-1 như một quá trình bao gồm $n$ vòng quyết định, trong đó đối với mỗi đồ vật có hai quyết định: không cho vào và cho vào, do đó bài toán thỏa mãn mô hình cây quyết định.

Mục tiêu của bài toán này là tìm "giá trị tối đa có thể cho vào cái túi trong giới hạn sức chứa", vì vậy nó nhiều khả năng là một bài toán quy hoạch động.

**Bước 1: Suy nghĩ về các quyết định trong mỗi vòng, định nghĩa trạng thái, và từ đó thu được bảng $dp$**

Đối với mỗi đồ vật, nếu không cho vào cái túi, sức chứa cái túi giữ nguyên; nếu cho vào, sức chứa cái túi giảm đi. Từ đây, chúng ta có thể rút ra định nghĩa trạng thái: số thứ tự đồ vật hiện tại $i$ và sức chứa cái túi $c$, ký hiệu là $[i, c]$.

Trạng thái $[i, c]$ tương ứng với bài toán con: **giá trị tối đa trong số $i$ đồ vật đầu tiên trong một cái túi có sức chứa $c$**, ký hiệu là $dp[i, c]$.

Những gì chúng ta cần tìm là $dp[n, cap]$, vì vậy chúng ta cần một bảng $dp$ hai chiều kích thước $(n+1) \times (cap+1)$.

**Bước 2: Xác định cấu trúc con tối ưu, và sau đó rút ra phương trình chuyển trạng thái**

Sau khi đưa ra quyết định cho đồ vật $i$, những gì còn lại là bài toán con của $i-1$ đồ vật đầu tiên, có thể chia thành hai trường hợp sau.

- **Không cho đồ vật $i$ vào**: Sức chứa cái túi giữ nguyên, và trạng thái chuyển thành $[i-1, c]$.
- **Cho đồ vật $i$ vào**: Sức chứa cái túi giảm đi $wgt[i-1]$, giá trị tăng thêm $val[i-1]$, và trạng thái chuyển thành $[i-1, c-wgt[i-1]]$.

Phân tích trên tiết lộ cấu trúc con tối ưu của bài toán này: **giá trị tối đa $dp[i, c]$ bằng giá trị lớn hơn trong các giá trị thu được bằng cách không cho đồ vật $i$ vào cái túi và cho nó vào cái túi**. Từ đây, phương trình chuyển trạng thái có thể được rút ra:

$$
dp[i, c] = \max(dp[i-1, c], dp[i-1, c - wgt[i-1]] + val[i-1])
$$

Lưu ý rằng nếu trọng lượng của đồ vật hiện tại $wgt[i - 1]$ vượt quá sức chứa còn lại của cái túi $c$, thì lựa chọn duy nhất là không cho nó vào cái túi.

**Bước 3: Xác định các điều kiện biên và thứ tự chuyển trạng thái**

Khi không có đồ vật nào hoặc sức chứa cái túi là $0$, giá trị tối đa là $0$, tức là cột đầu tiên $dp[i, 0]$ và hàng đầu tiên $dp[0, c]$ đều bằng $0$.

Trạng thái hiện tại $[i, c]$ chuyển trạng thái từ trạng thái phía trên $[i-1, c]$ và trạng thái phía trên bên trái $[i-1, c-wgt[i-1]]$, vì vậy chúng ta có thể duyệt qua toàn bộ bảng $dp$ theo thứ tự xuôi bằng hai vòng lặp lồng nhau.

Dựa trên phân tích trên, tiếp theo chúng ta sẽ lần lượt triển khai các giải pháp tìm kiếm vét cạn, ghi nhớ và quy hoạch động.

### Phương pháp 1: Tìm kiếm Vét cạn

Mã nguồn tìm kiếm bao gồm các yếu tố sau.

- **Các tham số đệ quy**: trạng thái $[i, c]$.
- **Giá trị trả về**: lời giải cho bài toán con $dp[i, c]$.
- **Điều kiện dừng**: khi không còn đồ vật nào ($i = 0$) hoặc sức chứa còn lại của cái túi là $0$, chấm dứt đệ quy và trả về giá trị $0$.
- **Cắt tỉa**: nếu trọng lượng của đồ vật hiện tại vượt quá sức chứa còn lại của cái túi, chỉ có tùy chọn không cho vào là khả thi.

```src
[file]{knapsack}-[class]{}-[func]{knapsack_dfs}
```

Như hiển thị trong hình bên dưới, vì mỗi đồ vật tạo ra hai nhánh tìm kiếm, không loại trừ và bao gồm nó, nên độ phức tạp thời gian là $O(2^n)$.

Quan sát cây đệ quy, dễ dàng thấy được các bài toán con chồng chéo, chẳng hạn như $dp[1, 10]$. Khi có nhiều đồ vật, sức chứa cái túi lớn, và đặc biệt là nhiều đồ vật có cùng trọng lượng, số lượng các bài toán con chồng chéo sẽ tăng lên đáng kể.

![Cây đệ quy tìm kiếm vét cạn cho bài toán cái túi 0-1](knapsack_problem.assets/knapsack_dfs.png)

### Phương pháp 2: Ghi nhớ (Memoization)

Để đảm bảo rằng các bài toán con chồng chéo chỉ được tính toán một lần, chúng ta sử dụng một danh sách ghi nhớ `mem` để ghi lại lời giải cho các bài toán con, trong đó `mem[i][c]` tương ứng với $dp[i, c]$.

Sau khi đưa vào ghi nhớ, **độ phức tạp thời gian phụ thuộc vào số lượng bài toán con**, chính là $O(n \times cap)$. Mã nguồn triển khai như sau:

```src
[file]{knapsack}-[class]{}-[func]{knapsack_dfs_mem}
```

Hình bên dưới hiển thị các nhánh tìm kiếm bị cắt tỉa trong phương pháp ghi nhớ.

![Cây đệ quy có ghi nhớ cho bài toán cái túi 0-1](knapsack_problem.assets/knapsack_dfs_mem.png)

### Phương pháp 3: Quy hoạch động

Quy hoạch động về bản chất là quá trình điền vào bảng $dp$ trong quá trình chuyển trạng thái. Mã nguồn như sau:

```src
[file]{knapsack}-[class]{}-[func]{knapsack_dp}
```

Như hiển thị trong hình bên dưới, cả độ phức tạp thời gian và độ phức tạp không gian đều được quyết định bởi kích thước của mảng `dp`, đó là $O(n \times cap)$.

=== "<1>"
    ![Quá trình quy hoạch động cho bài toán cái túi 0-1](knapsack_problem.assets/knapsack_dp_step1.png)

=== "<2>"
    ![knapsack_dp_step2](knapsack_problem.assets/knapsack_dp_step2.png)

=== "<3>"
    ![knapsack_dp_step3](knapsack_problem.assets/knapsack_dp_step3.png)

=== "<4>"
    ![knapsack_dp_step4](knapsack_problem.assets/knapsack_dp_step4.png)

=== "<5>"
    ![knapsack_dp_step5](knapsack_problem.assets/knapsack_dp_step5.png)

=== "<6>"
    ![knapsack_dp_step6](knapsack_problem.assets/knapsack_dp_step6.png)

=== "<7>"
    ![knapsack_dp_step7](knapsack_problem.assets/knapsack_dp_step7.png)

=== "<8>"
    ![knapsack_dp_step8](knapsack_problem.assets/knapsack_dp_step8.png)

=== "<9>"
    ![knapsack_dp_step9](knapsack_problem.assets/knapsack_dp_step9.png)

=== "<10>"
    ![knapsack_dp_step10](knapsack_problem.assets/knapsack_dp_step10.png)

=== "<11>"
    ![knapsack_dp_step11](knapsack_problem.assets/knapsack_dp_step11.png)

=== "<12>"
    ![knapsack_dp_step12](knapsack_problem.assets/knapsack_dp_step12.png)

=== "<13>"
    ![knapsack_dp_step13](knapsack_problem.assets/knapsack_dp_step13.png)

=== "<14>"
    ![knapsack_dp_step14](knapsack_problem.assets/knapsack_dp_step14.png)

### Tối ưu hóa Không gian

Vì mỗi trạng thái chỉ liên quan đến trạng thái ở hàng phía trên nó, chúng ta có thể sử dụng hai mảng cuốn chiếu lũy tiến để giảm độ phức tạp không gian từ $O(n^2)$ xuống $O(n)$.

Suy nghĩ xa hơn, liệu chúng ta có thể đạt được tối ưu hóa không gian chỉ bằng một mảng? Quan sát kỹ, chúng ta có thể thấy rằng mỗi trạng thái được chuyển từ ô trực tiếp phía trên hoặc ô ở phía trên bên trái. Nếu chỉ có một mảng, khi chúng ta bắt đầu duyệt hàng $i$, mảng đó vẫn lưu trữ trạng thái của hàng $i-1$.

- Nếu sử dụng duyệt xuôi, khi duyệt đến $dp[i, j]$, các giá trị ở phía trên bên trái $dp[i-1, 1]$ ~ $dp[i-1, j-1]$ có thể đã bị ghi đè, từ đó ngăn cản việc chuyển trạng thái chính xác.
- Nếu sử dụng duyệt ngược, sẽ không có vấn đề ghi đè, và việc chuyển trạng thái có thể tiến hành chính xác.

Hình bên dưới hiển thị quá trình chuyển trạng thái từ hàng $i = 1$ sang hàng $i = 2$ bằng cách sử dụng một mảng duy nhất. Xin hãy cân nhắc sự khác biệt giữa duyệt xuôi và duyệt ngược.

=== "<1>"
    ![Quá trình quy hoạch động tối ưu không gian cho cái túi 0-1](knapsack_problem.assets/knapsack_dp_comp_step1.png)

=== "<2>"
    ![knapsack_dp_comp_step2](knapsack_problem.assets/knapsack_dp_comp_step2.png)

=== "<3>"
    ![knapsack_dp_comp_step3](knapsack_problem.assets/knapsack_dp_comp_step3.png)

=== "<4>"
    ![knapsack_dp_comp_step4](knapsack_problem.assets/knapsack_dp_comp_step4.png)

=== "<5>"
    ![knapsack_dp_comp_step5](knapsack_problem.assets/knapsack_dp_comp_step5.png)

=== "<6>"
    ![knapsack_dp_comp_step6](knapsack_problem.assets/knapsack_dp_comp_step6.png)

Trong triển khai mã nguồn, chúng ta đơn giản chỉ cần xóa chiều đầu tiên $i$ của mảng `dp` và thay đổi vòng lặp bên trong thành duyệt ngược:

```src
[file]{knapsack}-[class]{}-[func]{knapsack_dp_comp}
```
