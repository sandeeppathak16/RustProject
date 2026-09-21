use chrono::NaiveDate;
use diesel::{Insertable, Queryable, AsChangeset};
use crate::schema::expenses;


#[derive(Queryable)]
pub struct ExpenseRow {
    pub id: String,
    pub expense_type: String,
    pub amount: f32,
    pub date: NaiveDate,
    pub description: Option<String>,
}



#[derive(Insertable)]
#[diesel(table_name = expenses)]
pub struct NewExpenseRow {
    pub id: String,
    pub expense_type: String,
    pub amount: f32,
    pub date: NaiveDate,
    pub description: Option<String>,
}


#[derive(AsChangeset)]
#[diesel(table_name = expenses)]
pub struct UpdateExpenseRow {
    pub expense_type: String,
    pub amount: f32,
    pub date: NaiveDate,
    pub description: Option<String>,
}