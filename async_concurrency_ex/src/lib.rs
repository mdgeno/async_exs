//COPY TO ROOT FILE
/*
//use aync_concurrency_ex::functions??
use std::time::Duration;
 
fn main(){
 
        trpl::block_on(async{
                let (prod, mut consumr) = trpl::channel();
                let prod2 = prod.clone();
 
                let channel1 = async move{
                        let val_list = vec!["hi", "from", "the", "future"];
 
                        for val in val_list{
                                prod.send(val).unwrap();
                                trpl::sleep(Duration::from_millis(500)).await;
                        }
                };
 
                let channel2 = async{
                        while let Some(val) = consumr.recv().await{
                                println!("received value {val}");
                        }
                };
 
                let channel3 = async move{
                        let val_list = vec!["more", "messages", "for", "you"];
 
                        for val in val_list{
                                prod2.send(val).unwrap();
                                trpl::sleep(Duration::from_millis(1500)).await;
                        }
                };

		trpl::join!(channel1, channel2, channel3);
	});
 
}
*/
