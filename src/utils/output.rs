use cli_table::{print_stdout, Cell, Color, Style, Table};
use std::io::Write;
use zip::write::FileOptions;
use zip::CompressionMethod;

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

pub fn zip_file(file_path: &str, zip_path: &str) -> std::io::Result<()> {
    let data = std::fs::read(file_path)?;

    let zip_file = std::fs::File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(zip_file);

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);
    zip.start_file(std::path::Path::new(file_path).file_name().unwrap().to_str().unwrap(), options)?;
    zip.write_all(&data)?;

    Ok(())
}
