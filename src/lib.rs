use scraper::{Html, Selector};
use std::fs::File;
use csv::WriterBuilder;
 
#[derive(Debug)]
#[derive(serde::Serialize)]
pub struct Practice {
    pub name: String,
    pub eircode: String,
    pub phone_number: String,
    pub email_address: String,
    pub website: String,
}
 
pub fn page_counter(document: &Html, output_file: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut full_practice_data = Vec::new();
    // Goal: hse-pagination-listing-item hse-pagination-listing-item--page
    let page_data_selector = Selector::parse("a.hse-pagination-listing__link").unwrap();
    let page_data: Vec<_> = document.select(&page_data_selector).collect();
    let page_count = page_data.len();
    let sample_page = &page_data[page_count-3].text().collect::<String>().replace("Page", "");
    println!("Reading from {sample_page} pages of practice listings");
 
    let base_url = "https://www2.hse.ie/services/find-a-gp/?page=";
 
    let page_count: i32 = sample_page.parse().unwrap();
 
    for page_nr in 1..10 {
        println!("Reading from page {page_nr}");
        let target_url = base_url.to_owned() + &page_nr.to_string();
        let looped_page_content = page_parser(&target_url);
        let looped_page_data = practice_reader(&looped_page_content);
        for practice_data in looped_page_data{
            full_practice_data.push(practice_data);  
    }}
 
        // Write full_practice_data to CSV
    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(File::create(output_file)?);
   
    for data in full_practice_data {
        // Writes each row into the CSV
        wtr.serialize(data)?; // Change if your data type is different
    }
 
    // Ensure all data is flushed and written to the file
    wtr.flush()?;
   
    println!("Data successfully written to practice_data.csv");
    Ok(())
}
 
pub fn page_parser(url:&str) -> Html {
    // Download the target HTML document
    let response = reqwest::blocking::get(url);
 
    // Get the HTML content from the request response
    let html_content = response.unwrap().text().unwrap();
 
    let document = scraper::Html::parse_document(&html_content);
 
    document
}
 
pub fn practice_reader(document: &Html) -> Vec<Practice> {
 
    let mut practice_data = Vec::new();
 
    let practice_name_selector = Selector::parse("h3.hse-location-card__title").unwrap();
    let practice_names: Vec<_> = document.select(&practice_name_selector).collect();
 
    let address_selector = Selector::parse("p.hse-body-s").unwrap();
    let addresses: Vec<_> = document.select(&address_selector).collect();
 
    let contact_selector = Selector::parse("div.hse-result-card__contacts").unwrap();
    let contacts: Vec<_> = document.select(&contact_selector).collect();
 
    // Use the length of the practice_names to ensure we iterate correctly
    let num_practices = practice_names.len();
 
    for i in 0..num_practices {
        // Access elements by index
        let name_element = &practice_names[i];
        let address_element = &addresses[i];
        let contact_element = &contacts[i];
 
        // Extract text
        let name = name_element.text().collect::<String>();
        // Assuming `address_element` is defined and holds the relevant data
        let location = address_element.text().collect::<String>(); // Collect text from the element into a String
        let location_array: Vec<&str> = location.split(',').collect(); // Now you can split the String
        let location_array_length = location_array.len();
 
        // Extract components of street address from initial element
        let eircode = location_array[location_array_length-1].to_string();
 
        // Extract phone number
        let phone_number = contact_element
            .select(&Selector::parse("a").unwrap())
            .filter_map(|a| a.value().attr("href"))
            .find(|href| href.starts_with("tel:"))
            .map(|s| s.to_string())
            .unwrap_or_else(|| "None".to_string()).replace("tel:", "");
 
        // Extract email address
        let email_address = contact_element
            .select(&Selector::parse("a").unwrap())
            .filter_map(|a| a.value().attr("href"))
            .find(|href| href.starts_with("mailto:"))
            .map(|s| s.to_string())
            .unwrap_or_else(|| "None".to_string()).replace("mailto:", "");
 
        // Extract website
        let website = contact_element
            .select(&Selector::parse("a").unwrap())
            .filter_map(|a| a.value().attr("href"))
            .find(|href| href.starts_with("http:") || href.starts_with("https:"))
            .map(|s| s.to_string())
            .unwrap_or_else(|| "None".to_string());
 
        // Create the Practice struct
        let practice_information = Practice {
            name,
            eircode,
            phone_number,
            email_address,
            website,
        };
 
        practice_data.push(practice_information);
    }    
    practice_data
}