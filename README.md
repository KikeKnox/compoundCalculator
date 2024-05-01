# Compound Interest Calculator

This is a Rust project that implements a compound interest calculator.

## Description

The compound interest calculator takes a CSV file as input and calculates the compound interest for each line in the file. The CSV file must have a specific structure for the program to work correctly.

The program is highly customizable, allowing different amounts, frequencies, and interests for each line in the CSV file. Currently, the interest is fixed, but there are plans to make it variable in the future.

The calculations takes the additions at the end and not in the begining (that means first calculate the interest and add to the amount and later adds the deposits the user may have configured). This and the use of more than 2 decimals in calculations (even the final result is only with 2) maybe cause the calculation to have a little error compared to other platforms.

## Option 1 - Clone and build
### Installation

To use this project, you need to have Rust installed. If you don't have it, you can download it from the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

Once you have Rust installed, you can clone this repository:

```sh
git clone https://github.com/yourusername/compound_calculator.git
```

### Usage

To run the program, navigate to the project directory and run the following command:

```sh
cargo run
```

This will start the program.

## Option 2 - Download the exe
Just for people that don't want to struggle with programming stuff, can download the `compoundCalculator.exe` from [here](https://github.com/KikeKnox/Scripts/raw/main/Rust/compoundCalculator/compoundCalculator.exe). Is the code of the repo already compiled so is just download and run.

## CSV File Structure

The CSV file used by this program has a specific structure. Each line in the file represents a different calculation, and each field in the line represents a different parameter for the calculation. Here is a description of each field:

1. `INITIAL AMOUNT (00.00)`: The initial amount of money.
2. `AMOUNT TO ADD (00.00)`: The amount of money to add in each period.
3. `AMOUNT FREQUENCY (00)`: The numeric period to add the amount. I.E: If the amount is added every 3 months, here goes the 3. Only integers.
4. `AMOUNT REGULARITY (D/W/M/Y)`: The regularity of adding money (daily, weekly, monthly, yearly). I.E: If the amount is added every 3 months, here goes the "months", "month", "monthly" or "m".
5. `INTEREST (00.00%)`: The interest rate.
6. `PAYMENT FREQUENCY (00)`: The frequency of interest payment. I.E: If the interest is paid every day, here goes a 1. Only integers.
7. `PAYMENT REGULARITY (D/W/M/Y)`: The regularity of interest payment (daily, weekly, monthly, yearly). I.E: If the interest is paid every day, here goes a "day", "days", "daily" or "d".
8. `TIME PERIOD (00)`: The total time period for the calculation. I.E: If the deposit is for 30 years, here goes the 30. Only integers.
9. `REGULARITY (D/W/M/Y)`: The regularity of the time period (daily, weekly, monthly, yearly). I.E: If the deposit is for 30 years, here goes a "year", "years", "yearly" or "y".
10. `FINAL DATE (YYYY-MM-DD)`: The final date of the calculation (this field is calculated by the program).
11. `FINAL AMOUNT (00.00)`: The final amount of money after the calculation (this field is calculated by the program).
12. `INTEREST GAINED (00.00)`: The total interest gained during the calculation (this field is calculated by the program).
13. `AMOUNT APPORTED (00.00)`: The total amount of money apported during the calculation (this field is calculated by the program).

Each field should be separated by a semicolon (`;`).

## Contribution

Contributions are welcome. If you find a bug or have a suggestion for improving the program, please open an issue or a pull request.

## License

This project is licensed under the MIT license.
