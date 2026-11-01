# 🚗 GIIAS Main Hall Simulator (`giias-main-hall`)

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Aplikasi simulasi pameran otomotif GIIAS interaktif berbasis navigasi denah grid 2D di terminal dengan sistem dealer kendaraan, belanja aksesoris, kasir, dan mode admin dinamis dalam **Rust**.

Dikonversi dan di-*overhaul* dari kode tugas C++ menjadi aplikasi berorientasi objek/data-driven yang modular.

---

## 🗺️ Denah Expo & Booth Dealer

Pengunjung dapat menavigasikan karakter (`P`) menggunakan tombol navigasi:
- `w` : Bergerak ke atas
- `s` : Bergerak ke bawah
- `a` : Bergerak ke kiri
- `d` : Bergerak ke kanan
- `c` : Memeriksa keranjang belanja
- `e` : Keluar dari hall pameran

### Booth yang Tersedia:
- **`H` (Honda)**: Accord, Civic RS, Brio RS.
- **`T` (Toyota)**: Yaris Cross, GR Yaris.
- **`B` (BYD)**: Seal, Atto 3, Dolphin, M6, Tang, Han.
- **`D` (Daihatsu)**: Ayla, Terios, Xenia, Sigra, Rocky, Sirion.
- **`A` (Audio & Aksesoris)**: Speaker High-End, DSP Processor, Power Amplifier.
- **`K` (Kasir)**: Melakukan checkout dan mencetak rincian struk belanja kendaraan.

---

## 🛠️ Mode Admin

Penyelenggara pameran dapat masuk ke **Admin Mode** untuk memperbarui harga unit kendaraan atau aksesoris secara *real-time*.

---

## 🚀 Cara Menjalankan

```bash
git clone https://github.com/ZengLLQ/giias-main-hall.git
cd giias-main-hall

# Jalankan unit tests
cargo test

# Jalankan simulasi
cargo run
```

---

## 📜 Lisensi

Proyek ini dilisensikan di bawah lisensi [MIT](LICENSE).  
Dibuat ulang oleh **Jonathan Cahyadi** (*Overhauled College Project*).
