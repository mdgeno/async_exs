use std::thread;
use std::time::Duration;

pub fn slow(name: &str, duration: u64){
	thread::sleep(Duration::from_millis(duration));
	println!("{name} ran for {duration}ms");
}

//=====================
//COPY TO MAIN
//=====================
/*
use await_ex::slow;
use std::time::Duration;
use trpl;
 
fn main(){ 
 
        trpl::block_on(async{ 
                let fut_a = async{  
                        println!("a started");
                       	slow("a", 30);
                        trpl::yield_now().await;
                        slow("a", 10);
                        trpl::yield_now().await;
                        slow("a", 20);
                        trpl::yield_now().await;
                        println!("a finished");
                };
 
                 let fut_b = async{ 
                        println!("b started");
                        slow("b", 75);
                        trpl::yield_now().await;
                        slow("b", 10);
                        trpl::yield_now().await;
                        slow("b", 15);
                        trpl::yield_now().await;
                        slow("b", 350);
                        trpl::yield_now().await;
                        println!("b finished");
                };      
             
                trpl::join(fut_a, fut_b).await;
        });
 
}
*/
