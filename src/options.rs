/*
MIT License
Copyright (c) 2023 m!haly4
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

//For colored menu
use colored::Colorize;

//For standart I/O
use std::io;

use crate::product;

//Function for choosing an option
pub fn make_choise() -> String {
    let mut choise = String::new();

    io::stdin()
        .read_line(&mut choise)
        .expect("Failed to read line");

    choise
}

//Function for input products
pub fn read_product() -> String {
    let mut tmp = String::new();

    io::stdin()
        .read_line(&mut tmp)
        .expect("failed to read line");

    tmp.to_string()
}

//Function for showing the list of products
pub fn show_list(products: &Vec<product::Product>) {
    for element in products {
        println!(" {}\n Product: {} Price: {} {}",
                 "-----------------------------------------".red(),
                 element.name,
                 element.price,
                 "\n -----------------------------------------".red());
    }
}

//Function for counting total sum
pub fn count_total_sum(products: &Vec<product::Product>, sum: &mut f64) {
    //Counting total sum
    for element in products {
        *sum += element.price;
    }

    //Printing total sum
    println!(" {}\n {} {}\n {}",
             "-----------------------------------------".red(),
             "Total sum =".yellow(),
             sum,
             "-----------------------------------------".red());

    //Counting percentage
    for element in products {
        println!(" {}\n Product: {}\n Price: {}\n Percentage of the purchase (%) : {}%\n {}",
        "-----------------------------------------".red(),
        element.name,
        element.price,
        (element.price * 100.0) / *sum,
        "\n -----------------------------------------".red());
    }
}
