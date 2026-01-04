/// 従業員名のバリデーション
///
/// ルール:
/// - 空文字列は不可
/// - 前後の空白は不可
/// - 文字間の空白は不可
/// - 漢字とカタカナのみ許可
pub fn validate_employee_name(name: &str) -> Result<(), String> {
    // 空文字列チェック
    if name.is_empty() {
        return Err("名前を入力してください".to_string());
    }

    // 前後の空白チェック
    if name != name.trim() {
        return Err("名前の前後に空白を含めることはできません".to_string());
    }

    // 空白が含まれているかチェック
    if name.contains(char::is_whitespace) {
        return Err("名前に空白を含めることはできません".to_string());
    }

    // 漢字とカタカナのみかチェック
    let is_valid = name.chars().all(|c| {
        // 漢字（CJK統合漢字）
        ('\u{4E00}'..='\u{9FFF}').contains(&c) ||
        // カタカナ
        ('\u{30A0}'..='\u{30FF}').contains(&c)
    });

    if !is_valid {
        return Err("名前は漢字とカタカナのみ使用できます".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(validate_employee_name("山田太郎").is_ok());
        assert!(validate_employee_name("タナカハナコ").is_ok());
        assert!(validate_employee_name("佐藤カオリ").is_ok());
    }

    #[test]
    fn test_empty_name() {
        assert!(validate_employee_name("").is_err());
    }

    #[test]
    fn test_whitespace_prefix_suffix() {
        assert!(validate_employee_name(" 山田太郎").is_err());
        assert!(validate_employee_name("山田太郎 ").is_err());
        assert!(validate_employee_name(" 山田太郎 ").is_err());
    }

    #[test]
    fn test_whitespace_in_middle() {
        assert!(validate_employee_name("山田 太郎").is_err());
        assert!(validate_employee_name("タナカ　ハナコ").is_err());
    }

    #[test]
    fn test_invalid_characters() {
        assert!(validate_employee_name("yamada").is_err());
        assert!(validate_employee_name("山田123").is_err());
        assert!(validate_employee_name("山田!太郎").is_err());
        assert!(validate_employee_name("やまだ").is_err()); // ひらがなはNG
    }
}
