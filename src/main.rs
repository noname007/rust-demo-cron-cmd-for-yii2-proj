use std::env::{args};

fn main() {
    if args().len() < 3 {
        println!("Usage： {} module_id yii2_cmd", args().nth(0).expect("executable program"));
        return;
    }
    let module_id = args().nth(1).expect("module_id");
    let cmd = args().nth(2).expect("command");

    // let kill = "ps aux|grep \"php /data0/www/htdocs/{}/yii {}\"|grep -v 'grep' || (php /data0/www/htdocs/{}/yii {} >> /data0/www/applogs/{}/{} 2>&1 &)";

    let run_cmd = format!("ps aux|grep \"php /data0/www/htdocs/{}/yii {}\"|grep -v 'grep' || (php /data0/www/htdocs/{}/yii {} >> /data0/www/applogs/{}/{} 2>&1 &)",
                           module_id, cmd, module_id, cmd, module_id, cmd.replace("/", "-"));

    println!("{}", run_cmd);

    let kill_cmd = format!("ps aux|grep \"php /data0/www/htdocs/{}/yii {}\"|grep -v 'grep'|awk '{{print $2}}' | xargs -n 1 kill -9",
                           module_id, cmd);
    println!("{}", kill_cmd);

}
