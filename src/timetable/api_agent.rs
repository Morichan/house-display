extern crate reqwest;

use std::io::Read;

pub struct ApiAgent;

impl ApiAgent {
    pub fn new() -> Self { ApiAgent{} }

    pub fn search_train_time(&self) -> String {
        let mut resp = reqwest::get("https://roote.ekispert.net/ja/result?arr=都庁前&arr_code=29213&connect=true&day=7&dep=豊島園%28都営線%29&dep_code=22836&highway=true&hour=7&liner=true&local=true&locale=ja&minute10=5&minute1=5&plane=true&provider=&shinkansen=true&ship=true&sort=time&submit_btn=検索&surcharge=3&ticket_type=0&transfer=2&type=dep&utf8=✓&via1=&via1_code=&via2=&via2_code=&yyyymm=201905").unwrap();
        let mut s = String::new();
        resp.read_to_string(&mut s).unwrap();
        return s;
    }
}
