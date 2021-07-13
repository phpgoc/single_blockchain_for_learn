mod structs;
mod global;
mod algorithm;


pub  fn do_it(){
    loop{
        println!("{:?}",algorithm::block::generate_block());
    }
 //    println!("{}",U256::MAX);
 //    let one = U256::one();
 //    let two = U256::from("123");
 //
 //    println!("{}",two  >= one);
 // // let now = Instant::now();
 // //   std::thread::sleep(Duration::from_secs(1));
 // //    global::MILLISECOND_COUNTER.fetch_add(1,Ordering::SeqCst);
 //    global::HARD.store(10000,Ordering::SeqCst);
 //       println!("{:?}", global::HARD.load(Ordering::Relaxed));

    //    println!("{:?}", global::MILLISECOND_COUNTER.load(Ordering::Relaxed));
 //    println!("milliseconds = {}",now.elapsed().as_millis());


}