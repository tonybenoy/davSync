use error_chain::error_chain;
use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use std::io::Write;
error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}
#[tokio::main]
async fn main()-> Result<()> {
    let target = ""; // calendar url goes here
    download_ical(target).await?;
    read_ical().await?;
    Ok(())
}

async fn download_ical(url: &str) -> Result<()> {
    let response = reqwest::get(url).await?;
    let path = Path::new("./ical/basic.ical");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content =  response.text().await?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
async fn read_ical() -> Result<()>{
    let buf = BufReader::new(File::open("./ical/basic.ical")
        .unwrap());

    let reader = ical::IcalParser::new(buf);

    for line in reader {
        println!("{:?}", line);
    }
    Ok(())

}