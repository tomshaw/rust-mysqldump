use cli_table::{print_stdout, Cell, Color, Style, Table};

pub fn print_databases(databases: &Vec<(usize, String, u128)>) {
    let table_rows: Vec<_> = databases
        .into_iter()
        .map(|(i, db, duration)| vec![
            (i + 1).to_string().cell().foreground_color(Some(Color::Yellow)),
            db.cell().foreground_color(Some(Color::Yellow)),
            duration.to_string().cell().foreground_color(Some(Color::Yellow))
        ])
        .collect();

    let table = table_rows
        .table()
        .title(vec![
            "Index".cell().bold(true),
            "Database Name".cell().bold(true),
            "Export Duration (microseconds)".cell().bold(true)
        ]);

    assert!(print_stdout(table).is_ok());
}

