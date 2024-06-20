#![warn(missing_docs)]

use proc_macro::TokenStream;

/// 示例宏，用于演示 proc macro 的基本结构
#[proc_macro]
pub fn example_macro(input: TokenStream) -> TokenStream {
    // 这里可以实现宏的逻辑
    input
}

/// 示例 derive 宏
#[proc_macro_derive(ExampleDerive)]
pub fn example_derive(input: TokenStream) -> TokenStream {
    // 这里可以实现 derive 宏的逻辑
    input
}

/// 示例属性宏
#[proc_macro_attribute]
pub fn example_attribute(args: TokenStream, input: TokenStream) -> TokenStream {
    // 这里可以实现属性宏的逻辑
    input
}
