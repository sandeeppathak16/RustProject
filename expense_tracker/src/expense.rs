use chrono::NaiveDate;
use uuid::Uuid;
use std::fmt;
use std::str::Fromstr;
use crate::model::ExpenseRow;


enum ExpenseType {
    Need,
    Want,
}

impl fmt::Display for ExpenseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self:Err> {
        match s {
            "Need" => Ok(ExpenseType::Need),
            "Want" => Ok(Expense::Want),
            _ => Err(format!("Invalid expense type: {}", s)),
        }
    }
}

impl FromStr for ExpenseType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Need" => Ok(ExpenseType::Need),
            "Want" => Ok(ExpenseType::Want),
            _ => Err(format!("Invalid expense type: {}", s)),
        }
    }
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

impl TryFrom<ExpenseRow> for Expense {
    type Error = String;

    fn try_from(row: ExpenseRow) -> Result<Self, Self::Error> {
        Ok(Expense {
            id: row.id.parse().unwrap(),
            expense_type: row.expense_type.parse()?,
            amount: row.amount,
            date: row.date,
            description: row.description,
        })
    }
}


