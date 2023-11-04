pub mod drugs;

#[tokio::main]
async fn main() {
    let url: &str = "https://www.moh.gov.sg/healthcare-schemes-subsidies/subsidised-drug-list";

    let mut drugs_list: drugs::DrugList = drugs::DrugList::new();
    drugs_list.get_drugs(url).await;
    if let Err(e) = drugs_list.write_to_disk() {
        println!("Error writing to disk: {}", e);
    }
}
