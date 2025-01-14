use log::info;
use tokio::{
    io::{
        AsyncBufReadExt,
        AsyncWriteExt,
    },
    process::Command,
};

pub fn gen_ua() -> String {
    // let rn = rand::random::<u64>();
    // let n1 = 50 + (rn % 30);
    // let n2 = 1000 + (rn % 6000);
    // let n3 = 10 + (rn % 150);
    // format!(
    //     "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.{}.{} Safari/537.36",
    //     n1, n2, n3
    // );
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:94.0) Gecko/20100101 Firefox/94.0".into()
    // "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36".into()
    // "Mozilla/5.0 (X11; Linux x86_64; rv:94.0) Gecko/20100101 Firefox/94.0".into()
    // "Mozilla/5.0 (Android 10; Mobile; rv:94.0) Gecko/94.0 Firefox/94.0".into()
    // "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36"
    //     .into()
}

pub async fn js_call(js: &str, func: &str, args: &Vec<(u8, String)>) -> anyhow::Result<Vec<String>> {
    let mut tmp = format!("console.log({}(", &func);
    for a in args {
        // 0 for non-quotation marks, 1 for quotation marks
        if a.0 == 0 {
            tmp.push_str(&a.1);
            tmp.push_str(",");
        } else {
            tmp.push_str("\"");
            tmp.push_str(&a.1);
            tmp.push_str("\"");
            tmp.push_str(",");
        }
    }
    tmp.push_str("));");
    let tmp = format!("{};{}", &js, &tmp);
    let mut rt =
        Command::new("node").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn()?;
    let mut rtin = rt.stdin.take().unwrap();
    let rtout = rt.stdout.take().unwrap();
    tokio::task::spawn_local(async move {
        let _ = rtin.write_all(tmp.as_bytes()).await.unwrap();
        rtin.shutdown().await.unwrap();
    });
    // let rtout = rt.wait_with_output().await.unwrap();

    let mut reader = tokio::io::BufReader::new(rtout).lines();
    let mut ret = Vec::new();
    while let Some(line) = reader.next_line().await.unwrap() {
        ret.push(line)
    }
    info!("{:?}", &ret);
    Ok(ret)
}

pub fn vn(mut val: u64) -> Vec<u8> {
    let mut buf = b"".to_vec();
    while (val >> 7) != 0 {
        let m = val & 0xFF | 0x80;
        buf.push(m.to_le_bytes()[0]);
        val >>= 7;
    }
    buf.push(val.to_le_bytes()[0]);
    buf
}

pub fn tp(a: u64, b: u64, ary: &[u8]) -> Vec<u8> {
    let mut v = vn((b << 3) | a);
    v.append(ary.to_vec().as_mut());
    v
}

pub fn rs(a: u64, ary: &[u8]) -> Vec<u8> {
    let mut v = vn(ary.len() as u64);
    v.append(ary.to_vec().as_mut());
    tp(2, a, &v)
}

pub fn nm(a: u64, ary: u64) -> Vec<u8> {
    tp(0, a, &vn(ary))
}
