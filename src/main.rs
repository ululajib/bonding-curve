use std::f64;

/// Fungsi untuk menghitung harga token berdasarkan bonding curve
/// supply: Jumlah token yang beredar saat ini
/// reserve_balance: Jumlah token yang ada di reserve
/// reserve_ratio: Rasio reserve (0-1)
pub fn calculate_token_price(supply: f64, reserve_balance: f64, reserve_ratio: f64) -> f64 {
    // Menggunakan rumus bonding curve: price = reserve_balance / (supply * reserve_ratio)
    if supply == 0.0 || reserve_ratio == 0.0 {
        return 0.0;
    }

    let price = reserve_balance / (supply * reserve_ratio);
    price
}

/// Fungsi untuk menghitung jumlah token yang bisa dibeli dengan sejumlah SOL
/// amount_in_sol: Jumlah SOL yang ingin digunakan untuk membeli token
/// current_supply: Supply token saat ini
/// reserve_balance: Jumlah token di reserve
/// reserve_ratio: Rasio reserve (0-1) 
pub fn calculate_purchase_amount(
    amount_in_sol: f64,
    current_supply: f64,
    reserve_balance: f64,
    reserve_ratio: f64,
) -> f64 {
    // Menggunakan rumus: tokens = current_supply * ((1 + amount_in_sol/reserve_balance)^reserve_ratio - 1)
    if amount_in_sol <= 0.0 {
        return 0.0;
    }

    let base = 1.0 + (amount_in_sol / reserve_balance);
    let exp = base.powf(reserve_ratio);
    let tokens = current_supply * (exp - 1.0);
    
    tokens
}

/// Fungsi untuk menghitung jumlah SOL yang akan diterima saat menjual token
/// tokens_to_sell: Jumlah token yang ingin dijual
/// current_supply: Supply token saat ini
/// reserve_balance: Jumlah token di reserve
/// reserve_ratio: Rasio reserve (0-1)
pub fn calculate_sale_return(
    tokens_to_sell: f64,
    current_supply: f64,
    reserve_balance: f64, 
    reserve_ratio: f64,
) -> f64 {
    // Menggunakan rumus: sol = reserve_balance * (1 - (1 - tokens_to_sell/current_supply)^(1/reserve_ratio))
    if tokens_to_sell <= 0.0 || tokens_to_sell > current_supply {
        return 0.0;
    }

    let base = 1.0 - (tokens_to_sell / current_supply);
    let exp = 1.0 / reserve_ratio;
    let sol = reserve_balance * (1.0 - base.powf(exp));
    
    sol
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const EPSILON: f64 = 0.000001; // Untuk perbandingan floating point
    
    #[test]
    fn test_calculate_token_price() {
        // Test case normal
        assert!((calculate_token_price(100.0, 50.0, 0.5) - 1.0).abs() < EPSILON);
        
        // Test case supply = 0
        assert_eq!(calculate_token_price(0.0, 50.0, 0.5), 0.0);
        
        // Test case reserve_ratio = 0
        assert_eq!(calculate_token_price(100.0, 50.0, 0.0), 0.0);
    }
    
    #[test]
    fn test_calculate_purchase_amount() {
        // Test case normal
        let tokens = calculate_purchase_amount(10.0, 100.0, 50.0, 0.5);
        assert!(tokens > 0.0);
        
        // Test case amount_in_sol = 0
        assert_eq!(calculate_purchase_amount(0.0, 100.0, 50.0, 0.5), 0.0);
        
        // Test case amount_in_sol negative
        assert_eq!(calculate_purchase_amount(-10.0, 100.0, 50.0, 0.5), 0.0);
    }
    
    #[test]
    fn test_calculate_sale_return() {
        // Test case normal
        let sol = calculate_sale_return(10.0, 100.0, 50.0, 0.5);
        assert!(sol > 0.0);
        
        // Test case tokens_to_sell = 0
        assert_eq!(calculate_sale_return(0.0, 100.0, 50.0, 0.5), 0.0);
        
        // Test case tokens_to_sell > current_supply
        assert_eq!(calculate_sale_return(150.0, 100.0, 50.0, 0.5), 0.0);
        
        // Test case tokens_to_sell negative
        assert_eq!(calculate_sale_return(-10.0, 100.0, 50.0, 0.5), 0.0);
    }
}
