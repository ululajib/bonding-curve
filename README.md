# Implementasi Bonding Curve di Rust

Repositori ini berisi implementasi bonding curve dalam bahasa Rust. Bonding curve adalah mekanisme penentuan harga token yang dinamis berdasarkan supply dan demand.

## Fitur

- Perhitungan harga token berdasarkan supply saat ini
- Kalkulasi jumlah token yang bisa dibeli dengan sejumlah SOL
- Kalkulasi jumlah SOL yang akan diterima saat menjual token
- Menggunakan formula bonding curve dengan reserve ratio

## Fungsi Utama

### calculate_token_price
Menghitung harga token berdasarkan supply saat ini, reserve balance, dan reserve ratio.

### calculate_purchase_amount  
Menghitung jumlah token yang bisa dibeli dengan sejumlah SOL berdasarkan supply dan reserve saat ini.

### calculate_sale_return
Menghitung jumlah SOL yang akan diterima saat menjual token berdasarkan supply dan reserve saat ini.

## Penggunaan

Semua fungsi menerima parameter dalam bentuk floating point (f64) dan mengembalikan hasil dalam format yang sama.


