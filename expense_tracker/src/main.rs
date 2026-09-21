use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
};

use askama::Template;
use chrono::NaiveDate;
use expense_tracker::crud::{
    create_expense, delete_expense, get_all_expenses, get_expense_by_id, update_expense,
};
use expense_tracker::expense::ExpenseType;
use expense_tracker::templates::{
    EditExpensesTemplate, ExpensesTemplate, NewExpensesTemplate,
};
use expense_tracker::worker::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 16384];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let request_str = String::from_utf8_lossy(&buffer[..bytes_read]);
    let first_line = request_str.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let full_uri = parts.next().unwrap_or("/");

    let mut uri_parts = full_uri.splitn(2, '?');
    let path = uri_parts.next().unwrap_or("/");
    let query_str = uri_parts.next().unwrap_or("");

    let body = match request_str.find("\r\n\r\n") {
        Some(idx) => &request_str[idx + 4..],
        None => "",
    };

    match (method, path) {
        ("GET", "/") | ("GET", "/expenses") => {
            let expenses = get_all_expenses();
            let mut total_amount = 0.0;
            let mut needs_total = 0.0;
            let mut wants_total = 0.0;

            for exp in &expenses {
                total_amount += exp.amount;
                match exp.expense_type {
                    ExpenseType::Need => needs_total += exp.amount,
                    ExpenseType::Want => wants_total += exp.amount,
                }
            }

            let template = ExpensesTemplate {
                expenses,
                total_amount,
                needs_total,
                wants_total,
            };

            let contents = template.render().unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                contents.as_bytes().len(),
                contents
            );
            stream.write_all(response.as_bytes()).unwrap();
        }

        ("GET", "/expenses/new") => {
            let template = NewExpensesTemplate {};
            let contents = template.render().unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                contents.as_bytes().len(),
                contents
            );
            stream.write_all(response.as_bytes()).unwrap();
        }

        ("POST", "/expenses") => {
            let params = parse_form(body);
            let expense_type_str = params
                .get("expense_type")
                .map(|s| s.as_str())
                .unwrap_or("Need");
            let expense_type =
                ExpenseType::from_str(expense_type_str).unwrap_or(ExpenseType::Need);
            let amount: f32 = params
                .get("amount")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.0);
            let date = params
                .get("date")
                .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
                .unwrap_or_else(|| chrono::Local::now().date_naive());
            let description = params
                .get("description")
                .cloned()
                .filter(|s| !s.is_empty());

            create_expense(expense_type, amount, date, description);

            let response =
                "HTTP/1.1 303 See Other\r\nLocation: /\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }

        ("GET", "/expenses/edit") => {
            let query = parse_query(query_str);
            if let Some(expense_id) = query.get("id") {
                if let Some(expense) = get_expense_by_id(expense_id) {
                    let template = EditExpensesTemplate { expense };
                    let contents = template.render().unwrap();
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                        contents.as_bytes().len(),
                        contents
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                    return;
                }
            }
            let response =
                "HTTP/1.1 303 See Other\r\nLocation: /\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }

        ("POST", "/expenses/edit") | ("POST", "/expenses/update") => {
            let params = parse_form(body);
            if let Some(id) = params.get("id") {
                let expense_type_str = params
                    .get("expense_type")
                    .map(|s| s.as_str())
                    .unwrap_or("Need");
                let expense_type =
                    ExpenseType::from_str(expense_type_str).unwrap_or(ExpenseType::Need);
                let amount: f32 = params
                    .get("amount")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0.0);
                let date = params
                    .get("date")
                    .and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
                    .unwrap_or_else(|| chrono::Local::now().date_naive());
                let description = params
                    .get("description")
                    .cloned()
                    .filter(|s| !s.is_empty());

                update_expense(
                    id.clone(),
                    expense_type,
                    amount,
                    date,
                    description,
                );
            }

            let response =
                "HTTP/1.1 303 See Other\r\nLocation: /\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }

        ("POST", "/expenses/delete") => {
            let params = parse_form(body);
            if let Some(id) = params.get("id") {
                delete_expense(id.clone());
            }

            let response =
                "HTTP/1.1 303 See Other\r\nLocation: /\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
        }

        _ => {
            let contents = fs::read_to_string("src/404.html")
                .or_else(|_| fs::read_to_string("404.html"))
                .unwrap_or_else(|_| "<h1>404 Not Found</h1>".to_string());
            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                contents.as_bytes().len(),
                contents
            );
            stream.write_all(response.as_bytes()).unwrap();
        }
    }
}

fn parse_form(body: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in body.split('&') {
        if pair.is_empty() {
            continue;
        }
        let mut kv = pair.splitn(2, '=');
        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
            map.insert(url_decode(k), url_decode(v));
        }
    }
    map
}

fn parse_query(query_str: &str) -> HashMap<String, String> {
    parse_form(query_str)
}

fn url_decode(input: &str) -> String {
    let mut bytes = Vec::new();
    let input_bytes = input.as_bytes();
    let mut i = 0;
    while i < input_bytes.len() {
        if input_bytes[i] == b'+' {
            bytes.push(b' ');
            i += 1;
        } else if input_bytes[i] == b'%' && i + 2 < input_bytes.len() {
            if let Ok(b) =
                u8::from_str_radix(std::str::from_utf8(&input_bytes[i + 1..i + 3]).unwrap_or(""), 16)
            {
                bytes.push(b);
                i += 3;
            } else {
                bytes.push(input_bytes[i]);
                i += 1;
            }
        } else {
            bytes.push(input_bytes[i]);
            i += 1;
        }
    }
    String::from_utf8_lossy(&bytes).to_string()
}