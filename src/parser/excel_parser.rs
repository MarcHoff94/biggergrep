use calamine::{open_workbook, Xlsx, DataType, Reader};


pub fn search_excelfile(file_path: &str, query: &str) -> Result<bool,&'static str> {
    let mut workbook: Xlsx<_> = open_workbook(file_path).expect("could not open excelfile");
    let worksheets = workbook.worksheets();
    if worksheets.len() == 0 {
        return Err("This can never happen??? Why do i have to handle this case :/")
    };
    for worksheet in worksheets {
        let values = worksheet.1;
        let matched_values: Vec<(usize, usize, &DataType)> = values.cells().filter(|x: &(usize, usize, &DataType)| x.2 == query).collect();
        if matched_values.len() > 0 {
           return  Ok(true)
        } 
    }
    Ok(false)
}
