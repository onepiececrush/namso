use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CardNetwork {
    pub name: &'static str,
    pub identifier: &'static str,
    pub bins: Vec<&'static str>,
    pub length: Vec<usize>,
    pub cvv_length: usize,
}

impl CardNetwork {
    #[allow(dead_code)]
    pub fn matches_bin(&self, bin: &str) -> bool {
        self.bins.iter().any(|&supported_bin| {
            if bin.len() < supported_bin.len() {
                false
            } else if bin.len() == supported_bin.len() {
                bin == supported_bin
            } else {
                bin.starts_with(supported_bin)
            }
        })
    }

    #[allow(dead_code)]
    pub fn is_valid_length(&self, length: usize) -> bool {
        self.length.contains(&length)
    }
}

pub struct CardNetworkRegistry;

impl CardNetworkRegistry {
    pub fn get_all_networks() -> HashMap<&'static str, CardNetwork> {
        let mut networks = HashMap::new();

        networks.insert("visa", CardNetwork {
            name: "Visa",
            identifier: "visa",
            bins: vec!["4"],
            length: vec![13, 16, 19],
            cvv_length: 3,
        });

        networks.insert("mastercard", CardNetwork {
            name: "Mastercard",
            identifier: "mastercard",
            bins: vec!["51", "52", "53", "54", "55"],
            length: vec![16],
            cvv_length: 3,
        });

        networks.insert("amex", CardNetwork {
            name: "American Express",
            identifier: "amex",
            bins: vec!["34", "37"],
            length: vec![15],
            cvv_length: 3, // Changed from 4 to 3 as requested
        });

        networks.insert("discover", CardNetwork {
            name: "Discover",
            identifier: "discover",
            bins: vec!["6011", "644", "645", "646", "647", "648", "649", "65"],
            length: vec![16],
            cvv_length: 3,
        });

        networks.insert("unionpay", CardNetwork {
            name: "UnionPay",
            identifier: "unionpay",
            bins: vec!["62"],
            length: vec![16, 17, 18, 19],
            cvv_length: 3,
        });

        networks.insert("diners", CardNetwork {
            name: "Diners Club",
            identifier: "diners",
            bins: vec!["300", "301", "302", "303", "304", "305", "36", "38"],
            length: vec![14, 16],
            cvv_length: 3,
        });

        networks
    }

    pub fn detect_network(card_number: &str) -> Option<String> {
        let networks = Self::get_all_networks();

        for (_, network) in networks {
            // Check if any BIN matches
            if network.bins.iter().any(|bin| {
                card_number.starts_with(bin) && network.is_valid_length(card_number.len())
            }) {
                return Some(network.name.to_string());
            }

            // Special handling for Mastercard's extended range (2221-2720)
            if network.identifier == "mastercard" && card_number.len() == 16 {
                if let Ok(first_four) = card_number[0..4].parse::<u32>() {
                    if (2221..=2720).contains(&first_four) {
                        return Some(network.name.to_string());
                    }
                }
            }

            // Special handling for Discover's range (644-649)
            if network.identifier == "discover" && card_number.len() == 16 {
                if card_number.len() >= 3 {
                    if let Ok(first_three) = card_number[0..3].parse::<u32>() {
                        if (644..=649).contains(&first_three) {
                            return Some(network.name.to_string());
                        }
                    }
                }
            }
        }

        None
    }

    pub fn get_network_by_identifier(identifier: &str) -> Option<CardNetwork> {
        let networks = Self::get_all_networks();
        networks.get(identifier).cloned()
    }

    pub fn get_network_names() -> Vec<(&'static str, &'static str)> {
        vec![
            ("random", "Random"),
            ("visa", "Visa"),
            ("mastercard", "Mastercard"),
            ("amex", "American Express"),
            ("discover", "Discover"),
            ("unionpay", "UnionPay"),
            ("diners", "Diners Club"),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_visa() {
        assert_eq!(CardNetworkRegistry::detect_network("4111111111111111"), Some("Visa".to_string()));
        assert_eq!(CardNetworkRegistry::detect_network("4111111111111"), Some("Visa".to_string()));
        assert_eq!(CardNetworkRegistry::detect_network("4111111111111111111"), Some("Visa".to_string()));
    }

    #[test]
    fn test_detect_mastercard() {
        assert_eq!(CardNetworkRegistry::detect_network("5555555555554444"), Some("Mastercard".to_string()));
        assert_eq!(CardNetworkRegistry::detect_network("2221000000000000"), Some("Mastercard".to_string()));
    }

    #[test]
    fn test_detect_amex() {
        assert_eq!(CardNetworkRegistry::detect_network("378282246310005"), Some("American Express".to_string()));
        assert_eq!(CardNetworkRegistry::detect_network("371449635398431"), Some("American Express".to_string()));
    }

    #[test]
    fn test_detect_invalid() {
        assert_eq!(CardNetworkRegistry::detect_network("1234567890123456"), None);
    }
}