

include!("lib.rs");
include!("gen_cph.rs");

#[macro_use]
extern crate log;
extern crate chrono;
extern crate env_logger;

fn init_log() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] [{}] {}",
                Local::now().format("%Y-%m-%d/%H:%M:%S"),
                record.level(),
                9999,
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();

    info!("env_logger initialized.");
}


use std::time::{Duration, SystemTime};

fn main() {
    init_log();
    gen_cph(10000*10000);
    return;

    let mut long_text = LongText::default();
    long_text.set_text(String::from("这应该是中国下一步最重要任务，北京上海出手了！　　确实是一点大意不得。　　昨天（3月7日），北京警方发布情况通报：　　2020年3月4日，廖某君、廖某海等两家一行8人自意大利乘机抵达北京首都国际机场。其中，廖某君、廖某海等4人被我市确诊为新冠肺炎病例。　　经查，廖某君和廖某海系姐弟关系，与其家人长期在意大利经商。2月下旬以来，廖某君、廖某海相继出现发热、干咳等症状，在登机前使用药物退烧降温。家庭成员中存在不如实填写《中华人民共和国出/入境健康申明卡》的情形，给同机人员造成传染风险。3月6日，顺义公安分局以廖某君等人涉嫌妨害传染病防治罪，依法开展立案侦查……　　信息量其实挺大的：　　1，北京发生了4起输入性病例，来源是意大利。　　2，在登机回京前，廖某君等人其实已经出现发热、干咳等症状，即存在疑似症状。　　3，应该是为了能顺利回来，他们登机前使用药物退烧降温。　　4、回国入境，肯定需要填写健康申明，他们应该也没有如实填写自己发热、干咳症状。　　5，警方出手，3月6日，警方以廖某君等人涉嫌妨害传染病防治罪，依法开展立案侦查。　　也难怪警方要出手，因为这里面存在一系列的隐患：　　第一，你隐瞒了病情，给同机人员带来传染风险。　　第二，肯定的，坐在他们周边的人员，也要接受隔离观察。　　第三，他们如确实患病，没如实申报，存在在北京继续传播的风险。　　毕竟，北京也是重中之重啊。最近的领导人会议，一再强调要加强北京的防控。北京不容有失，17年前教训深刻，首都安全稳定，直接关系党和国家工作大局啊！　　而且，北京警方也再次警告，对于类似拒绝执行疫情防控措施等违法犯罪行为，将依法坚决打击。　　同一天，看到上海连发多次警告。　　比如，当天下午，在上海防控工作新闻发布会上，上海海关副关长蒋原就警告：如实申报是出入境人员所承担的一项法律义务，出入境人员如有隐瞒或虚假填报，造成疫情传播，将被依据法律追究相关责任。　　请注意，隐瞒或虚假填报，是要付法律责任的。　　这应该也是有针对性的。　　随着国外疫情加重，一些华人华侨和外国人，觉得还是中国安全，于是纷纷去中国。这可以理解，但也确实存在一些隐瞒症状的不自觉者，考虑到新冠病毒的强大传播性，稍有疏忽，真的是前功尽弃啊。　　这一点，万万大意不得啊！　　（二）　　疫情在发生重大变化。　　数据冷冰冰，显示了残酷的现实。　　就这几天，海外确诊人数突破了2万例，已经大大高于除湖北以外的中国所有省份总和。　　这讲的是存量！更重要的，其实是增量，以3月6日例：　　这一天，全球新增病例至少3000多例。　　这一天，中国新增病例99例。　　中国境外是中国境内的30倍。　　这说明什么？　　印证了WHO之前的判断，现在世界最担心的，已经不是中国，而是中国之外！　　对中国来说，既要打好武汉最后的保卫战，下一步更严峻的挑战，则是应对好输入性传播。　　这也是钟南山院士的担忧。他曾说过，新冠肺炎疫情在国外迅速蔓延，中国存在从输出病例变为输入病例的可能性。　　现在，他担心的事情，已经在发生了。　　如果仔细看最近的新增病例，武汉除外，相当部分都是输入性病例。　　6日，全国除湖北以外地区，新增确诊25例，其中有24例为境外输入；前一日，湖北以外新增的17例确诊中，有16例为境外输入，占据了绝对的大多数。"));
//    println!("{} {} {}",long_text.text,long_text.bytes_count(),long_text.chars_count());
//    println!("{:?}",long_text);

    let sy_time = SystemTime::now();
    let count = 10000;
    for _ in 0..count {
        let sents = long_text.split("。?？!！".to_string(), ",，".to_string(), 64);
    }
//    println!("{:?},{:?}", SystemTime::now().duration_since(sy_time).unwrap().as_micros(),1);
//    std::thread::sleep(Duration::from_millis(5));

//    for sent in &sents{
//        println!("{}",sent)
//    }
//    println!("{} {}",&long_text.text.len(), &sents.len());
    println!("{:?}us/call", sy_time.elapsed().unwrap().as_micros()/count);
}
