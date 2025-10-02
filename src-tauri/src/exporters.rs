use crate::card_generator::CardData;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::io::Cursor;

/// 导出为 PIPE 格式
pub fn export_to_pipe(cards: &[CardData]) -> String {
    cards
        .iter()
        .map(|card| {
            let mut parts = vec![card.number.clone()];
            parts.push(format!("{}/{}", card.exp_month, &card.exp_year[2..]));
            if let Some(cvv) = &card.cvv {
                parts.push(cvv.clone());
            }
            parts.join("|")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// 导出为 CSV 格式
pub fn export_to_csv(cards: &[CardData]) -> Result<String, String> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    
    // 写入表头
    wtr.write_record(&[
        "number", "network", "exp_month", "exp_year", "expiry", 
        "cvv", "balance", "currency", "bin"
    ]).map_err(|e| e.to_string())?;
    
    // 写入数据
    for card in cards {
        wtr.write_record(&[
            &card.number,
            &card.network,
            &card.exp_month,
            &card.exp_year,
            &card.expiry,
            card.cvv.as_deref().unwrap_or(""),
            &card.balance.map(|b| b.to_string()).unwrap_or_default(),
            card.currency.as_deref().unwrap_or(""),
            &card.bin,
        ]).map_err(|e| e.to_string())?;
    }
    
    let data = wtr.into_inner().map_err(|e| e.to_string())?;
    String::from_utf8(data).map_err(|e| e.to_string())
}

/// 导出为 JSON 格式
pub fn export_to_json(cards: &[CardData]) -> Result<String, String> {
    serde_json::to_string_pretty(cards).map_err(|e| e.to_string())
}

/// 导出为 XML 格式
pub fn export_to_xml(cards: &[CardData]) -> Result<String, String> {
    let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    
    // XML 声明
    writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))
        .map_err(|e| e.to_string())?;
    
    // 根元素
    writer.write_event(Event::Start(BytesStart::new("cards")))
        .map_err(|e| e.to_string())?;
    
    for card in cards {
        // card 元素
        writer.write_event(Event::Start(BytesStart::new("card")))
            .map_err(|e| e.to_string())?;
        
        // 写入各字段
        write_xml_element(&mut writer, "number", &card.number)?;
        write_xml_element(&mut writer, "network", &card.network)?;
        write_xml_element(&mut writer, "exp_month", &card.exp_month)?;
        write_xml_element(&mut writer, "exp_year", &card.exp_year)?;
        write_xml_element(&mut writer, "expiry", &card.expiry)?;
        
        if let Some(cvv) = &card.cvv {
            write_xml_element(&mut writer, "cvv", cvv)?;
        }
        
        if let Some(balance) = card.balance {
            write_xml_element(&mut writer, "balance", &balance.to_string())?;
        }
        
        if let Some(currency) = &card.currency {
            write_xml_element(&mut writer, "currency", currency)?;
        }
        
        write_xml_element(&mut writer, "bin", &card.bin)?;
        
        // 关闭 card 元素
        writer.write_event(Event::End(BytesEnd::new("card")))
            .map_err(|e| e.to_string())?;
    }
    
    // 关闭根元素
    writer.write_event(Event::End(BytesEnd::new("cards")))
        .map_err(|e| e.to_string())?;
    
    let result = writer.into_inner().into_inner();
    String::from_utf8(result).map_err(|e| e.to_string())
}

fn write_xml_element(writer: &mut Writer<Cursor<Vec<u8>>>, name: &str, content: &str) -> Result<(), String> {
    writer.write_event(Event::Start(BytesStart::new(name)))
        .map_err(|e| e.to_string())?;
    writer.write_event(Event::Text(BytesText::new(content)))
        .map_err(|e| e.to_string())?;
    writer.write_event(Event::End(BytesEnd::new(name)))
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// 导出为 SQL 格式
pub fn export_to_sql(cards: &[CardData], table_name: &str) -> String {
    let mut sql_lines = vec![];
    
    // 创建表结构
    sql_lines.push(format!("CREATE TABLE IF NOT EXISTS {} (", table_name));
    sql_lines.push("    number VARCHAR(20),".to_string());
    sql_lines.push("    network VARCHAR(50),".to_string());
    sql_lines.push("    exp_month VARCHAR(4),".to_string());
    sql_lines.push("    exp_year VARCHAR(4),".to_string());
    sql_lines.push("    expiry VARCHAR(10),".to_string());
    sql_lines.push("    cvv VARCHAR(4),".to_string());
    sql_lines.push("    balance DECIMAL(10,2),".to_string());
    sql_lines.push("    currency VARCHAR(10),".to_string());
    sql_lines.push("    bin VARCHAR(20)".to_string());
    sql_lines.push(");".to_string());
    sql_lines.push("".to_string());
    
    // 插入数据
    for card in cards {
        let cvv = card.cvv.as_ref().map(|s| format!("'{}'", s)).unwrap_or("NULL".to_string());
        let balance = card.balance.map(|b| b.to_string()).unwrap_or("NULL".to_string());
        let currency = card.currency.as_ref()
            .map(|s| format!("'{}'", s))
            .unwrap_or("NULL".to_string());
        
        sql_lines.push(format!(
            "INSERT INTO {} (number, network, exp_month, exp_year, expiry, cvv, balance, currency, bin) VALUES ('{}', '{}', '{}', '{}', '{}', {}, {}, {}, '{}');",
            table_name,
            card.number,
            card.network,
            card.exp_month,
            card.exp_year,
            card.expiry,
            cvv,
            balance,
            currency,
            card.bin
        ));
    }
    
    sql_lines.join("\n")
}

/// 导出为 CARD 格式（卡片显示）
pub fn export_to_card(cards: &[CardData]) -> String {
    cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let mut lines = vec![
                format!("🔖 卡片 #{}", i + 1),
                format!("💳 卡号: {}", card.number),
                format!("🌐 网络: {}", card.network),
                format!("📅 过期: {}", card.expiry),
            ];
            
            if let Some(cvv) = &card.cvv {
                lines.push(format!("🔒 CVV: {}", cvv));
            }
            
            if let Some(balance) = card.balance {
                let currency = card.currency.as_ref().map(|s| s.as_str()).unwrap_or("USD");
                lines.push(format!("💰 余额: {} {}", balance, currency));
            }
            
            lines.push("─".repeat(40));
            lines.join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card_generator::CardData;
    
    fn create_test_card() -> CardData {
        CardData {
            number: "4111111111111111".to_string(),
            network: "Visa".to_string(),
            exp_month: "12".to_string(),
            exp_year: "2025".to_string(),
            expiry: "12/25".to_string(),
            cvv: Some("123".to_string()),
            balance: Some(1000.50),
            currency: Some("USD".to_string()),
            bin: "411111".to_string(),
        }
    }
    
    #[test]
    fn test_export_to_pipe() {
        let cards = vec![create_test_card()];
        let result = export_to_pipe(&cards);
        assert!(result.contains("4111111111111111|12/25|123"));
    }
    
    #[test]
    fn test_export_to_json() {
        let cards = vec![create_test_card()];
        let result = export_to_json(&cards);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("4111111111111111"));
    }
    
    #[test]
    fn test_export_to_csv() {
        let cards = vec![create_test_card()];
        let result = export_to_csv(&cards);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("number,network"));
    }
}


