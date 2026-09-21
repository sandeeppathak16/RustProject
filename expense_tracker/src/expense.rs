use chrono::NaiveDate;
use uuid::Uuid;
use std::fmt;
use std::str::FromStr;
use crate::models::ExpenseRow;


pub enum ExpenseType {
    Need,
    Want,
}


impl fmt::Display for ExpenseType {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            ExpenseType::Need => write!(f, "Need"),
            ExpenseType::Want => write!(f, "Want"),
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

pub struct Expense {
    pub id: Uuid,
    pub expense_type: ExpenseType,
    pub amount: f32,
    pub date: NaiveDate,
    pub description: Option<String>,
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


