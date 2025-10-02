use crate::card_generator::{CardData, CreditCardGenerator};
use crate::exporters::{export_to_card, export_to_csv, export_to_json, export_to_pipe, export_to_sql, export_to_xml};
use crate::fake_data::{generate_fake_users, generate_lorem_ipsum, FakeUser};
use crate::networks::CardNetworkRegistry;
use crate::validator::{validate_card_number, ValidationResult};
use tauri::State;
use std::sync::Mutex;

pub struct AppState {
    pub generator: Mutex<CreditCardGenerator>,
}

/// 生成信用卡数据
#[tauri::command(rename_all = "snake_case")]
pub async fn generate_cards(
    network: String,
    quantity: usize,
    exp_month: Option<u32>,
    exp_year: Option<i32>,
    include_cvv: bool,
    include_balance: bool,
    currency: Option<String>,
    bin_code: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<CardData>, String> {
    let generator = state.generator.lock().map_err(|e| e.to_string())?;

    generator.generate_card_data(
        &network,
        quantity,
        exp_month,
        exp_year,
        include_cvv,
        include_balance,
        currency,
        bin_code,
    )
}

/// 验证信用卡号码
#[tauri::command]
pub async fn validate_card(card_number: String) -> Result<ValidationResult, String> {
    Ok(validate_card_number(&card_number))
}

/// 导出为指定格式
#[tauri::command]
pub async fn export_cards(
    cards: Vec<CardData>,
    format: String,
) -> Result<String, String> {
    match format.to_uppercase().as_str() {
        "PIPE" => Ok(export_to_pipe(&cards)),
        "CSV" => export_to_csv(&cards),
        "JSON" => export_to_json(&cards),
        "XML" => export_to_xml(&cards),
        "SQL" => Ok(export_to_sql(&cards, "test_cards")),
        "CARD" => Ok(export_to_card(&cards)),
        _ => Err(format!("不支持的导出格式: {}", format)),
    }
}

/// 生成虚假用户数据
#[tauri::command]
pub async fn generate_users(count: usize) -> Result<Vec<FakeUser>, String> {
    if count > 100 {
        return Err("最多生成 100 个用户".to_string());
    }
    Ok(generate_fake_users(count))
}

/// 生成 Lorem Ipsum 文本
#[tauri::command]
pub async fn generate_lorem(paragraphs: usize) -> Result<String, String> {
    if paragraphs > 50 {
        return Err("最多生成 50 段".to_string());
    }
    Ok(generate_lorem_ipsum(paragraphs))
}

/// 获取支持的货币列表
#[tauri::command]
pub async fn get_currencies() -> Result<Vec<(String, String)>, String> {
    Ok(vec![
        ("USD".to_string(), "United States Dollar".to_string()),
        ("PHP".to_string(), "Philippine Peso".to_string()),
        ("EUR".to_string(), "Euro".to_string()),
        ("JPY".to_string(), "Japanese Yen".to_string()),
        ("GBP".to_string(), "British Pound Sterling".to_string()),
        ("CHF".to_string(), "Swiss Franc".to_string()),
        ("CAD".to_string(), "Canadian Dollar".to_string()),
        ("AUD".to_string(), "Australian Dollar".to_string()),
        ("CNY".to_string(), "Chinese Yuan Renminbi".to_string()),
        ("INR".to_string(), "Indian Rupee".to_string()),
        ("BRL".to_string(), "Brazilian Real".to_string()),
        ("ZAR".to_string(), "South African Rand".to_string()),
        ("RUB".to_string(), "Russian Ruble".to_string()),
        ("SAR".to_string(), "Saudi Riyal".to_string()),
        ("SGD".to_string(), "Singapore Dollar".to_string()),
        ("MXN".to_string(), "Mexican Peso".to_string()),
    ])
}

/// 获取支持的卡网络列表
#[tauri::command]
pub async fn get_networks() -> Result<Vec<(String, String)>, String> {
    Ok(CardNetworkRegistry::get_network_names()
        .into_iter()
        .map(|(id, name)| (id.to_string(), name.to_string()))
        .collect())
}


