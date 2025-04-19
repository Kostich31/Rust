#![allow(dead_code)]
pub struct User {
    name: String,
    age: u32,
    height: f32,
    visit_count: u32,
    last_blood_pressure: Option<(u32, u32)>,
}

pub struct Measurements {
    height: f32,
    blood_pressure: (u32, u32),
}

pub struct HealthReport<'a> {
    patient_name: &'a str,
    visit_count: u32,
    height_change: f32,
    blood_pressure_change: Option<(i32, i32)>,
}

impl User {
    pub fn new(name: String, age: u32, height: f32) -> Self {
        Self { name, age, height, visit_count: 0, last_blood_pressure: None }
    }
    pub fn visit_doctor(&mut self, measurements: Measurements) -> HealthReport {
        // Увеличиваем счетчик визитов
        self.visit_count += 1;
        
        // Сохраняем предыдущий рост и обновляем текущий
        let old_height = self.height;
        self.height = measurements.height;
        
        // Вычисляем изменение роста
        let height_change = measurements.height - old_height;
        
        // Обрабатываем кровяное давление
        let blood_pressure_change = match self.last_blood_pressure {
            // Если есть предыдущее значение - считаем разницу
            Some((prev_sys, prev_dia)) => {
                let curr_sys = measurements.blood_pressure.0 as i32;
                let curr_dia = measurements.blood_pressure.1 as i32;
                Some((curr_sys - prev_sys as i32, curr_dia - prev_dia as i32))
            }
            // Для первого визита изменения нет
            None => None,
        };
        
        // Обновляем последнее давление
        self.last_blood_pressure = Some(measurements.blood_pressure);
        
        // Возвращаем отчет
        HealthReport {
            patient_name: &self.name,
            visit_count: self.visit_count,
            height_change,
            blood_pressure_change,
        }
    }
}

#[test]
fn test_visit() {
    let mut bob = User::new(String::from("Гиппократ"), 32, 155.2);
    assert_eq!(bob.visit_count, 0);
    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (120, 80) });
    assert_eq!(report.patient_name, "Гиппократ");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);
    assert!((report.height_change - 0.9).abs() < 0.00001);

    let report =
        bob.visit_doctor(Measurements { height: 156.1, blood_pressure: (115, 76) });

    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-5, -4)));
    assert_eq!(report.height_change, 0.0);
}
