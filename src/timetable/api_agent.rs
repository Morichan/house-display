extern crate reqwest;

use std::io::Read;
use url::Url;

pub struct ApiAgent {
    ekispert_url: String,
}

impl ApiAgent {
    pub fn new() -> Self {
        ApiAgent {
            ekispert_url: "https://roote.ekispert.net/ja/result".to_string(),
        }
    }

    pub fn search_train_time(&self) -> String {
        let mut resp = reqwest::get(&self.create_url()).unwrap();
        let mut s = String::new();

        resp.read_to_string(&mut s).unwrap();
        println!("{}", resp.url().as_str());

        return s;
    }

    fn create_url(&self) -> String {
        // let mut url = Url::parse("https://roote.ekispert.net/ja/result?arr=都庁前&arr_code=29213&connect=true&day=7&dep=豊島園%28都営線%29&dep_code=22836&highway=true&hour=7&liner=true&local=true&locale=ja&minute10=5&minute1=5&plane=true&provider=&shinkansen=true&ship=true&sort=time&submit_btn=検索&surcharge=3&ticket_type=0&transfer=2&type=dep&utf8=✓&via1=&via1_code=&via2=&via2_code=&yyyymm=201905").unwrap();
        let mut url = Url::parse(&self.ekispert_url).unwrap();

        url.query_pairs_mut().clear()
            .append_pair("dep", "豊島園%28都営線%29")
            .append_pair("dep_code", "22836")
            .append_pair("arr", "都庁前")
            .append_pair("arr_code", "29213")

            .append_pair("yyyymm", "201905")
            .append_pair("day", "7")
            .append_pair("hour", "7")
            .append_pair("minute10", "5")
            .append_pair("minute1", "5")

            .append_pair("locate", "ja")
            .append_pair("connect", "true")
            .append_pair("highway", "true")
            .append_pair("liner", "true")
            .append_pair("local", "true")
            .append_pair("plane", "true")
            .append_pair("shinkansen", "true")
            .append_pair("ship", "true")

            .append_pair("sort", "time")
            .append_pair("submit_btn", "検索")
            .append_pair("surcharge", "3")
            .append_pair("ticket_type", "0")
            .append_pair("transfer", "2")
            .append_pair("type", "dep")
            .append_pair("utf8", "✓");

        return String::from(url.as_str());
    }
}
