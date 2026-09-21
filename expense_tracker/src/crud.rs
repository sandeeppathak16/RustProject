use chrono::NaiveDate;
use diesel::prelude::*;

use crate::db::establish_connection;
use crate::expense::{Expense, ExpenseType};
use crate::models::{ExpenseRow, NewExpenseRow, UpdateExpenseRow};
use crate::schema::expenses::dsl::{expenses, id};

pub fn create_expense(
    expense_type: ExpenseType,
    amount: f32,
    date: NaiveDate,
    description: Option<String>,
) {
    let mut conn = establish_connection();

    let expense = Expense::new(
        expense_type,
        amount,
        date,
        description,
    );

    let row = NewExpenseRow {
        id: expense.id.to_string(),
        expense_type: expense.expense_type.to_string(),
        amount: expense.amount,
        date: expense.date,
        description: expense.description,
    };

    diesel::insert_into(expenses)
        .values(&row)
        .execute(&mut conn)
        .unwrap();
}


pub fn get_all_expenses() -> Vec<Expense> {
    let mut conn = establish_connection();
    let rows: Vec<ExpenseRow> = expenses
    .load::<ExpenseRow>(&mut conn)
    .expect("Error loading.expenses");

    rows.into_iter()
    .filter_map(|row| Expense::try_from(row).ok())
    .collect()
}

pub fn get_expense_by_id(expense_id: &str) -> Option<Expense> {
    let mut conn = establish_connection();
    let row: Option<ExpenseRow> = expenses
        .filter(id.eq(expense_id))
        .first::<ExpenseRow>(&mut conn)
        .optional()
        .expect("Error loading expense");

    row.and_then(|r| Expense::try_from(r).ok())
}


pub fn update_expense(
    expense_id: String,
    expense_type_val: ExpenseType,
    amount_val: f32,
    date_val: NaiveDate,
    description_val: Option<String>,
) {
    let mut conn = establish_connection();

    let updated_row = UpdateExpenseRow {
        expense_type: expense_type_val.to_string(),
        amount: amount_val,
        date: date_val,
        description: description_val,
    };

    diesel::update(expenses.filter(id.eq(expense_id)))
        .set(&updated_row)
        .execute(&mut conn)
        .expect("Failed to update expense");
}

pub fn delete_expense(expense_id: String) {
    let mut conn = establish_connection();

    diesel::delete(expenses.filter(id.eq(expense_id)))
        .execute(&mut conn)
        .expect("Failed to delete expense");
}