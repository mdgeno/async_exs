use std::time::Duration;
use trpl::Either;
//use std::future::Future;?

pub async fn timeout<F: Future>(fut: F, timelimit: Duration) -> Result<F::Output, Duration>{
	match trpl::select(fut, trpl::sleep(timelimit)).await{
		Either::Left(val) => Ok(val),
	        Either::Right(_) => Err(timelimit)
	}
}

//================
//COPY TO MAIN
/*
use std::time::Duration;
use async_abstraction::timeout;
 
fn main(){ 
 
        trpl::block_on(async{ 
                let fut = async{
                        trpl::sleep(Duration::from_secs(5)).await;
                        "Finally Finished"
                };
 
                let timelimit = Duration::from_secs(2);
 
                match timeout(fut, timelimit).await{
                        Ok(val) => println!("Succeeded with message {val}"),
                        Err(val) => println!("Failed after {} seconds", val.as_secs())
                };
        });
 
}
*/
