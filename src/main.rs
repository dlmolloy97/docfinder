use docfinder::{page_counter, page_parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Allow the user to pass the output file name and the initial URL as command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <output_file> [initial_url]", args[0]);
        return Ok(());
    }
    let output_file = if args.len() > 1 { &args[1] }
    else { "practice_data.csv" };
    let initial_url = if args.len() > 2 { &args[2] }
    else { "https://www2.hse.ie/services/find-a-gp/?page=
1" };
    // Set the output file for the CSV writer
    // Initialize the HTML document from the first page
    let document = page_parser(initial_url);
    
    // Call the page_counter function to process the document
    page_counter(&document, output_file.to_string())?;
    Ok(())
}