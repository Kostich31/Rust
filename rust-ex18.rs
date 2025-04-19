pub fn luhn(cc_number: &str) -> bool {
    let mut digits = Vec::new();
    
    // Собираем все цифры, игнорируя пробелы и другие символы
    for c in cc_number.chars() {
        if c.is_whitespace() {
            continue;
        }
        if let Some(digit) = c.to_digit(10) {
            digits.push(digit);
        } else {
            return false; // Нецифровой символ - сразу невалидно
        }
    }
    
    // Проверяем минимальную длину
    if digits.len() < 2 {
        return false;
    }
    
    let mut sum = 0;
    let mut double = false;
    
    // Обрабатываем цифры справа налево
    for &digit in digits.iter().rev() {
        if double {
            let doubled = digit * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += digit;
        }
        double = !double;
    }
    
    sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }

    #[test]
    fn test_all_zeros() {
        assert!(luhn("0000 0000 0000 0000")); // 0*2=0 + 0 = 0 → valid
        assert!(luhn("00"));
    }

    #[test]
    fn test_alternating_nines() {
        assert!(!luhn("9999 9999 9999 9999")); // 9*2=18→9 + 9 = 18 → 18%10≠0
        assert!(luhn("9999 9999 9999 9995")); // Специально подобранный валидный
    }

    #[test]
    fn test_long_numbers() {
        assert!(luhn("1234 5678 9012 3456 7890 1234 5678 9012 3456")); // Длинный номер
        assert!(!luhn("1234 5678 9012 3456 7890 1234 5678 9012 3457")); // Инвалидная версия
    }

    #[test]
    fn test_single_space_between_groups() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4263  9826  4026  9299")); // Множественные пробелы
        assert!(luhn("4263982640269299")); // Без пробелов
    }

    #[test]
    fn test_unicode_spaces() {
        assert!(luhn("4263\u{00A0}9826\u{2007}4026\u{202F}9299")); // Разные виды пробелов
        assert!(!luhn("4263\u{00A0}9826\u{2007}4026\u{202F}9298")); // Инвалидная версия
    }

    #[test]
    fn test_only_one_digit_after_spaces() {
        assert!(!luhn("4 2 6 3 9 8 2 6 4 0 2 6 9 2 9"));
        assert!(luhn("4 2 6 3 9 8 2 6 4 0 2 6 9 2 9 9")); // Добавили последнюю 9
    }

    #[test]
    fn test_leading_trailing_spaces() {
        assert!(luhn("  4263 9826 4026 9299  "));
        assert!(!luhn("  4223 9826 4026 9299  ")); // Инвалидная версия
    }

    #[test]
    fn test_special_chars_mixed() {
        assert!(!luhn("4263-9826-4026-9299"));
        assert!(!luhn("4263 9826 4026 9299X"));
        assert!(!luhn("CC: 4263 9826 4026 9299"));
    }

    #[test]
    fn test_empty_string() {
        assert!(!luhn(""));
        assert!(!luhn("  "));
    }

    #[test]
    fn test_single_digit() {
        assert!(!luhn("0"));
        assert!(!luhn("1"));
    }

    #[test]
    fn test_non_digit_chars() {
        assert!(!luhn("foo"));
        assert!(!luhn("4263 9826 4026 9299!"));
        assert!(!luhn("1234-5678-9012-3456"));
    }

    #[test]
    fn test_valid_numbers_with_spaces() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4 2 6 3 9 8 2 6 4 0 2 6 9 2 9 9"));
    }

    #[test]
    fn test_edge_case_min_length() {
        assert!(luhn("91"));  // 9*2=18→9 + 1 = 10 → valid
        assert!(!luhn("80")); // 8*2=16→7 + 0 = 7 → invalid
    }

    #[test]
    fn test_double_nine_handling() {
        assert!(luhn("59"));  // 5*2=10→1 + 9 = 10 → valid
        assert!(!luhn("58")); // 5*2=10→1 + 8 = 9 → invalid
    }

    #[test]
    fn test_known_valid_numbers() {
        // Test Visa
        assert!(luhn("4111 1111 1111 1111"));
        // Test Mastercard
        assert!(luhn("5555 5555 5555 4444"));
        // Test American Express
        assert!(luhn("3782 822463 10005"));
    }

    #[test]
    fn test_known_invalid_numbers() {
        assert!(!luhn("4111 1111 1111 1112"));
        assert!(!luhn("5555 5555 5555 4445"));
        assert!(!luhn("3782 822463 10006"));
    }
}
