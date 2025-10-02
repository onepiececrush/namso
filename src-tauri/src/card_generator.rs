use rand::Rng;
use serde::{Deserialize, Serialize};
use chrono::{Datelike, Local};
use crate::networks::CardNetworkRegistry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardData {
    pub number: String,
    pub network: String,
    pub exp_month: String,
    pub exp_year: String,
    pub expiry: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    pub bin: String,
}

pub struct CreditCardGenerator;

impl CreditCardGenerator {
    pub fn new() -> Self {
        Self
    }
    
    /// Luhn 算法校验和计算
    pub fn luhn_checksum(card_num: &str) -> u32 {
        let digits: Vec<u32> = card_num.chars()
            .filter_map(|c| c.to_digit(10))
            .collect();
        
        let odd_sum: u32 = digits.iter().rev().step_by(2).sum();
        let even_sum: u32 = digits.iter().rev().skip(1).step_by(2)
            .map(|&d| {
                let doubled = d * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            })
            .sum();
        
        (odd_sum + even_sum) % 10
    }
    
    /// 验证卡号是否符合 Luhn 算法
    pub fn is_luhn_valid(card_num: &str) -> bool {
        Self::luhn_checksum(card_num) == 0
    }
    
    /// 生成卡号
    pub fn generate_card_number(
        &self,
        network: &str,
        bin_code: Option<&str>
    ) -> Result<String, String> {
        let network_info = CardNetworkRegistry::get_network_by_identifier(network)
            .ok_or_else(|| format!("不支持的网络类型: {}", network))?;
        
        let mut rng = rand::thread_rng();
        
        // 处理 BIN 码
        let (chosen_bin, target_length) = if let Some(bin) = bin_code {
            if bin.contains('x') || bin.contains('X') {
                // 处理带 x 占位符的 BIN
                let processed = self.process_bin_with_placeholder(bin)?;
                let length = bin.len();
                (processed, length)
            } else {
                // 纯数字 BIN，使用默认长度
                let length = *network_info.length.iter().next().unwrap();
                (bin.to_string(), length)
            }
        } else {
            // 随机选择 BIN 和长度
            let bin_index = rng.gen_range(0..network_info.bins.len());
            let length_index = rng.gen_range(0..network_info.length.len());
            (network_info.bins[bin_index].to_string(), network_info.length[length_index])
        };
        
        // 生成卡号（不包含校验位）
        let mut card_number = chosen_bin;
        while card_number.len() < target_length - 1 {
            card_number.push_str(&rng.gen_range(0..10).to_string());
        }
        
        // 如果超过目标长度，截断
        if card_number.len() > target_length - 1 {
            card_number.truncate(target_length - 1);
        }
        
        // 计算并添加校验位
        let checksum = Self::luhn_checksum(&format!("{}0", card_number));
        let check_digit = (10 - checksum) % 10;
        card_number.push_str(&check_digit.to_string());
        
        Ok(card_number)
    }
    
    /// 处理带占位符的 BIN
    fn process_bin_with_placeholder(&self, bin: &str) -> Result<String, String> {
        let mut rng = rand::thread_rng();
        
        // 提取固定部分（x 之前的数字）
        let fixed_part: String = bin.chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        
        // 计算需要生成的随机数字数量（x 的数量减去校验位）
        let x_count = bin.chars().filter(|c| *c == 'x' || *c == 'X').count();
        
        // 生成随机部分（不包括校验位）
        let mut result = fixed_part;
        for _ in 0..(x_count - 1) {
            result.push_str(&rng.gen_range(0..10).to_string());
        }
        
        Ok(result)
    }
    
    /// 生成过期日期
    pub fn generate_expiry(&self, month: Option<u32>, year: Option<i32>) -> (String, String) {
        let mut rng = rand::thread_rng();
        let current_date = Local::now();
        let current_year = current_date.year();
        let current_month = current_date.month();

        let (exp_month, exp_year) = if let Some(m) = month {
            if let Some(y) = year {
                // 如果都指定了，验证是否是未来日期
                if y < current_year || (y == current_year && m < current_month) {
                    // 如果不是未来日期，使用默认逻辑
                    let default_year = rng.gen_range(current_year..=current_year + 8);
                    if default_year == current_year {
                        (rng.gen_range(current_month..=12), default_year)
                    } else {
                        (rng.gen_range(1..=12), default_year)
                    }
                } else {
                    (m, y)
                }
            } else {
                // 只指定月份，生成年份
                let default_year = rng.gen_range(current_year..=current_year + 8);
                if default_year == current_year && m < current_month {
                    (rng.gen_range(current_month..=12), default_year)
                } else {
                    (m, default_year)
                }
            }
        } else if let Some(y) = year {
            // 只指定年份，生成月份
            if y == current_year {
                // 如果是当前年份，月份必须大于等于当前月份
                (rng.gen_range(current_month..=12), y)
            } else {
                (rng.gen_range(1..=12), y)
            }
        } else {
            // 都没指定，完全随机生成未来日期
            let random_year = rng.gen_range(current_year..=current_year + 8);
            let random_month = if random_year == current_year {
                rng.gen_range(current_month..=12)
            } else {
                rng.gen_range(1..=12)
            };
            (random_month, random_year)
        };

        (format!("{:02}", exp_month), exp_year.to_string())
    }
    
    /// 生成 CVV
    pub fn generate_cvv(&self, network: &str) -> Result<String, String> {
        let network_info = CardNetworkRegistry::get_network_by_identifier(network)
            .ok_or_else(|| format!("不支持的网络类型: {}", network))?;
        
        let mut rng = rand::thread_rng();
        let cvv: String = (0..network_info.cvv_length)
            .map(|_| char::from_digit(rng.gen_range(0..10), 10).unwrap())
            .collect();
        
        Ok(cvv)
    }
    
    /// 生成余额
    pub fn generate_balance(&self, min: f64, max: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let balance = rng.gen_range(min..=max);
        (balance * 100.0).round() / 100.0
    }
    
    /// 生成完整卡数据
    pub fn generate_card_data(
        &self,
        network: &str,
        quantity: usize,
        exp_month: Option<u32>,
        exp_year: Option<i32>,
        include_cvv: bool,
        include_balance: bool,
        currency: Option<String>,
        bin_code: Option<String>,
    ) -> Result<Vec<CardData>, String> {
        let mut cards = Vec::new();
        
        // 处理随机网络
        let mut rng = rand::thread_rng();
        let all_networks: Vec<String> = CardNetworkRegistry::get_all_networks()
            .keys()
            .map(|k| k.to_string())
            .collect();
        
        for _ in 0..quantity {
            let chosen_network = if network == "random" {
                &all_networks[rng.gen_range(0..all_networks.len())]
            } else {
                network
            };
            
            let card_number = self.generate_card_number(
                chosen_network, 
                bin_code.as_deref()
            )?;
            
            let (month, year) = self.generate_expiry(exp_month, exp_year);
            
            let cvv = if include_cvv {
                Some(self.generate_cvv(chosen_network)?)
            } else {
                None
            };
            
            let balance = if include_balance {
                Some(self.generate_balance(100.0, 10000.0))
            } else {
                None
            };
            
            let network_name = CardNetworkRegistry::get_network_by_identifier(chosen_network)
                .map(|n| n.name.to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            
            let year_short: String = year.chars().skip(2).collect();
            let expiry = format!("{}/{}", month, year_short);
            let bin = card_number.chars().take(6).collect::<String>();
            
            cards.push(CardData {
                number: card_number,
                network: network_name,
                exp_month: month,
                exp_year: year,
                expiry,
                cvv,
                balance,
                currency: if include_balance { currency.clone() } else { None },
                bin,
            });
        }
        
        Ok(cards)
    }
    
    /// 获取所有支持的网络名称
    #[allow(dead_code)]
    pub fn get_networks(&self) -> Vec<String> {
        CardNetworkRegistry::get_all_networks()
            .keys()
            .map(|k| k.to_string())
            .collect()
    }
}

impl Default for CreditCardGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_luhn_checksum() {
        assert_eq!(CreditCardGenerator::luhn_checksum("4111111111111111"), 0);
        assert_eq!(CreditCardGenerator::luhn_checksum("5555555555554444"), 0);
        assert!(CreditCardGenerator::is_luhn_valid("4111111111111111"));
    }
    
    #[test]
    fn test_generate_card() {
        let generator = CreditCardGenerator::new();
        let result = generator.generate_card_number("visa", None);
        assert!(result.is_ok());
        
        let card_num = result.unwrap();
        assert!(card_num.starts_with('4'));
        assert!(CreditCardGenerator::is_luhn_valid(&card_num));
    }
    
    #[test]
    fn test_bin_with_placeholder() {
        let generator = CreditCardGenerator::new();
        let result = generator.generate_card_number("visa", Some("4532xxxxxxxxxxxxxx"));
        assert!(result.is_ok());
        
        let card_num = result.unwrap();
        assert!(card_num.starts_with("4532"));
        assert!(CreditCardGenerator::is_luhn_valid(&card_num));
    }
}


