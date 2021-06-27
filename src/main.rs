use structopt::StructOpt;
use exitfailure::{ExitFailure};
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "c", long = "category", default_value = "sport")]
    category: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Joke {
    value: String,
}

impl Joke {
    async fn get(category: &String) -> Result<Self, ExitFailure> {
        let url = format!("https://api.chucknorris.io/jokes/random?category={}", category);
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<Joke>()
            .await?;
        Ok(resp)        
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let resp = Joke::get(&args.category).await?;
    println!("{}", resp.value);

    Ok(())
}
