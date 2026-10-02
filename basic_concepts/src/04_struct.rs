
// تعریف یک ساختار برای سنسور
struct Sensor {
    id: u32,
    name: String,
    value: f64,
    is_active: bool,
}

// پیاده‌سازی متدها برای ساختار
impl Sensor {
    // متد سازنده
    fn new(id: u32, name: &str, value: f64) -> Sensor {
        Sensor {
            id: id,
            name: String::from(name),
            value: value,
            is_active: true,
        }
    }
    
    // متد برای نمایش اطلاعات
    fn display_info(&self) {
        println!("سنسور [{}]: {} = {:.2} (فعال: {})", 
                 self.id, self.name, self.value, self.is_active);
    }
    
    // متد برای به‌روزرسانی مقدار
    fn update_value(&mut self, new_value: f64) {
        self.value = new_value;
    }
}

fn main() {
    // ایجاد نمونه‌هایی از سنسور
    let mut temp_sensor = Sensor::new(1, "دما", 25.5);
    let mut pressure_sensor = Sensor::new(2, "فشار", 101.3);
    
    temp_sensor.display_info();
    pressure_sensor.display_info();
    
    // به‌روزرسانی مقادیر
    temp_sensor.update_value(26.8);
    pressure_sensor.update_value(101.5);
    
    println!("\nپس از به‌روزرسانی:");
    temp_sensor.display_info();
    pressure_sensor.display_info();
}