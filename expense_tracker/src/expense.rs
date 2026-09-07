use chrono::NaiveDate;
use uuid::Uuid;

enum ExpenseType {
    Need,
    Want,
}

struct Expense {
    id: Uuid,
    expense_type: ExpenseType,
    amount: f32,
    date: NaiveDate,
    description: Option<String>,
}

impl Expense {
    pub fn new(
        expense_type: ExpenseType,
        amount: f32,
        date: NaiveDate,
        description: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            expense_type,
            amount,
            date,
            description,
        }
    }
}