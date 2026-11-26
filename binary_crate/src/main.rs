//To execute: cargo run -- "url address including http://" "another url address same format"

use async_ex::title_content;
use std::env;
use trpl::Either;
 
fn main() {
 
        let command_line: Vec<String> = env::args().collect();
        let url1 = &command_line[1];
        let url2 = &command_line[2];
     
        trpl::block_on(async{
                 let (site, title_val) = match trpl::select(title_content(url1), title_content(url2)).await{
                                                Either::Left(val) => val, 
                                                Either::Right(val) => val  
                                        };
        
                 println!("{site} returned first");
                 match title_val{
                         Some(val) => println!("It's page title was: {val}"),
                         None => println!("It had no title") 
                 };
         });
}
