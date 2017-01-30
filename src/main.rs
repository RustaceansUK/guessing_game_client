extern crate hyper;

use hyper::client::*;
use std::io::prelude::*;
use std::thread;
use std::time;

fn get_contents(num : i32) -> String {
    let client = Client::new();

    let mut res = client.get(&format!("http://efb26cd2.ngrok.io/guess/{}",
                                      num)).send().unwrap();
    assert_eq!(res.status, hyper::Ok);

    let mut output = String::new();
    res.read_to_string(&mut output).unwrap();
    return output;
}

fn main() {
    let mut guess = 100;
    let mut direction = 0;
    loop {
        println!("{}", guess);
        let result = get_contents(guess);
        if result == "bigger" {
            if direction == 1 {
                guess = guess - 1;
            }
            else {
                guess = guess - 10;
                direction = -1;
            }
        }
        else if result == "smaller" {
            if direction == -1 {
                guess = guess + 1;
            }
            else {
                guess = guess + 10;
                direction = 1;
            }
        }
        else {
            println!("Success: {}", guess);
        }
        
        let sleep_time = time::Duration::from_millis(1000);
        thread::sleep(sleep_time);
    }

}
