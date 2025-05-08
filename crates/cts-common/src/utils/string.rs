
/// 将八进制字符串转换成utf-8中文
/// ```
///  列如：\347\273\204\345\220\210 2 转换成 ”组合 02“
/// ```
pub fn parse_octal_escapes(s: &str) -> String {
    let mut result = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            // 读取 3 位八进制数字
            let mut octal_str = String::new();
            for _ in 0..3 {
                if let Some(&next_c) = chars.peek() {
                    if next_c.is_digit(8) { // 检查 0-7
                        octal_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
            }
            if octal_str.len() == 3 {
                if let Ok(byte) = u8::from_str_radix(&octal_str, 8) {
                    result.push(byte);
                }
            }
        } else {
            // 非转义字符直接转为字节
            result.extend(c.to_string().as_bytes());
        }
    }

    String::from_utf8_lossy(&result).into_owned() // 容忍无效 UTF-8
}
