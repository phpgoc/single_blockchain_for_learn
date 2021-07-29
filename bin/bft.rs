use clap::{App, Arg};
use rand::Rng;
use std::process::exit;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::{Duration, SystemTime};
use threadpool::ThreadPool;

static mut GENERAL_NUMBER: usize = 20;
static mut CHANNEL_VEC: Vec<Sender<usize>> = vec![];
fn main() {
    unsafe { unsafe_main() }
}
unsafe fn unsafe_main() {
    let (s, g) = parse_args();
    GENERAL_NUMBER = g;
    let (tx_to_main, rx_to_main) = channel();
    for i in 0..g {
        let (tx, rx) = channel();
        CHANNEL_VEC.push(tx);
        let tx_to_main_clone = tx_to_main.clone();
        std::thread::spawn(move || handle(i < s, i, rx, tx_to_main_clone));
    }
    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let command: usize = rng.gen_range(0..=1);
        for j in CHANNEL_VEC.iter() {
            j.send(command).unwrap();
        }
        println!("round {}", i);
        println!("main send command {}", command);
        let mut sum = 0;
        let now = SystemTime::now();
        for _ in 0..GENERAL_NUMBER {
            sum += rx_to_main.recv().unwrap();
        }
        println!(
            "main recv command {}",
            if sum * 2 < GENERAL_NUMBER { 0 } else { 1 }
        );
        println!("消耗时间 {:?}", now.elapsed().unwrap());
    }
}
unsafe fn handle(spy: bool, num: usize, rx: Receiver<usize>, tx_to_main: Sender<usize>) {
    let pool = ThreadPool::new(5000/(GENERAL_NUMBER+10).min(GENERAL_NUMBER));
    //由于操作系统限制，这个开启不了太高，如果线程数量可以设置得很高，时间基本等于随机数的上限

    let mut rng = rand::thread_rng();
    let mut vec = vec![];
    while let Ok(t) = rx.recv() {
        if t < 10 {
            if spy {
                vec.push(1 - t);
            } else {
                vec.push(t);
            }
            for j in 0..GENERAL_NUMBER {
                if num == j {
                    continue;
                }
                let rand_millis: u64 = rng.gen_range(100..=900);
                pool.execute(move || {
                    std::thread::sleep(Duration::from_millis(rand_millis));
                    if spy {
                        CHANNEL_VEC[j].clone().send((num + 1) * 10 + 1 - t).unwrap();
                    } else {
                        CHANNEL_VEC[j].clone().send((num + 1) * 10 + t).unwrap();
                    }
                });
            }
        } else {
            vec.push(t % 10);
            if vec.len() == GENERAL_NUMBER {
                let to_main = if vec.iter().sum::<usize>() * 2 < GENERAL_NUMBER {
                    0
                } else {
                    1
                };
                vec.clear();
                match tx_to_main.send(to_main){
                     Ok(_) =>{}
                     Err(e) => {
                         print!("error : {}",e);
                         exit(1);
                     }
                }
            }
        }
    }
}
fn parse_args() -> (usize, usize) {
    let matches = App::new("bft")
        .version("1.0")
        .author("zxq <phpgoc@gmail.com>")
        .about("拜占庭容错")
        .arg(
            Arg::with_name("spy")
                .short("s")
                .long("spy")
                .value_name("spy")
                .help("奸细数量")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("general")
                .short("g")
                .long("general")
                .value_name("general")
                .help("将军数量")
                .takes_value(true),
        )
        .get_matches();
    let spy = match matches.value_of("spy") {
        Some(t) => match t.parse::<usize>() {
            Ok(n) => n,
            Err(_) => 3,
        },
        None => 3,
    };
    let general = match matches.value_of("general") {
        Some(t) => match t.parse::<usize>() {
            Ok(n) => n,
            Err(_) => 10,
        },
        None => 10,
    };
    if spy * 3 >= general {
        print!("奸细数量需要小将军数量的1/3");
        exit(1);
    }
    if general >= 1000 {
        print!("将军数量太多，测试程序无法承受");
        exit(1);
    }
    if spy == 0 {
        print!("没有奸细，测试无意义");
        exit(1);
    }
    (spy, general)
}
