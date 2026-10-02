// یک مثال ساده از کنترل‌کننده تناسبی (P Controller)
fn proportional_controller(setpoint: f64, current_value: f64, kp: f64) -> f64 {
    let error = setpoint - current_value;
    let control_output = kp * error;
    control_output
}

fn main() {
    let setpoint = 100.0; // دمای هدف
    let kp = 0.5; // ضریب تناسبی
    
    println!("شبیه‌سازی کنترل‌کننده دما");
    println!("========================\n");
    
    // شبیه‌سازی چند گام زمانی
    let mut current_temp = 25.0;
    
    for step in 1..=10 {
        let control_output = proportional_controller(setpoint, current_temp, kp);
        
        // شبیه‌سازی تأثیر کنترل‌کننده بر دما
        current_temp += control_output * 0.1;
        
        println!("گام {}: دما = {:.2}°C, خطا = {:.2}, خروجی کنترل = {:.2}", 
                 step, current_temp, setpoint - current_temp, control_output);
    }
}