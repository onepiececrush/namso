use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FakeUser {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

const FIRST_NAMES: &[&str] = &["张", "李", "王", "刘", "陈", "杨", "赵", "黄", "周", "吴", "徐", "孙", "马", "朱", "胡"];
const MIDDLE_NAMES: &[&str] = &["伟", "芳", "娜", "敏", "静", "丽", "强", "军", "勇", "艳", "杰", "涛", "明", "超", "磊"];
const LAST_NAMES: &[&str] = &["华", "峰", "鹏", "辉", "刚", "平", "波", "东", "文", "斌", "宇", "洋", "飞", "龙", "浩"];
const CITIES: &[&str] = &["北京", "上海", "广州", "深圳", "杭州", "南京", "成都", "武汉", "重庆", "西安"];
const DISTRICTS: &[&str] = &["朝阳", "海淀", "西城", "东城", "浦东", "黄浦", "徐汇", "静安", "天河", "越秀"];
const EMAIL_DOMAINS: &[&str] = &["gmail.com", "163.com", "qq.com", "outlook.com", "example.com", "test.com"];

/// 生成虚假用户数据
pub fn generate_fake_users(count: usize) -> Vec<FakeUser> {
    let mut rng = rand::thread_rng();
    let mut users = Vec::new();
    
    for _ in 0..count {
        // 生成姓名
        let first = FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())];
        let middle = MIDDLE_NAMES[rng.gen_range(0..MIDDLE_NAMES.len())];
        let last = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())];
        let name = format!("{}{}{}", first, middle, last);
        
        // 生成邮箱
        let name_pinyin = romanize(&name);
        let random_num: u32 = rng.gen_range(100..999);
        let domain = EMAIL_DOMAINS[rng.gen_range(0..EMAIL_DOMAINS.len())];
        let email = format!("{}{}@{}", name_pinyin, random_num, domain);
        
        // 生成电话
        let phone = format!("1{}{}",
            rng.gen_range(3..9),
            (0..9).map(|_| rng.gen_range(0..10)).map(|d| d.to_string()).collect::<String>()
        );
        
        // 生成地址
        let city = CITIES[rng.gen_range(0..CITIES.len())];
        let district = DISTRICTS[rng.gen_range(0..DISTRICTS.len())];
        let street_num: u32 = rng.gen_range(1..999);
        let address = format!("{}市{}区{}号", city, district, street_num);
        
        users.push(FakeUser {
            name,
            email: email.to_lowercase(),
            phone,
            address,
        });
    }
    
    users
}

/// 简单的汉字转拼音（只用于姓名，非常简化版本）
fn romanize(name: &str) -> String {
    // 这是一个简化版本，实际应用中应该使用专门的拼音库
    let mut result = String::new();
    
    for c in name.chars() {
        let pinyin = match c {
            '张' => "zhang", '李' => "li", '王' => "wang", '刘' => "liu", '陈' => "chen",
            '杨' => "yang", '赵' => "zhao", '黄' => "huang", '周' => "zhou", '吴' => "wu",
            '徐' => "xu", '孙' => "sun", '马' => "ma", '朱' => "zhu", '胡' => "hu",
            '伟' => "wei", '芳' => "fang", '娜' => "na", '敏' => "min", '静' => "jing",
            '丽' => "li", '强' => "qiang", '军' => "jun", '勇' => "yong", '艳' => "yan",
            '杰' => "jie", '涛' => "tao", '明' => "ming", '超' => "chao", '磊' => "lei",
            '华' => "hua", '峰' => "feng", '鹏' => "peng", '辉' => "hui", '刚' => "gang",
            '平' => "ping", '波' => "bo", '东' => "dong", '文' => "wen", '斌' => "bin",
            '宇' => "yu", '洋' => "yang", '飞' => "fei", '龙' => "long", '浩' => "hao",
            _ => "x",
        };
        result.push_str(pinyin);
    }
    
    result
}

/// 生成 Lorem Ipsum 文本
pub fn generate_lorem_ipsum(paragraphs: usize) -> String {
    let lorem_words = vec![
        "Lorem", "ipsum", "dolor", "sit", "amet", "consectetur", "adipiscing", "elit",
        "sed", "do", "eiusmod", "tempor", "incididunt", "ut", "labore", "et", "dolore",
        "magna", "aliqua", "Ut", "enim", "ad", "minim", "veniam", "quis", "nostrud",
        "exercitation", "ullamco", "laboris", "nisi", "ut", "aliquip", "ex", "ea",
        "commodo", "consequat", "Duis", "aute", "irure", "dolor", "in", "reprehenderit",
        "in", "voluptate", "velit", "esse", "cillum", "dolore", "eu", "fugiat", "nulla",
        "pariatur", "Excepteur", "sint", "occaecat", "cupidatat", "non", "proident",
        "sunt", "in", "culpa", "qui", "officia", "deserunt", "mollit", "anim", "id",
        "est", "laborum"
    ];
    
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();
    
    for _ in 0..paragraphs {
        let word_count = rng.gen_range(50..100);
        let mut paragraph = Vec::new();
        
        for i in 0..word_count {
            let word = lorem_words[rng.gen_range(0..lorem_words.len())];
            if i == 0 {
                // 首字母大写
                paragraph.push(word.to_string());
            } else {
                paragraph.push(word.to_lowercase());
            }
        }
        
        result.push(format!("{}.", paragraph.join(" ")));
    }
    
    result.join("\n\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_fake_users() {
        let users = generate_fake_users(5);
        assert_eq!(users.len(), 5);
        
        for user in users {
            assert!(!user.name.is_empty());
            assert!(user.email.contains('@'));
            assert!(user.phone.starts_with('1'));
            assert!(user.phone.len() == 11);
            assert!(user.address.contains('市'));
        }
    }
    
    #[test]
    fn test_generate_lorem_ipsum() {
        let text = generate_lorem_ipsum(3);
        assert!(!text.is_empty());
        
        let paragraphs: Vec<&str> = text.split("\n\n").collect();
        assert_eq!(paragraphs.len(), 3);
    }
    
    #[test]
    fn test_romanize() {
        assert_eq!(romanize("张伟"), "zhangwei");
        assert_eq!(romanize("李娜"), "lina");
    }
}


