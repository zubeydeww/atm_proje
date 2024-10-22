use std::io;

fn main() {
    let mut bakiye = 1000.0; // Başlangıç bakiyesi
    loop {
        println!("\nATM'ye Hoş Geldiniz!");
        println!("Bakiyeniz: {:.2} TL", bakiye);
        println!("Yapmak istediğiniz işlemi seçin:");
        println!("1. Para Yatırma");
        println!("2. Para Çekme");
        println!("3. Bakiye Sorgulama");
        println!("4. Çıkış");

        let mut secim = String::new();
        io::stdin().read_line(&mut secim).expect("Hata: Girdi okunamadı");
        let secim: u32 = secim.trim().parse().expect("Lütfen geçerli bir sayı girin.");

        match secim {
            1 => {
                println!("Yatırmak istediğiniz tutarı girin:");
                let mut yatir = String::new();
                io::stdin().read_line(&mut yatir).expect("Hata: Girdi okunamadı");
                let yatir: f64 = yatir.trim().parse().expect("Lütfen geçerli bir sayı girin.");
                bakiye += yatir;
                println!("Başarıyla {:.2} TL yatırdınız.", yatir);
            },
            2 => {
                println!("Çekmek istediğiniz tutarı girin:");
                let mut cek = String::new();
                io::stdin().read_line(&mut cek).expect("Hata: Girdi okunamadı");
                let cek: f64 = cek.trim().parse().expect("Lütfen geçerli bir sayı girin.");
                
                if cek > bakiye {
                    println!("Hata: Yetersiz bakiye.");
                } else {
                    bakiye -= cek;
                    println!("Başarıyla {:.2} TL çektiniz.", cek);
                }
            },
            3 => {
                println!("Bakiyeniz: {:.2} TL", bakiye);
            },
            4 => {
                println!("ATM'den çıkış yapıyorsunuz.");
                break;
            },
            _ => {
                println!("Hata: Geçersiz işlem seçimi.");
            }
        }
    }
}