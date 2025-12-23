use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use reqwest::Url;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("bad http response: {0}")]
    BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand{
    url:Url,
    extract_link: bool,
}


fn visit_page(client:&Client, command: &CrawlCommand)->Result<Vec<Url>,Error>{
println!("checking {:#}",command.url);
let response= client.get(command.url.clone()).send()?;
if !response.status().is_success(){
    return Err(Error::BadResponse(response.status().to_string()));
}
let mut links_url=Vec::new();

if !command.extract_link{
    return Ok(links_url);
}

let base_url=response.url().to_owned();
let body_text=response.text()?;
let document=Html::parse_document(&body_text);

let selector= Selector::parse("a").unwrap();

let href_values=document.select(&selector).
filter_map(|element| element.value().attr("href"));
 for href in href_values{
    match base_url.join(href) {
        Ok(link_url)=>{
            links_url.push(link_url);
        }
        Err(err)=>{
            println!("On {base_url}: ignore umparsable {href:?}: {err}")
        }
        
    }
 }

 Ok(links_url)

}


struct CrawlState{
    domain:String,
    visited_pages:std::collections::HashSet<String>,
}


impl  CrawlState  {

    fn new(start_url: &Url)->CrawlState{
        let mut visited_pages=std::collections::HashSet::new();
        visited_pages.insert(start_url.as_str().to_string());
        CrawlState { domain: start_url.domain().unwrap().to_string(), visited_pages }
    }
    
    fn shoul_extract_links(&self, url: &Url)->bool{
        let Some(url_domain)=url.domain() else{
            return false;
        };
        url_domain==self.domain
    }

    fn mark_visited(&mut self, url: &Url)->bool{
        self.visited_pages.insert(url.as_str().to_string())
    }
    
}

type CrawlResult = Result<Vec<Url>, (Url,Error)>;

fn spawn_crawler_thread(
    command_receiver: mpsc::Receiver<CrawlCommand>,
    result_sender:mpsc::Sender<CrawlResult>,
    thread_count:u32,
){
    let command_receiver=Arc::new(Mutex::new(command_receiver));

    for _ in 0..thread_count{

        let result_sender=result_sender.clone();
        let command_receiver=Arc::clone(&command_receiver);

        thread::spawn(move||{
            let client=Client::new();
            loop{

                let command_result={
                    let receiver_guard=command_receiver.lock().unwrap();
                    receiver_guard.recv()
                };

                let Ok(crawl_command) = command_result else {
                    break;
                };
                let crawl_result= match visit_page(&client, &crawl_command){
                    Ok(link_urls)=>Ok(link_urls),
                    Err(error)=> Err((crawl_command.url,error)),

                };

                result_sender.send(crawl_result).unwrap();
            }
        });
    }
}

fn control_crawl(
    start_url:Url,
    command_sender:mpsc::Sender<CrawlCommand>,
    result_receiver:mpsc::Receiver<CrawlResult>,
    )->Vec<Url>{
        let mut crawl_state=CrawlState::new(&start_url);
        let start_command=CrawlCommand{
            url:start_url,
            extract_link:true
        };
        command_sender.send(start_command).unwrap();

        let mut pending_url=1;
        let mut bad_urls=Vec::new();
        while pending_url>0{
            let crwal_result=result_receiver.recv().unwrap();        
             pending_url -=1;

             match crwal_result{
                Ok(link_urls)=>{
                    for url in link_urls{
                        if crawl_state.mark_visited(&url){
                            let extract_link=crawl_state.shoul_extract_links(&url);
                            let crawl_command=CrawlCommand{
                                url,extract_link
                            };
                            command_sender.send(crawl_command).unwrap();
                            pending_url +=1;
                        }
                    }
                }
                Err((url,error))=>{
                     bad_urls.push(url);
                     println!("Got crawling error: {:#}",error);
                     continue;
                }
             }
        
        }
        bad_urls
    
    }


    fn check_links(start_url:Url)->Vec<Url>{
        let (result_sender,result_receiver)=mpsc::channel::<CrawlResult>();
        let (command_sender,command_receiver)=mpsc::channel::<CrawlCommand>();
        spawn_crawler_thread((command_receiver), result_sender, 16);
        control_crawl(start_url, command_sender, result_receiver)

    }


fn main() {
   let start_url=reqwest::Url::parse("https://www.google.org").unwrap();
   let bad_url=check_links(start_url);
   println!("Bad Urls: {:#?}", bad_url);


}
