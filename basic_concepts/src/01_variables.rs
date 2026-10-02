fn main() {
    // متغیر غیرقابل تغییر (Immutable)
    let name = "Rust";
    let version: f64 = 1.80; // عدد اعشاری
    
    println!("زبان {} نسخه {}", name, version);
    
    // متغیر قابل تغییر (Mutable)
    let mut counter = 0;
    println!("شمارنده اولیه: {}", counter);
    
    counter = counter + 1;
    println!("شمارنده بعد از افزایش: {}", counter);
    
    // انواع داده مختلف
    let is_rust_fun: bool = true;
    let max_value: u32 = 4294967295; // عدد صحیح بدون علامت ۳۲ بیتی
    let temperature: i16 = -40; // عدد صحیح با علامت ۱۶ بیتی (مناسب برای دما)
    
    println!("آیا Rust جالب است؟ {}", is_rust_fun);
    println!("حداکثر مقدار: {}", max_value);
    println!("دما: {} درجه", temperature);
}