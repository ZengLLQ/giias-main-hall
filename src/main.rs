//! # GIIAS Main Hall Simulator
//!
//! Simulasi interaktif pameran otomotif GIIAS berbasis denah grid 2D:
//! - Navigasi pergerakan pengunjung menggunakan tombol `w`, `a`, `s`, `d`
//! - Booth Dealer Mobil: Honda, Toyota, BYD, Daihatsu
//! - Booth Audio & Aksesoris Mobil
//! - Kasir (K) untuk checkout keranjang belanja
//! - Mode Admin untuk mengubah harga katalog kendaraan & aksesoris secara dinamis

use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct Product {
    pub name: String,
    pub price: i64,
}

#[derive(Debug, Clone)]
pub struct Catalog {
    pub honda: Vec<Product>,
    pub toyota: Vec<Product>,
    pub byd: Vec<Product>,
    pub daihatsu: Vec<Product>,
    pub aksesoris: Vec<Product>,
}

impl Default for Catalog {
    fn default() -> Self {
        Self {
            honda: vec![
                Product { name: "Honda Accord".into(), price: 800_000_000 },
                Product { name: "Honda Civic RS".into(), price: 600_000_000 },
                Product { name: "Honda Brio RS".into(), price: 250_000_000 },
            ],
            toyota: vec![
                Product { name: "Toyota Yaris Cross".into(), price: 400_000_000 },
                Product { name: "Toyota GR Yaris".into(), price: 850_000_000 },
            ],
            byd: vec![
                Product { name: "BYD Seal".into(), price: 130_000_000 },
                Product { name: "BYD Atto 3".into(), price: 250_000_000 },
                Product { name: "BYD Dolphin".into(), price: 150_000_000 },
                Product { name: "BYD M6".into(), price: 379_000_000 },
                Product { name: "BYD Tang".into(), price: 800_000_000 },
                Product { name: "BYD Han".into(), price: 850_000_000 },
            ],
            daihatsu: vec![
                Product { name: "Daihatsu Ayla".into(), price: 130_000_000 },
                Product { name: "Daihatsu Terios".into(), price: 250_000_000 },
                Product { name: "Daihatsu Xenia".into(), price: 220_000_000 },
                Product { name: "Daihatsu Sigra".into(), price: 150_000_000 },
                Product { name: "Daihatsu Rocky".into(), price: 230_000_000 },
                Product { name: "Daihatsu Sirion".into(), price: 240_000_000 },
            ],
            aksesoris: vec![
                Product { name: "Speaker High-End".into(), price: 100_000_000 },
                Product { name: "DSP Processor".into(), price: 4_500_000 },
                Product { name: "Power Amplifier".into(), price: 5_000_000 },
            ],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CartItem {
    pub name: String,
    pub price: i64,
    pub qty: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Cart {
    pub items: Vec<CartItem>,
}

impl Cart {
    pub fn add_item(&mut self, name: &str, price: i64, qty: u32) {
        if let Some(existing) = self.items.iter_mut().find(|i| i.name == name) {
            existing.qty += qty;
        } else {
            self.items.push(CartItem {
                name: name.to_string(),
                price,
                qty,
            });
        }
    }

    pub fn total(&self) -> i64 {
        self.items.iter().map(|i| i.price * i.qty as i64).sum()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}

pub fn format_rupiah(amount: i64) -> String {
    let s = amount.to_string();
    let mut result = String::new();
    let mut count = 0;
    for ch in s.chars().rev() {
        if count == 3 {
            result.push('.');
            count = 0;
        }
        result.push(ch);
        count += 1;
    }
    result.chars().rev().collect()
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap_or_default();
    buffer.trim().to_string()
}

fn handle_booth(booth_name: &str, products: &[Product], cart: &mut Cart) {
    loop {
        println!("\n+-------------------------+");
        println!("| Booth {:<17} |", booth_name);
        println!("+-------------------------+");
        for (i, p) in products.iter().enumerate() {
            println!("{}. {:<20} : Rp {}", i + 1, p.name, format_rupiah(p.price));
        }
        println!("0. Kembali ke Denah Expo");

        let input = read_line("Pilih kendaraan/barang untuk dibeli: ");
        if input == "0" {
            break;
        }

        if let Ok(idx) = input.parse::<usize>() {
            if idx >= 1 && idx <= products.len() {
                let chosen = &products[idx - 1];
                let qty_str = read_line("Jumlah yang ingin dibeli: ");
                let qty = qty_str.parse::<u32>().unwrap_or(1).max(1);

                cart.add_item(&chosen.name, chosen.price, qty);
                println!(
                    "[Sukses] {}x {} berhasil dimasukkan ke keranjang!",
                    qty, chosen.name
                );
                continue;
            }
        }
        println!("[Error] Pilihan tidak valid!");
    }
}

fn handle_kasir(cart: &mut Cart) {
    println!("\n+-----------------------------+");
    println!("|          KASIR EXPO         |");
    println!("+-----------------------------+");
    if cart.items.is_empty() {
        println!("Keranjang belanja kamu masih kosong!");
        return;
    }

    for (i, item) in cart.items.iter().enumerate() {
        println!(
            "{}. {:<20} x{} = Rp {}",
            i + 1,
            item.name,
            item.qty,
            format_rupiah(item.price * item.qty as i64)
        );
    }
    println!("-------------------------------");
    println!("Total Pembayaran : Rp {}", format_rupiah(cart.total()));
    println!("-------------------------------");
    let confirm = read_line("Bayar dan checkout sekarang? (y/n): ");
    if confirm.to_lowercase() == "y" {
        println!("Pembayaran Berhasil! Terima kasih telah berbelanja di GIIAS!");
        cart.clear();
    }
}

fn show_cart(cart: &Cart) {
    println!("\n+-----------------------------+");
    println!("|       KERANJANG BELANJA     |");
    println!("+-----------------------------+");
    if cart.items.is_empty() {
        println!("Keranjang kosong.");
    } else {
        for (i, item) in cart.items.iter().enumerate() {
            println!(
                "{}. {:<20} ({} unit) : Rp {}",
                i + 1,
                item.name,
                item.qty,
                format_rupiah(item.price * item.qty as i64)
            );
        }
        println!("Total: Rp {}", format_rupiah(cart.total()));
    }
}

fn run_user_expo(catalog: &Catalog, cart: &mut Cart) {
    let mut px: usize = 1;
    let mut py: usize = 1;

    // Peta ukuran 5 baris x 7 kolom
    // Legenda:
    // H: Honda (1, 1)
    // T: Toyota (1, 3)
    // B: BYD (1, 5)
    // D: Daihatsu (3, 1)
    // A: Aksesoris (3, 3)
    // K: Kasir (3, 5)
    // . : Jalan

    loop {
        println!("\n+---------------------------------------+");
        println!("|           GIIAS MAIN HALL MAP         |");
        println!("+---------------------------------------+");
        println!("|  H = Honda        T = Toyota  B = BYD |");
        println!("|  D = Daihatsu     A = Audio   K = Kasir|");
        println!("|  P = Posisi Kamu                      |");
        println!("+---------------------------------------+");

        for y in 0..5 {
            print!("  |");
            for x in 0..7 {
                if x == px && y == py {
                    print!(" P ");
                } else if x == 1 && y == 1 {
                    print!(" H ");
                } else if x == 3 && y == 1 {
                    print!(" T ");
                } else if x == 5 && y == 1 {
                    print!(" B ");
                } else if x == 1 && y == 3 {
                    print!(" D ");
                } else if x == 3 && y == 3 {
                    print!(" A ");
                } else if x == 5 && y == 3 {
                    print!(" K ");
                } else {
                    print!(" . ");
                }
            }
            println!("|");
        }
        println!("+---------------------------------------+");
        println!("Kontrol: [w] Atas  [s] Bawah  [a] Kiri  [d] Kanan");
        println!("Aksi:    [c] Cek Keranjang  [e] Keluar Expo");

        let cmd = read_line(">> ");
        match cmd.to_lowercase().as_str() {
            "w" => {
                if py > 0 {
                    py -= 1;
                }
            }
            "s" => {
                if py < 4 {
                    py += 1;
                }
            }
            "a" => {
                if px > 0 {
                    px -= 1;
                }
            }
            "d" => {
                if px < 6 {
                    px += 1;
                }
            }
            "c" => show_cart(cart),
            "e" => break,
            _ => {}
        }

        // Cek interaksi booth
        if px == 1 && py == 1 {
            handle_booth("Honda", &catalog.honda, cart);
        } else if px == 3 && py == 1 {
            handle_booth("Toyota", &catalog.toyota, cart);
        } else if px == 5 && py == 1 {
            handle_booth("BYD", &catalog.byd, cart);
        } else if px == 1 && py == 3 {
            handle_booth("Daihatsu", &catalog.daihatsu, cart);
        } else if px == 3 && py == 3 {
            handle_booth("Audio & Aksesoris", &catalog.aksesoris, cart);
        } else if px == 5 && py == 3 {
            handle_kasir(cart);
        }
    }
}

fn edit_booth_prices(booth_name: &str, products: &mut [Product]) {
    loop {
        println!("\n=== Admin Edit: {} ===", booth_name);
        for (i, p) in products.iter().enumerate() {
            println!("{}. {:<20} : Rp {}", i + 1, p.name, format_rupiah(p.price));
        }
        println!("0. Selesai");

        let choice = read_line("Pilih nomor produk untuk ubah harga: ");
        if choice == "0" {
            break;
        }

        if let Ok(idx) = choice.parse::<usize>() {
            if idx >= 1 && idx <= products.len() {
                let new_price_str = read_line(&format!(
                    "Masukkan harga baru untuk {}: ",
                    products[idx - 1].name
                ));
                if let Ok(new_price) = new_price_str.parse::<i64>() {
                    products[idx - 1].price = new_price;
                    println!("[Sukses] Harga berhasil diubah!");
                    continue;
                }
            }
        }
        println!("[Error] Masukan tidak valid!");
    }
}

fn run_admin_mode(catalog: &mut Catalog) {
    loop {
        println!("\n=== GIIAS MAIN HALL ADMIN MODE ===");
        println!("1. Edit Booth Honda");
        println!("2. Edit Booth Toyota");
        println!("3. Edit Booth BYD");
        println!("4. Edit Booth Daihatsu");
        println!("5. Edit Booth Aksesoris");
        println!("0. Kembali ke Menu Utama");

        let choice = read_line("Pilih menu: ");
        match choice.as_str() {
            "1" => edit_booth_prices("Honda", &mut catalog.honda),
            "2" => edit_booth_prices("Toyota", &mut catalog.toyota),
            "3" => edit_booth_prices("BYD", &mut catalog.byd),
            "4" => edit_booth_prices("Daihatsu", &mut catalog.daihatsu),
            "5" => edit_booth_prices("Aksesoris", &mut catalog.aksesoris),
            "0" => break,
            _ => println!("[Error] Pilihan tidak valid!"),
        }
    }
}

fn main() {
    let mut catalog = Catalog::default();
    let mut cart = Cart::default();

    loop {
        println!("+----------------------+");
        println!("|      GIIAS EXPO      |");
        println!("+----------------------+");
        println!("1. Masuk Expo (User)");
        println!("2. Admin (Ubah Harga)");
        println!("0. Exit");
        println!("+----------------------+");

        let menu = read_line(">> ");
        match menu.as_str() {
            "1" => run_user_expo(&catalog, &mut cart),
            "2" => run_admin_mode(&mut catalog),
            "0" => {
                println!("Sampai jumpa di gelaran GIIAS berikutnya!");
                break;
            }
            _ => println!("[Error] Pilihan tidak valid!\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cart_calculation() {
        let mut cart = Cart::default();
        cart.add_item("Honda Brio RS", 250_000_000, 2);
        cart.add_item("Speaker", 100_000_000, 1);

        assert_eq!(cart.total(), 600_000_000);
    }

    #[test]
    fn test_catalog_admin_price_update() {
        let mut catalog = Catalog::default();
        catalog.honda[0].price = 750_000_000;
        assert_eq!(catalog.honda[0].price, 750_000_000);
    }
}
