use fjall::{Database, KeyspaceCreateOptions, KvSeparationOptions};
use mysql::{Pool, prelude::Queryable};
use subtitles::Subtitle;

fn main() {
    let db = "mysql://user:password@127.0.0.1:3306/rrys";
    let pub_pool = Pool::new(db).unwrap();
    let mut pub_conn = pub_pool.get_conn().unwrap();

    let subtitles: Vec<Subtitle> = pub_conn
        .query_map(
            "SELECT id, cnname, enname, segment, segment_num, source, lang, format, file, views, downloads, dateline FROM subtitle",
            |(
                id,
                cnname,
                enname,
                segment,
                segment_num,
                source,
                lang,
                format,
                file,
                views,
                downloads,
                dateline,
            )| Subtitle {
                id,
                cnname,
                enname,
                segment,
                segment_num,
                source,
                lang,
                format,
                file,
                views,
                downloads,
                dateline,
            },
        )
        .unwrap();

    println!("Fetched {} subtitles from MySQL", subtitles.len());

    let db = Database::builder("fjall.db").open().unwrap();
    let subtitles_ks = db
        .keyspace("subtitles", || {
            KeyspaceCreateOptions::default()
                .with_kv_separation(Some(KvSeparationOptions::default()))
        })
        .unwrap();

    for subtitle in subtitles {
        let mut s = subtitle;
        s.lang = s.lang.trim_matches('/').to_string();
        s.format = s.format.trim_matches('/').to_string();
        let subtitle_json = serde_json::to_string(&s).unwrap();
        let key = format!(
            "{}{}{:05}",
            s.cnname,
            s.enname.unwrap_or_default().to_lowercase(),
            s.id,
        );
        subtitles_ks.insert(&key, subtitle_json.as_bytes()).unwrap();
    }
    println!("Finished inserting subtitles into Fjall database");
}
