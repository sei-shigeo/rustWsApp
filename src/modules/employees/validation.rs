/// 従業員コードのバリデーション
///
/// ルール:
/// - 空文字列は不可
/// - 前後の空白は不可
/// - 文字間の空白は不可
/// - 半角大文字アルファベット(A-Z)と数字(0-9)のみ許可
pub fn validate_employee_code(code: &str) -> Result<(), String> {
    // 空文字列チェック
    if code.is_empty() {
        return Err("従業員コードを入力してください".to_string());
    }

    // 前後の空白チェック
    if code != code.trim() {
        return Err("従業員コードの前後に空白を含めることはできません".to_string());
    }

    // 空白が含まれているかチェック
    if code.contains(char::is_whitespace) {
        return Err("従業員コードに空白を含めることはできません".to_string());
    }

    // 半角大文字アルファベットと数字のみかチェック
    let is_valid = code
        .chars()
        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit());

    if !is_valid {
        return Err(
            "従業員コードは半角大文字アルファベット(A-Z)と数字(0-9)のみ使用できます".to_string(),
        );
    }

    Ok(())
}

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

/// 郵便番号のバリデーション
///
/// ルール:
/// - 空文字列は不可
/// - 7桁の数字のみ許可（ハイフンなし）
pub fn validate_postal_code(postal_code: &str) -> Result<(), String> {
    if postal_code.is_empty() {
        return Err("郵便番号を入力してください".to_string());
    }

    if postal_code.len() != 7 {
        return Err("郵便番号は7桁で入力してください".to_string());
    }

    if !postal_code.chars().all(|c| c.is_ascii_digit()) {
        return Err("郵便番号は数字のみで入力してください".to_string());
    }

    Ok(())
}

/// 住所フィールドのバリデーション
///
/// ルール:
/// - 空文字列は不可
/// - 前後の空白は削除して検証
pub fn validate_address_field(field: &str, field_name: &str) -> Result<(), String> {
    let trimmed = field.trim();

    if trimmed.is_empty() {
        return Err(format!("{}を入力してください", field_name));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_employee_codes() {
        assert!(validate_employee_code("EMP001").is_ok());
        assert!(validate_employee_code("ABC123").is_ok());
        assert!(validate_employee_code("A1B2C3").is_ok());
        assert!(validate_employee_code("12345").is_ok());
        assert!(validate_employee_code("ABCDE").is_ok());
    }

    #[test]
    fn test_invalid_employee_codes() {
        assert!(validate_employee_code("").is_err()); // 空文字列NG
        assert!(validate_employee_code(" EMP001").is_err()); // 前の空白
        assert!(validate_employee_code("EMP001 ").is_err()); // 後ろの空白
        assert!(validate_employee_code("EMP 001").is_err()); // 間の空白
        assert!(validate_employee_code("emp001").is_err()); // 小文字
        assert!(validate_employee_code("EMP-001").is_err()); // ハイフン
        assert!(validate_employee_code("EMP_001").is_err()); // アンダースコア
        assert!(validate_employee_code("従業員001").is_err()); // 日本語
        assert!(validate_employee_code("EMP001!").is_err()); // 記号
    }

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

    #[test]
    fn test_valid_postal_codes() {
        assert!(validate_postal_code("1234567").is_ok());
        assert!(validate_postal_code("0000000").is_ok());
    }

    #[test]
    fn test_invalid_postal_codes() {
        assert!(validate_postal_code("").is_err()); // 空文字列
        assert!(validate_postal_code("123-4567").is_err()); // ハイフン付き
        assert!(validate_postal_code("12345").is_err()); // 桁数不足
        assert!(validate_postal_code("123456789").is_err()); // 桁数超過
        assert!(validate_postal_code("abcd567").is_err()); // 英字含む
    }

    #[test]
    fn test_valid_address_fields() {
        assert!(validate_address_field("東京都", "都道府県").is_ok());
        assert!(validate_address_field("千代田区", "市区町村").is_ok());
        assert!(validate_address_field("  千代田区  ", "市区町村").is_ok()); // 前後の空白は許可
    }

    #[test]
    fn test_invalid_address_fields() {
        assert!(validate_address_field("", "都道府県").is_err());
        assert!(validate_address_field("   ", "都道府県").is_err());
    }
}
