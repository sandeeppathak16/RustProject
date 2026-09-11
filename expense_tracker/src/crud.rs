use diesel::prelude::*;
use chrono::NaiveDate;
use crate::expense::ExpenseType;

use crate::db::establish_connection;
use crate::models::NewExpenseRow;
use crate::schema::expenses;
use crate::expense::Expense;

pub fn create_expense(
    expense_type: ExpenseType,
    amount: f32,
    date: NaiveDate,
    description: Option<String>,
) {
    let mut conn = establish_connection();
    let expense: Expense = Expense::new(expense_type, amount, date, description);

    let row = NewExpenseRow {
        id: expense.id.to_string(),
        expense_type: expense.expense_type.to_string(),
        amount: expense.amount,
        date: expense.date,
        description: expense.description,
    };

    diesel::insert_into(expenses::table)
        .values(&row)
        .execute(&mut conn)
        .unwrap();
}


pub fn get_all_expenses() -> Vec<Expense> {
    let mut conn = establish_connection();
    let rows: Vec<ExpenseRow> = expenses
    .load::<ExpenseRow>(&mut conn)
    .expect("Error loading.expenses");

    rows.into_iter().map(|row| {
        let expense = match Expense::try_from(row) {
            Ok(exp) => exp,
            Err(_) => panic!()
        };
    }).collect()

    
}