use askama::Template;
use crate::expense::Expense;

#[derive(Template)]
#[template(path = "expenses.html")]
pub struct ExpensesTemplate {
    pub expenses: Vec<Expense>,
    pub total_amount: f32,
    pub needs_total: f32,
    pub wants_total: f32,
}

#[derive(Template)]
#[template(path = "new_expense.html")]
pub struct NewExpensesTemplate {}

#[derive(Template)]
#[template(path = "edit_expense.html")]
pub struct EditExpensesTemplate {
    pub expense: Expense,
}