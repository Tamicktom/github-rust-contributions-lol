// Escrevendo documentação 📚
// Data: 11/08/1981
// Commit #1

use std::collections::HashMap;

#[derive(Debug)]
struct DailyActivity {
    date: String,
    activity: String,
    completed: bool,
}

impl DailyActivity {
    fn new(date: &str, activity: &str) -> Self {
        DailyActivity {
            date: date.to_string(),
            activity: activity.to_string(),
            completed: true,
        }
    }

    fn log_activity(&self) {
        println!("Data: {}", self.date);
        println!("Atividade: {}", self.activity);
    }
}

fn main() {
    let activities = vec![
        "Implementando ownership patterns",
        "Corrigindo borrowing issues",
        "Otimizando performance zero-cost",
        "Escrevendo testes com cargo",
        "Implementando traits e generics",
        "Refatorando para memory safety"
    ];

    let activity = DailyActivity::new("11/08/1981", "Escrevendo documentação 📚");
    activity.log_activity();
}
