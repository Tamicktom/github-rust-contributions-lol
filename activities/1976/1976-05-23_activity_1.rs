// Resolvendo bugs 🐛
// Data: 23/05/1976
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

    let activity = DailyActivity::new("23/05/1976", "Resolvendo bugs 🐛");
    activity.log_activity();
}
