use std::net::TcpListener;
use std::io::{Read,Write};
use std::net::TcpStream;
fn main() {
    let re= regex::Regex::new(r"^GET /mpv://(.*) HTTP/1.1").unwrap();
    
   let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
      let link =  get_link(stream,&re);
        println!("{}",link);
    }
}






fn get_link(stream:Result<TcpStream, std::io::Error>,re:&regex::Regex) -> String {
    let success ="HTTP/1.1 200 OK\r\n\r\n";
    let error ="HTTP/1.1 404 Not Found\r\n\r\n";
    match stream {
        Ok(mut stream) => {
            //read stream
            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            let req = std::str::from_utf8(&buffer).unwrap();
            let req = req.split('\n').collect::<Vec<_>>()[0];
            let link = match re.captures(&req) {
                Some(x) =>x[1].to_string(),
                None =>"".to_string()
               };
            if link.is_empty() {
                stream.write(error.as_bytes()).unwrap();
            } else {
                stream.write(success.as_bytes()).unwrap();
            }
            stream.flush().unwrap();
           link
        },
        Err(_) =>  {
            
            "".to_string()},
        
    }
}