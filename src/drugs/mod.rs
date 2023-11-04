use crate::drugs::drug::Drug;
use bincode::Error;
use chrono::{DateTime, Utc};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub mod drug;

#[derive(Debug, Serialize, Deserialize)]
pub struct DrugList {
    pub date_generated: Option<DateTime<Utc>>,
    drugs: Vec<Drug>,
}

impl Default for DrugList {
    fn default() -> Self {
        Self::new()
    }
}

impl DrugList {
    pub fn new() -> Self {
        DrugList {
            date_generated: None,
            drugs: Vec::new(),
        }
    }

    pub fn import_from_disk(file_path: &str) -> Self {
        let file = File::open(file_path).unwrap();
        let drugs: DrugList = bincode::deserialize_from(file).unwrap();
        DrugList { ..drugs }
    }

    pub fn add_drug(&mut self, drug: Drug) {
        self.drugs.push(drug);
    }

    pub async fn get_drugs(&mut self, url: &str) {
        let html = reqwest::get(url).await.unwrap().text().await.unwrap();
        let document = Html::parse_document(&html);

        self.date_generated = Some(Utc::now());

        let tbody_selector = Selector::parse("tbody").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        for element in document.select(&tbody_selector) {
            for tr in element.select(&tr_selector) {
                let mut v: Vec<String> = Vec::new();
                for td in tr.select(&td_selector) {
                    v.push(td.text().collect::<Vec<_>>().join(",").to_string());
                }
                let d: Drug = Drug::new(
                    v[0].clone(),
                    v[1].clone(),
                    v[2].clone(),
                    v[3].clone(),
                    v[4].clone(),
                );
                self.add_drug(d);
            }
        }
    }

    pub fn write_to_disk(&self) -> Result<(), Error> {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        let mut file: File = File::create("drugs.bin").unwrap();
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn _has_timestamp(&self) -> bool {
        self.date_generated.is_some()
    }

    pub fn get_timestamp_in_string(&self) -> Option<String> {
        self.date_generated
            .map(|date: DateTime<Utc>| date.to_rfc3339())
    }

    pub fn get_num_records(&self) -> usize {
        self.drugs.len()
    }
}
