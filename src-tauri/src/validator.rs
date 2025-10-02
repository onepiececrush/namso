use crate::card_generator::CreditCardGenerator;
use crate::networks::CardNetworkRegistry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub luhn_valid: bool,
    pub network: Option<String>,
    pub length: usize,
    pub reason: String,
}

/// 验证信用卡号码
pub fn validate_card_number(card_number: &str) -> ValidationResult {
    // 清理输入，只保留数字
    let clean_number: String = card_number.chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    
    // 检查长度
    if clean_number.len() < 13 || clean_number.len() > 19 {
        return ValidationResult {
            valid: false,
            luhn_valid: false,
            network: None,
            length: clean_number.len(),
            reason: "卡号长度无效".to_string(),
        };
    }
    
    // 检查 Luhn 算法
    let luhn_valid = CreditCardGenerator::is_luhn_valid(&clean_number);
    
    // 识别网络
    let network = CardNetworkRegistry::detect_network(&clean_number);
    
    let valid = luhn_valid && network.is_some();
    let reason = if valid {
        "有效".to_string()
    } else if !luhn_valid {
        "Luhn 校验失败".to_string()
    } else {
        "无法识别卡网络".to_string()
    };
    
    ValidationResult {
        valid,
        luhn_valid,
        network,
        length: clean_number.len(),
        reason,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validate_visa() {
        let result = validate_card_number("4111111111111111");
        assert!(result.valid);
        assert!(result.luhn_valid);
        assert_eq!(result.network, Some("Visa".to_string()));
    }
    
    #[test]
    fn test_validate_mastercard() {
        let result = validate_card_number("5555555555554444");
        assert!(result.valid);
        assert!(result.luhn_valid);
        assert_eq!(result.network, Some("Mastercard".to_string()));
    }
    
    #[test]
    fn test_validate_invalid() {
        let result = validate_card_number("1234567890123456");
        assert!(!result.valid);
    }
    
    #[test]
    fn test_validate_invalid_length() {
        let result = validate_card_number("123");
        assert!(!result.valid);
        assert_eq!(result.reason, "卡号长度无效");
    }
}


