extern crate rand;

use rand::prelude::*;
use rand::distributions::{Distribution, Uniform, Normal};
use std::fs::File;
use std::io;
use std::io::prelude::*;

fn gen_cph(count: usize) {
    let zm: &str = "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789ABCDEFGHJKLMNPQRSTUVWXYZ";//-IO
    let PA : [&str;408] = ["京A","京C","京E","京F","京H","京J","京K","京L","京M","京N","京P","京G","京Y","京B","京P","京Q","京O","津A","津B","津C","津D","津E","津F","津G","津H","津J","津K","津L","津O","沪A","沪B","沪D","沪E","沪F","沪G","沪H","沪L","沪J","沪K","沪C","沪R","渝A","渝B","渝C","渝F","渝G","渝H","冀A","冀B","冀C","冀D","冀E","冀F","冀G","冀H","冀J","冀R","冀T","豫A","豫B","豫C","豫D","豫E","豫F","豫G","豫H","豫J","豫K","豫L","豫M","豫N","豫P","豫Q","豫R","豫S","豫U","云A","云C","云D","云E","云F","云G","云H","云J","云L","云K","云M","云N","云P","云Q","云R","云S","辽A","辽B","辽C","辽D","辽E","辽F","辽G","辽H","辽J","辽K","辽L","辽M","辽N","辽P","辽V","黑A","黑B","黑C","黑D","黑E","黑F","黑G","黑H","黑J","黑K","黑L","黑M","黑N","黑P","黑R","湘A","湘B","湘C","湘D","湘E","湘F","湘G","湘H","湘J","湘K","湘L","湘M","湘N","湘U","皖A","皖B","皖C","皖D","皖E","皖F","皖G","皖H","皖J","皖K","皖L","皖M","皖N","皖P","皖Q","皖R","皖S","鲁A","鲁B","鲁C","鲁D","鲁E","鲁F","鲁G","鲁H","鲁J","鲁K","鲁L","鲁M","鲁N","鲁P","鲁Q","鲁R","鲁S","鲁U","鲁V","鲁W","鲁Y","新A","新B","新C","新D","新E","新F","新G","新H","新J","新K","新L","新M","新N","新P","新Q","新R","苏A","苏B","苏C","苏D","苏E","苏F","苏G","苏H","苏J","苏K","苏L","苏M","苏N","浙A","浙B","浙C","浙D","浙E","浙F","浙G","浙H","浙J","浙K","浙L","赣A","赣B","赣C","赣D","赣E","赣F","赣G","赣H","赣J","赣K","赣L","赣M","赣S","鄂A","鄂B","鄂C","鄂D","鄂E","鄂F","鄂G","鄂H","鄂J","鄂K","鄂L","鄂M","鄂N","鄂P","鄂Q","鄂R","鄂S","桂A","桂B","桂C","桂D","桂E","桂F","桂G","桂H","桂J","桂K","桂L","桂M","桂N","桂P","桂R","甘A","甘B","甘C","甘D","甘E","甘F","甘G","甘H","甘J","甘K","甘L","甘M","甘N","甘P","晋A","晋B","晋C","晋D","晋E","晋F","晋H","晋J","晋K","晋L","晋M","蒙A","蒙B","蒙C","蒙D","蒙E","蒙F","蒙G","蒙H","蒙J","蒙K","蒙L","蒙M","陕A","陕B","陕C","陕D","陕E","陕F","陕G","陕H","陕J","陕K","陕V","吉A","吉B","吉C","吉D","吉E","吉F","吉G","吉H","吉J","闽A","闽B","闽C","闽D","闽E","闽F","闽G","闽H","闽J","闽K","贵A","贵B","贵C","贵D","贵E","贵F","贵G","贵H","贵J","粤A","粤B","粤C","粤D","粤E","粤F","粤G","粤H","粤J","粤K","粤L","粤M","粤N","粤P","粤Q","粤R","粤S","粤T","粤U","粤V","粤W","粤X","粤Y","粤Z","青A","青B","青C","青D","青E","青F","青G","青H","藏A","藏B","藏C","藏D","藏E","藏F","藏G","藏H","藏J","川A","川B","川C","川D","川E","川F","川H","川J","川K","川L","川M","川O","川Q","川R","川S","川T","川U","川V","川W","川X","川Y","川Z","宁A","宁B","宁C","宁D","宁E","琼A","琼B","琼C","琼D","琼E"];
    let cstrs: [&str; 5] = [zm, zm, zm, zm, zm];
    let mut css: [Vec<char>; 5] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for i in 0..cstrs.len() {
        for c in cstrs[i].chars() {
            css[i].push(c);
        }
    }

    let filen : &str = "./cph.txt";
    let mut fout:File = File::create(filen).unwrap();

    let pa_range_uniform = Uniform::new(0usize, PA.len());

    let mut cph = String::new();
    let mut cph_set: HashSet<String> = HashSet::new();
    for x in 0..count {
        let pa_sample_uniform = pa_range_uniform.sample(&mut rand::thread_rng());

        cph.push_str(PA[pa_sample_uniform]);

        for cs in css.iter() {
            let cs_range_uniform = Uniform::new(0usize, cs.len());
            let cs_sample_uniform = cs_range_uniform.sample(&mut rand::thread_rng());

            cph.push(' ');
            cph.push(cs[cs_sample_uniform]);
        }
        cph_set.insert(cph.clone());
        if x % 1000000 == 0 {
            println!("{} {}", x, cph);

        }
        write!(fout,"{}\n",cph);
        cph.clear();
    }
    println!("{}%", cph_set.len() as f32 / count as f32 * 100f32);

}