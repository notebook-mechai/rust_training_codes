fn main() {
    // آرایه با اندازه ثابت
    let sensor_readings: [f64; 5] = [23.5, 24.1, 22.8, 25.0, 23.9];
    
    println!("خوانش‌های سنسور:");
    for (index, &reading) in sensor_readings.iter().enumerate() {
        println!("سنسور {}: {} درجه", index + 1, reading);
    }
    
    // بردار (Vector) - آرایه با اندازه متغیر
    let mut temperatures = vec![20.0, 21.5, 22.0];
    
    println!("\nدماهای اولیه: {:?}", temperatures);
    
    temperatures.push(23.5); // اضافه کردن عنصر جدید
    temperatures.push(24.0);
    
    println!("دماهای بعد از اضافه کردن: {:?}", temperatures);
    
    // محاسبه میانگین
    let sum: f64 = temperatures.iter().sum();
    let average = sum / temperatures.len() as f64;
    println!("میانگین دما: {:.2} درجه", average);
}