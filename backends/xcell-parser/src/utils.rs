/// 规范化字符串，去除分隔符并转为小写
///
/// 将字符串中的 `-`、`_`、空格移除，并将 ASCII 字符转为小写，
/// 非 ASCII 字符保持不变。
///
/// # 参数
/// * `s` - 待规范化的字符串
///
/// # 返回值
/// 规范化后的字符串
pub fn norm_string(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for char in s.chars() {
        if char == '-' || char == '_' || char == ' ' {
            continue;
        }
        if char.is_ascii() {
            out.push(char.to_ascii_lowercase());
        }
        else {
            out.push(char);
        }
    }
    out
}
