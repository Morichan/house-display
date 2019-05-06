extern crate reqwest;

use std::io::Read;
use url::Url;
use chrono::Local;

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
        let mut url = Url::parse(&self.ekispert_url).unwrap();
        let now = self.now_time();

        url.query_pairs_mut().clear()
            .append_pair("dep", "豊島園(都営線)")
            .append_pair("dep_code", "22836")
            .append_pair("arr", "都庁前")
            .append_pair("arr_code", "29213")

            .append_pair("yyyymm", &now.year_and_month)
            .append_pair("day", &now.day)
            .append_pair("hour", &now.hour)
            .append_pair("minute10", &now.min10)
            .append_pair("minute1", &now.min1)

            .append_pair("locale", "ja")
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

    fn now_time(&self) -> Now {
        let local_now = Local::now();
        let minutes_string = local_now.format("%M").to_string();
        let mut minutes = minutes_string.chars();

        return Now {
            year_and_month: local_now.format("%Y%m").to_string(),
            day: local_now.format("%-d").to_string(),
            hour: local_now.format("%-H").to_string(),
            min10: minutes.nth(0).unwrap().to_string(), // 10の位
            min1: minutes.nth(0).unwrap().to_string(), // 1の位
        };
    }
}

struct Now {
    year_and_month: String,
    day: String,
    hour: String,
    min10: String,
    min1: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_now_time() {
        let expected = Local::now();

        let actual = ApiAgent::new().now_time();

        assert_eq!(expected.format("%Y%m").to_string(), actual.year_and_month);
        assert_eq!(expected.format("%-d").to_string(), actual.day);
        assert_eq!(expected.format("%-H").to_string(), actual.hour);
        assert_eq!(expected.format("%M").to_string(), format!("{}{}", actual.min10, actual.min1));
    }

    #[test]
    fn create_ekispert_url() {
        let obj = ApiAgent::new();
        let test_time = obj.now_time();
        let expected = Url::parse(&format!(
                "https://roote.ekispert.net/ja/result?dep=豊島園%28都営線%29&dep_code=22836&arr=都庁前&arr_code=29213&yyyymm={}&day={}&hour={}&minute10={}&minute1={}&locale=ja&connect=true&highway=true&liner=true&local=true&plane=true&shinkansen=true&ship=true&sort=time&submit_btn=検索&surcharge=3&ticket_type=0&transfer=2&type=dep&utf8=%E2%9C%93",
                test_time.year_and_month,
                test_time.day,
                test_time.hour,
                test_time.min10,
                test_time.min1)).unwrap();

        let actual = Url::parse(&obj.create_url()).unwrap();

        assert_eq!(expected, actual);
    }
}
