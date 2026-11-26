//Import this library to utilize the below title_content functionality that is able to take
a url address, and return the Title value of its CSS selector.

use trpl::Html;

pub async fn title_content(url: &str) -> (&str, Option<String>){
	let get_response = trpl::get(url).await;
	let response_contents = get_response.text().await;

	let title_value = match Html::parse(&response_contents).select_first("title"){
		Some(val) => Some(val.inner_html()),
	        None => panic!("not valid css selector") 
	};

	(url, title_value)
}


//COPY TO ROOT FILE
/*
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
*/
