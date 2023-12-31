use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};

pub async fn fetch(url: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let url = format!("{}", &url);
    let res = reqwest::get(url).await?.text().await?;

    let r: Vec<Vec<_>> = Document::from(res.as_str())
        .find(Class("list-row-container").descendant(Name("li")))
        .map(|tag| {
            tag.find(Name("span")).collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    println!("{:#?}", r);

    //let r: Option<String> = Document::from(res.as_str())
    //    .find(Class("list-row-container"))
    //    .filter_map(|n| n.attr(""))
    //    .map(From::from)
    //    .next();

    Ok(Some("".to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a = fetch("https://www.python.org/downloads/").await?;
    println!("{:#?}", a);
    Ok(())
}
