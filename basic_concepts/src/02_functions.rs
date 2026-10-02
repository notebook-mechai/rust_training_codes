// تعریف یک تابع که دو عدد را جمع می‌کند
fn add(a: i32, b: i32) -> i32 {
    a + b // در Rust، آخرین عبارت بدون سمی‌کالن، مقدار بازگشتی است
}

// تابعی که بدون مقدار بازگشتی است
fn greet(name: &str) {
    println!("سلام {}! به دنیای Rust خوش آمدی.", name);
}

// تابعی که شرط دارد
fn check_temperature(temp: f64) {
    if temp < 0.0 {
        println!("هوا سرد است: {} درجه", temp);
    } else if temp > 30.0 {
        println!("هوا گرم است: {} درجه", temp);
    } else {
        println!("هوا معتدل است: {} درجه", temp);
    }
}

fn main() {
    let result = add(10, 20);
    println!("نتیجه جمع: {}", result);
    
    greet("مهندس");
    
    check_temperature(25.5);
    check_temperature(-5.0);
    check_temperature(35.0);
}