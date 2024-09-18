// 引入必要的标准库和第三方库
use std::{ 
    fmt, // 导入格式化相关功能
    ops::{Add, AddAssign, Mul}, // 导入加法、加法赋值和乘法特征
    sync::mpsc, // 导入多生产者单消费者通道
    thread, // 导入线程模块
};
use anyhow::{anyhow, Result}; // 导入 Result 类型和错误处理工具
use crate::dot_product; // 引入点积函数
use crate::Vector; // 引入 Vector 结构体

// 定义线程数量常量
const NUM_THREADS: usize = 4;

// 定义矩阵结构体 Matrix，包含矩阵的数据、行数和列数
pub struct Matrix<T> {
    data: Vec<T>, // 存储矩阵数据的向量
    row: usize, // 矩阵的行数
    col: usize, // 矩阵的列数
}

// 定义输入消息结构体 MsgInput，包含索引、行向量和列向量
pub struct MsgInput<T> {
    idx: usize, // 索引
    row: Vector<T>, // 行向量
    col: Vector<T>, // 列向量
}

// 定义输出消息结构体 MsgOutput，包含索引和计算结果
pub struct MsgOutput<T> {
    idx: usize, // 索引
    value: T, // 计算结果
}

// 定义消息结构体 Msg，包含输入消息和发送者
pub struct Msg<T> {
    input: MsgInput<T>, // 输入消息
    sender: oneshot::Sender<MsgOutput<T>>, // 用于发送结果的通道
}

// 为矩阵实现乘法运算符
impl<T> Mul for Matrix<T> 
where 
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    type Output = Self; // 定义乘法运算的返回类型为 Matrix<T>

    // 实现乘法运算
    fn mul(self, rhs: Self) -> Self::Output {
        // 调用 multiply 函数进行矩阵乘法
        multiply(&self, &rhs).expect("Matrix multiply error")
    }
}

// 定义矩阵乘法函数，接受两个矩阵的引用并返回结果
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>> 
where 
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T> + Send + 'static,
{
    // 检查两个矩阵的列数和行数是否匹配
    if a.col != b.row {
        return Err(anyhow!("矩阵乘法错误：a.col != b.row")); // 返回错误信息
    }

    // 创建多个线程，发送和接收消息的通道
    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>(); // 创建通道
            thread::spawn(move || { // 启动新线程
                for msg in rx { // 接收消息
                    let value = dot_product(msg.input.row, msg.input.col)?; // 计算点积
                    // 发送计算结果
                    if let Err(e) = msg.sender.send(MsgOutput { idx: msg.input.idx, value }) {
                        eprintln!("发送错误: {:?}", e); // 输出错误信息
                    }
                } 
                Ok::<_, anyhow::Error>(())
            });
            tx // 返回发送者
        }).collect::<Vec<_>>(); // 收集所有发送者

    let matrix_len = a.row * b.col; // 计算结果矩阵的大小
    let mut data = vec![T::default(); matrix_len]; // 初始化结果数据
    let mut receivers = Vec::with_capacity(matrix_len); // 初始化接收者

    // map/reduce 的映射阶段
    for i in 0..a.row {
        for j in 0..b.col {
            // 创建当前行向量
            let row = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            // 提取当前列数据
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col) // 按列步进
                .copied()
                .collect::<Vec<_>>(); // 收集为 Vec<T>
            let col = Vector::new(col_data); // 创建列向量
            let idx = i * b.col + j; // 计算结果索引
            let input = MsgInput::new(idx, row, col); // 创建输入消息
            let (tx, rx) = oneshot::channel(); // 创建单次发送的通道
            let msg = Msg::new(input, tx); // 创建消息

            // 发送消息到对应线程
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprint!("发送错误: {:?}", e); // 输出错误信息
            }
            receivers.push(rx); // 收集接收者
        }
    }

    // map/reduce 的归约阶段
    for rx in receivers {
        let output = rx.recv()?; // 接收结果
        data[output.idx] = output.value; // 存储结果
    }

    // 返回结果矩阵
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

// 为矩阵实现构造函数
impl<T: fmt::Debug> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self { data: data.into(), row, col } // 初始化矩阵
    }
}

// 为输入消息实现构造函数
impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self { idx, row, col } // 初始化输入消息
    }
}

// 为消息实现构造函数
impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender } // 初始化消息
    }
}

// 为矩阵实现 fmt::Display 特征，用于格式化输出
impl<T> fmt::Display for Matrix<T> 
where 
    T: fmt::Display, // T 必须实现 fmt::Display
{
    // 定义格式化输出的方法
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?; // 开始大括号

        for i in 0..self.row { // 遍历每一行
            for j in 0..self.col { // 遍历每一列
                write!(f, "{}", self.data[i * self.col + j])?; // 写入当前元素
                if j != self.col - 1 {
                    write!(f, " ")?; // 列之间加空格
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?; // 行之间加逗号
            }
        }
        write!(f, "}}")?; // 结束大括号
        Ok(()) // 返回结果
    }
}

// 为矩阵实现 fmt::Debug 特征，用于调试输出
impl<T> fmt::Debug for Matrix<T> 
where 
    T: fmt::Display, // T 必须实现 fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix( row: {}, col: {}, {})", self.row, self.col, self) // 输出行列信息和内容
    }
}

// 测试模块
#[cfg(test)]
mod tests {
    use anyhow::Ok; // 导入 Ok 以便使用

    use super::*; // 引入上层模块的所有内容

    // 测试矩阵乘法
    #[test]
    fn test_matrix_multiply() -> Result<()> {
        // 创建两个测试矩阵
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b; // 执行矩阵乘法
        assert_eq!(c.col, 2); // 检查列数
        assert_eq!(c.row, 2); // 检查行数
        assert_eq!(c.data, vec![22, 28, 49, 64]); // 检查计算结果

        assert_eq!(format!("{:?}", c), "Matrix( row: 2, col: 2, {22 28, 49 64})"); // 检查调试输出
        Ok(()) // 返回成功
    }

    // 测试矩阵的显示输出
    #[test]
    fn test_matrix_display() -> Result<()> {
        let a = Matrix::new([1, 2, 3, 4], 2, 2); // 创建矩阵 a
        let b = Matrix::new([1, 2, 3, 4], 2, 2); // 创建矩阵 b
        let c = a * b; // 执行矩阵乘法
        assert_eq!(c.data, vec![7, 10, 15, 22]); // 检查计算结果
        assert_eq!(format!("{}", c), "{7 10, 15 22}"); // 检查格式化输出
        Ok(()) // 返回成功
    }

    // 测试无法进行的矩阵乘法
    #[test]
    fn test_a_can_not_multiply_b() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3); // 创建矩阵 a
        let b = Matrix::new([1, 2, 3, 4], 2, 2); // 创建矩阵 b
        let c = multiply(&a, &b); // 尝试执行矩阵乘法
        assert!(c.is_err()); // 检查是否返回错误
    }

    // 测试无法进行的矩阵乘法，预期会 panic
    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3); // 创建矩阵 a
        let b = Matrix::new([1, 2, 3, 4], 2, 2); // 创建矩阵 b
        let _c = a * b; // 尝试执行矩阵乘法，预期会 panic
    }
}
