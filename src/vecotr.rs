// 引入必要的标准库和第三方库
use std::ops::Deref; // 导入 Deref 特征，以便实现自定义的解引用行为
use anyhow::{Result, anyhow}; // 导入 Result 类型和错误处理工具
use std::ops::*; // 导入运算符特征

// 定义一个向量结构体 Vector，包含一个 Vec<T> 类型的字段 data
pub struct Vector<T> {
    data: Vec<T>, // 存储向量数据的向量
}

// 定义点积函数，接受两个 Vector<T> 类型的参数 a 和 b，并返回一个 Result<T>
pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> Result<T> 
where  
    T: Copy + Default + Add<Output = T> + AddAssign + Mul<Output = T>, // T 类型需要实现的特征
{
    // 检查两个向量的长度是否相同，如果不同则返回错误
    if a.len() != b.len() {
        return Err(anyhow!("点积错误：a.len != b.len")); // 返回具体错误信息
    }

    let mut sum = T::default(); // 初始化 sum，使用 T 的默认值
    // 遍历向量，计算点积
    for i in 0..a.len() {
        sum += a[i] * b[i]; // 累加每个元素的乘积
    }

    Ok(sum) // 返回计算得到的点积
}

// 为 Vector<T> 实现 Deref 特征，允许对 Vector<T> 使用解引用
impl<T> Deref for Vector<T> {
    type Target = Vec<T>; // 指定解引用后返回的类型为 Vec<T>

    // 实现解引用方法，返回 data 字段的引用
    fn deref(&self) -> &Self::Target {
        &self.data // 返回对内部 Vec<T> 的引用
    }
}

// 为 Vector<T> 实现一些方法
impl<T> Vector<T> {
    // 创建一个新的 Vector 实例，接受任何能够转换为 Vec<T> 的数据
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() } // 将传入数据转换为 Vec<T> 并存储在 data 字段中
    }
}
