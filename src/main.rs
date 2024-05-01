use std::error::Error;
use std::fs::File;
use std::io::{BufReader,Write};
use csv::{ReaderBuilder, Reader, StringRecord};
use chrono::{Duration, Local, Months, NaiveDateTime};

const NAME_OF_TABLE: &str = "table.csv";
const DATE_LAST_MODIFICATION: &str = "2024-03-28";

fn hello_user(){
    println!("Compound Interest Calculator by @KikeKnox (GitHub)");
    println!("This program is going to calculate the compound interest for each line in the CSV file");
    println!("The CSV file must have the same structure as the example file");
    println!("Last update: {}", DATE_LAST_MODIFICATION);
}

/* THIS FUNCTIOS ARE NOT USED IN THE FINAL VERSION BUT STILL ARE INTERESTING
fn leap_year(year: u16) -> bool {
    return (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
}

fn days_in_month(month: u8, year: u16) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if leap_year(year) { 29 } else { 28 },
        _ => 0,
    }
}
*/

fn main() -> Result<(), Box<dyn Error>> {
    // Start section: print info about the program
    hello_user();

    // Read the CSV file
    let mut input = String::new();

    print!("Please, insert the name of the CSV (if empty, name = table.csv): ");
    std::io::stdout().flush()?; // Flush to make sure the prompt is printed before the input
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    if input.is_empty() {
        input = NAME_OF_TABLE.to_string();
    } else if !input.ends_with(".csv") {
        input.push_str(".csv");
    }

    let file_path: &str = &input;
    let mut reader: Reader<BufReader<File>> = ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .from_reader(BufReader::new(File::open(file_path)?));
    let current_date = Local::now().naive_local();

    // Create the temporary file
    let tmp_file_path = "tmp.csv";
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path(tmp_file_path)?;
    let _ = writer.write_record(&["INITIAL AMOUNT (00.00)", "AMOUNT TO ADD (00.00)", "AMOUNT FREQUENCY (00)", "AMOUNT REGULARITY (D/W/M/Y)", "INTEREST (00.00%)", "PAYMENT FREQUENCY (00)", "PAYMENT REGULARITY (D/W/M/Y)", "TIME PERIOD (00)", "REGULARITY (D/W/M/Y)", "FINAL DATE (YYYY-MM-DD)", "FINAL AMOUNT (00.00)", "INTEREST GAINED (00.00)", "AMOUNT APPORTED (00.00)"]);

    // Create the variables to know how many lines failed and how many lines were processed
    let mut failed_lines = 0;
    let mut processed_lines = 0;

    // Iterate over each line in the CSV
    // NOTE: Probably I'm going to change this to load the lines in different threads and process them in parallel because the calculations are independent
    for (i,line) in reader.records().enumerate() {
        let line: StringRecord = line?;
        if line.len() != 9{
            // Copy the line to the tmp file
            writer.write_record(&line)?;
            if line.len() != 13 {
                // If the line has 13 fields, it means that the line was already processed or is a header
                failed_lines += 1;
            }
            continue;
        }

        // Parse the values from the CSV
        let initial_amount: f64 = line.get(0).unwrap_or_else(|| "0").parse()?;
        let added_amount: f64 = line.get(1).unwrap_or_else(|| "0").parse()?;
        let added_amount_frequency: u16 = line.get(2).unwrap_or_else(|| "0").parse()?;
        let regularity_amount: String = line.get(3).unwrap_or_else(|| "M").trim().to_uppercase();
        let interest_rate: f64 = line.get(4).unwrap_or_else(|| "0").parse()?;
        let interest_payment_frequency : u16 = line.get(5).unwrap_or_else(|| "0").parse()?;
        let regularity_interest: String = line.get(6).unwrap_or_else(|| "D").trim().to_uppercase();
        let time_period: u16 = line.get(7).unwrap_or_else(|| "0").parse()?;    // Consider it as frequency like the others
        let regularity_time: String = line.get(8).unwrap_or_else(|| "Y").trim().to_uppercase();

        // Perform compound interest calculation
        /* There is three variables that define the scenario:
            - The regularity of amount apported to the account
            - The regularity of the interest payment
            - The regularity of the time period
        
        These three parameters are going to define how the compound interest is going to be calculated
        but the most relevant is the regularity of the time period
        How I'm going to focus the problem:
            1 - Calculate the final date.
            2 - Calculate the number of days between the current date and the final date.
            3 - Create counters for the total of days (the main counter), the days of the calculation month, the calculation month and the calculation year.
            4 - Iterate over the days between the current date and the final date.
            5 - Check the regularitys of the amount and interest payment. If needed, convert to daily values and store in other variables
            6 - Calculate the interest for the current day and add to a variable apart
            7 - Check if is interest payday:
                - Daily is always, so no problem
                - Weekly is every 7 days, so check if the main counter is divisible by 7
                - Monthly is every month, so check if we are in the same day of the month (except for 30 or 31 because february, that moves to the last day of the month)
                - Yearly is every year, so check if we are in the same day and month of the year
                - Personalized days is every X days, so check if the main counter is divisible by X
                - Personalized months is every X months, so check if we are in the same day of the month that current day but desplaced by N % X == 0 months
                - Personalized years is every X years, so check if we are in the same day and month of the year but desplaced by N % X == 0 years
            8 - Same for the amount apport

        */

        // Calculate the final date
        let tmp_final_date : NaiveDateTime;
        if regularity_time.starts_with("D") || regularity_time.starts_with("W"){
            let mut dow = 1;
            if regularity_time.starts_with("W") {
                dow = 7;
            }
            tmp_final_date = current_date.checked_add_signed(Duration::try_days((dow * time_period) as i64).unwrap()).unwrap();
        }
        else if regularity_time.starts_with("M") {
            tmp_final_date = current_date.checked_add_months(Months::new(time_period as u32)).unwrap();
        }
        else if regularity_time.starts_with("Y") {
            tmp_final_date = current_date.checked_add_months(Months::new(12 * time_period as u32)).unwrap();
        }
        else
        {
            // Error
            println!("Error with the time period at line {}", i);
            writer.write_record(&line)?;
            failed_lines += 1;
            continue;
        }

        // Fix final date
        let final_date = tmp_final_date;

        // Calculate the "days" where the loop needs to add the interest
        let mut tmp_interest_days : Vec<NaiveDateTime> = Vec::new();
        let mut tmp_counter = current_date;
        if regularity_interest.starts_with("D") || regularity_interest.starts_with("W") {
            let mut dow = 1;
            if regularity_interest.starts_with("W") {
                dow = 7;
            }
            while tmp_counter <= final_date {
                tmp_interest_days.push(tmp_counter);
                tmp_counter = tmp_counter.checked_add_signed(Duration::try_days(dow * interest_payment_frequency as i64).unwrap()).unwrap();
            }
        } else if regularity_interest.starts_with("M") {
            while tmp_counter <= final_date {
                tmp_interest_days.push(tmp_counter);
                tmp_counter = tmp_counter.checked_add_months(Months::new(interest_payment_frequency as u32)).unwrap();
            }
        } else if regularity_interest.starts_with("Y") {
            while tmp_counter <= final_date {
                tmp_interest_days.push(tmp_counter);
                tmp_counter = tmp_counter.checked_add_months(Months::new(12 * interest_payment_frequency as u32)).unwrap();
            }
        } else {
            // Error
            println!("Error with the interest payment frequency at line {}", i);
            writer.write_record(&line)?;
            failed_lines += 1;
            continue;
        }

        // Calculate the "days" where the loop needs to add the amount
        let mut tmp_amount_days : Vec<NaiveDateTime> = Vec::new();
        let mut tmp_counter = current_date;
        if regularity_amount.starts_with("D") || regularity_amount.starts_with("W") {
            let mut dow = 1;
            if regularity_amount.starts_with("W") {
                dow = 7;
            }
            while tmp_counter <= final_date {
                tmp_amount_days.push(tmp_counter);
                tmp_counter = tmp_counter.checked_add_signed(Duration::try_days(dow * added_amount_frequency as i64).unwrap()).unwrap();
            }
        } else if regularity_amount.starts_with("M") {
            while tmp_counter <= final_date {
                tmp_amount_days.push(tmp_counter);
                tmp_counter = tmp_counter.checked_add_months(Months::new(added_amount_frequency as u32)).unwrap();
            }
        } else if regularity_amount.starts_with("Y") {
            while tmp_counter <= final_date {
                tmp_amount_days.push(tmp_counter);
                tmp_counter = tmp_counter.checked_add_months(Months::new(12 * added_amount_frequency as u32)).unwrap();
            }
        } else {
            // Error
            println!("Error with the amount apport frequency at line {}", i);
            writer.write_record(&line)?;
            failed_lines += 1;
            continue;
        }
        
        // Fix the vectors
        tmp_amount_days.remove(0);
        tmp_interest_days.remove(0);
        let amount_days = tmp_amount_days;
        let interest_days = tmp_interest_days;

        // Loop to calculate the final amount
        let mut date_counter = current_date;
        let mut interest_counter = 0.0;
        let mut final_amount = initial_amount;
        let mut interest_gain = 0.0;
        let mut amount_apport = initial_amount;

        while date_counter <= final_date {
            // calculate the interest
            let interest_day = (final_amount * interest_rate / 100.0) / 365.0;  // Daily interest (usually, the calculation don't consider leap years)
            interest_counter += interest_day;
            if interest_days.contains(&date_counter) {
                final_amount += interest_counter;
                interest_gain += interest_counter;
                interest_counter = 0.0;
            }
            if amount_days.contains(&date_counter) {
                final_amount += added_amount;
                amount_apport += added_amount;
            
            }
            date_counter = date_counter.checked_add_signed(Duration::try_days(1).unwrap()).unwrap();
        }

        // Convert the record to a vector of Strings
        let mut line_vec: Vec<String> = line.iter().map(|s| s.to_string()).collect();

        // Add the new fields to the vector
        line_vec.push(final_date.format("%Y-%m-%d").to_string());
        line_vec.push(format!("{:.2}", final_amount));
        line_vec.push(format!("{:.2}", interest_gain));
        line_vec.push(format!("{:.2}", amount_apport));
        
        // Write the vector to the CSV file
        writer.write_record(&line_vec)?;

        // Increase the processed lines counter
        processed_lines += 1;
    }

    // Close the reader
    if let Err(e) = reader.into_inner().into_inner().flush() {
        eprintln!("Error closing the file: {}", e);
    }

    // After the calculations, rename the tmp file to the original file
    std::fs::rename(tmp_file_path, file_path)?;

    // Close the writer
    if let Err(e) = writer.flush() {
        eprintln!("Error closing the file: {}", e);
    }

    // Print the results
    println!("Processed lines: {}", processed_lines);
    println!("Failed lines: {}", failed_lines);

    // Return Ok
    Ok(())
}
