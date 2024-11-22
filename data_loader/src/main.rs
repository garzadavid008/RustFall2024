use ureq;
use serde::Deserialize;

use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use reqwest;
use serde_json::Value;
use std::thread;
use std::time::Duration;


#[derive(Debug, Deserialize)]
struct Price 
{
    usd:f32,
}
impl Price
{
    fn new(price:f32)->Self
    {
        Self
        {
        usd:price,
        }
    }
}


#[derive(Debug, Deserialize)]
struct Bitcoin {
    bitcoin: Price,
}
#[derive(Debug, Deserialize)]
struct Ethereum{
    ethereum:Price,
}
#[derive(Debug, Deserialize)]
struct SP500
{
    sp500:Price,
}
impl Bitcoin
{
    fn new(price:Price)-> Self
    {
        Self
        {
            bitcoin: price,
        }
    }
}
impl Ethereum
{
    fn new(price:Price)-> Self
    {
        Self
        {
            ethereum: price,
        }
    }
}
impl SP500
{
    fn new(price:Price)-> Self
    {
        Self
        {
            sp500: price,
        }
    }
}

pub trait Pricing{
    fn fetch_price(&self) -> f32;
    fn save_to_file(&self, price:f32);
    fn name(&self)-> &str; 
}

impl Pricing for Bitcoin
{

    fn fetch_price(&self) -> f32
    {

        let url = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";

        let req = ureq::get(url).call().unwrap();
        let content = req.into_json::<Bitcoin>();

        content.unwrap().bitcoin.usd
    }

    fn save_to_file(&self,price:f32)
    {
        let b = Path::new("bitcoin.txt").is_file();
        if !b
        {
                let mut file = File::create("bitcoin.txt").unwrap();
                writeln!(file, "{}",price).unwrap();
        } else{  
            let mut file = OpenOptions::new()
              .append(true)
              .open("bitcoin.txt")
              .unwrap();

            writeln!(file,"{}",price).unwrap();
        }
       
    }
    fn name(&self) -> &str{
        "BIC"
    }
}

impl Pricing for Ethereum
{
    fn fetch_price(&self) -> f32
    {
        let url = "https://api.coingecko.com/api/v3/simple/price?ids=ethereum&vs_currencies=usd";

        let req = ureq::get(url).call().unwrap();
        let content = req.into_json::<Ethereum>();

        content.unwrap().ethereum.usd
    }

    fn save_to_file(&self,price:f32)
    {

        let b = Path::new("ethereum.txt").is_file();
        if !b
        {
                let mut file = File::create("ethereum.txt").unwrap();
                writeln!(file, "{}",price).unwrap();
        } else{  
            let mut file = OpenOptions::new()
              .append(true)
              .open("ethereum.txt")
              .unwrap();
            writeln!(file,"{}",price).unwrap();
        }
       
    }
    fn name(&self) -> &str{
        "ETH"
    }
}

impl Pricing for SP500 {
    fn fetch_price(&self) -> f32 {
        let url = "https://query1.finance.yahoo.com/v8/finance/chart/%5EGSPC?interval=1m&range=1d";

        let response = reqwest::blocking::get(url)
            .expect("Failed to grab data");

        if response.status().as_u16() == 429 {
            return 5948.71;
        }

        let response_text = response.text().expect("Failed");

        let v: Value = serde_json::from_str(&response_text).expect("Failed to parse string");

        if let Some(price) = v["chart"]["result"][0]["meta"]["regularMarketPrice"].as_f64() {
            price as f32
        } else {
            println!("Failed to grab price for SP500");
            0.0
        } 
    } 

    fn save_to_file(&self,price:f32){

       let b = Path::new("sp500.txt").is_file();
       if !b
       {
               let mut file = File::create("sp500.txt").unwrap();
               writeln!(file, "{}",price).unwrap();
       } else{  
           let mut file = OpenOptions::new()
             .append(true)
             .open("sp500.txt")
             .unwrap();

           writeln!(file,"{}",price).unwrap();
       }
    }
    fn name(&self) -> &str{
        "SP500"
    }
}

fn main() {
    let mut price1= Price::new(0.0);
    let mut price2= Price::new(0.0);
    let mut price3= Price::new(0.0);

    let mut bitcoin = Bitcoin::new(price1);
    let mut ethereum = Ethereum::new(price2);
    let mut sp500 = SP500::new(price3);


    let three_curr: Vec<&dyn Pricing> = vec![&bitcoin,&ethereum,&sp500];


let mut i =0;

while i < 100
{
    for curr in three_curr.iter() {
        let mut price = curr.fetch_price();
        println!("The price for {} is : {}",curr.name(),price);
        curr.save_to_file(price);
        thread::sleep(Duration::from_secs(10));
    }
    i+=1;
}


}