# DocFinder
The DocFinder project is a Rust-based web scraper designed to extract and store data from the HSE website, specifically focusing on healthcare practices in Ireland. It utilizes the `reqwest` library for HTTP requests, `scraper` for parsing HTML, and `csv` for writing data to CSV files.
## Features
- Scrapes healthcare practice data from the HSE website.
- Handles pagination to gather data from multiple pages.
- Writes the extracted data to a CSV file.

## Usage
1. Clone the repository:
    ```bash
    git clone https://github.com/yourusername/docfinder.git
    ```
2. Navigate to the project directory:
    ```bash
    cd docfinder
    ```
3. Build the project:
    ```bash
    cargo build --release
    ```
4. Run the scraper:
    ```bash
    cargo run --release
    ```
5. The output will be saved to `practice_data.csv` in the project directory. You can specify a different output file by modifying the `output_file` parameter in the `page_counter` function.

## Using DocFinder as an executable
After building the project, you can run the executable directly from the `target/release` directory:
```bash
./target/release/docfinder
```
## Project Structure
- `src/lib.rs`: Contains the main logic for scraping and data extraction.
- `Cargo.toml`: The configuration file for the Rust project, listing dependencies and metadata.
- `.gitignore`: Specifies files and directories to ignore in version control.
## Requirements
- Rust and Cargo installed on your machine.
- Internet access to fetch data from the HSE website.
## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
## Author
Desmond Molloy
## Acknowledgements
- Thanks to the Rust community for their support and resources.
- Special thanks to the maintainers of the `reqwest`, `scraper`, and `csv` libraries for their excellent work.

## Dependencies
- `reqwest`: For making HTTP requests.
- `scraper`: For parsing HTML documents.
- `csv`: For writing data to CSV files.
## Contributing
Contributions are welcome! Please fork the repository and submit a pull request with your changes.