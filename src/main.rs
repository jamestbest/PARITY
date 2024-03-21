#![allow(non_snake_case)]

use std::fs::read_to_string;
use std::io::{BufWriter, stdin, Write};
use std::fs::File;

use serde::{Deserialize, Serialize};
use crate::Choice::{INVALID, NEW};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Review {
    id: i32,
    rating: f64,
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Book {
    book_id: i32,
    user_rating: i32,
    title: String,
}

fn doesnt_have_review(book: &Book, reviews: &Vec<Review>) -> Option<Review> {
    reviews.iter().find(|rev| rev.id == book.book_id).cloned()
}

fn get_book_w_id(books: &Vec<Book>, id: i32) -> Option<Book>{
    books.iter().find(|book| book.book_id == id).cloned()
}

fn ask_for_review(buffer: &mut String) -> Option<f64> {
    buffer.clear();
    stdin().read_line(buffer).unwrap();

    let b = buffer.lines().next().unwrap();

    match b.parse() {
        Ok(v) => {
            if v < 0.0 || v > 5.0 {
                println!("Value must be between 0 and 5");
                None
            }
            else {
                Some(v)
            }
        }
        Err(err) => {
            println!("{}", err);
            None
        }
    }
}

fn get_review(book: &Book) -> Review {
    let mut buffer = String::new();

    let rating= loop {
        println!("title:{}\nOriginal Rating: {}\nPlease enter the rating:", book.title, book.user_rating);
        match ask_for_review(&mut buffer) {
            Some(r) => {
                break r
            }
            None => {

            }
        }
    };

    let rev = Review{id: book.book_id, rating, title: book.title.to_string() };

    return rev
}

#[derive(PartialEq)]
enum Choice {
    NEW,
    UPDATE,
    VIEW,
    CLEAR,
    EXIT,
    INVALID
}

fn is_valid_choice(buffer: &String) -> Choice {
    return match buffer.lines().next().unwrap() {
        "new" => {
            Choice::NEW
        }
        "update" => {
            Choice::UPDATE
        }
        "view" => {
            Choice::VIEW
        }
        "clear" => {
            Choice::CLEAR
        }
        "exit" => {
            Choice::EXIT
        }
        _ => Choice::INVALID
    }
}

const REVIEW_PATH: &str = "../api/data/NewReview.txt";
const GR_PATH: &str = "../api/data/goodreads.txt";


fn get_reviews() -> Vec<Review> {
    let rev_data: String = read_to_string(&REVIEW_PATH).expect("Unable to read review data");
    let mut reviews: Vec<Review> = serde_json::from_str(&*rev_data).unwrap();

    reviews.sort_by(|r, r2| r2.rating.partial_cmp(&r.rating).unwrap());

    return reviews
}

fn get_gr() -> Vec<Book> {
    let gr_data: String = read_to_string(GR_PATH).expect("Unable to read goodreads data file");
    let books: Vec<Book> = serde_json::from_str(&*gr_data).expect("Unable to parse goodreads data");

    return books;
}

fn new() {
    let books = get_gr();
    let reviews = get_reviews();

    let mut new_reviews: Vec<Review> = Vec::new();

    for book in books {
        new_reviews.push(match doesnt_have_review(&book, &reviews) {
            Some(r) => r,
            None => get_review(&book),
        });
    }

    let file = File::create(&REVIEW_PATH).unwrap();
    let write = BufWriter::new(file);

    serde_json::to_writer_pretty(write, &new_reviews).expect("TODO: panic message");

    print_reviews(&reviews);
}

fn print_reviews(reviews: &Vec<Review>) {
    let mut i: i32 = 0;

    reviews.iter().for_each(|rev| {println!("{}: {} rating: {}", i, rev.title, rev.rating); i+=1});
}

fn update() {
    let mut reviews: Vec<Review> = get_reviews();
    let books: Vec<Book> = get_gr();

    print_reviews(&reviews);

    let mut buffer = String::new();
    let index = loop {
        buffer.clear();
        println!("Please enter the index of the book to update");
        stdin().read_line(&mut buffer).unwrap();

        match buffer.lines().next().unwrap().parse::<usize>() {
            Ok(v) => {
                if v > 0 && v < reviews.len() {break v}
                else {println!("Index out of range")}
            }
            Err(e) => {println!("{}", e)}
        }
    };

    let rev = &mut reviews[index];
    let book = get_book_w_id(&books, rev.id);

    println!("You have selected: {}\nOriginal rating: {}\nNew rating: {}", rev.title, book.unwrap().user_rating, rev.rating);

    let mut buffer = String::new();
    let rating= loop {
        println!("Please enter the new rating of the book to update");
        match ask_for_review(&mut buffer) {
            Some(r) => {
                break r
            }
            None => {

            }
        }
    };

    rev.rating = rating;

    let file = File::create(&REVIEW_PATH).unwrap();
    let write = BufWriter::new(file);

    serde_json::to_writer_pretty(write, &reviews).expect("TODO: panic message");
}

fn clear() {
    println!("You are about to remove all reviews are you sure? (Y/n)");

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    match buffer.lines().next().unwrap() {
        "Y" => {}
        _ => {return;}
    }

    let file = File::create(&REVIEW_PATH).unwrap();
    let mut write = BufWriter::new(file);
    write.write("[]".as_ref()).expect("TODO: panic message");

    println!("Removed all reviews");
}

fn view() {
    let reviews: Vec<Review> = get_reviews();

    print_reviews(&reviews);
}

fn choice() -> bool {
    let mut buffer = String::new();

    let choice = loop  {
        buffer.clear();
        println!("Please enter choice (new/update/view/clear/exit): ");
        stdin().read_line(&mut buffer).unwrap();
        let c = is_valid_choice(&buffer);

        if c != INVALID {break c};
    };

    match choice {
        NEW => {new()}
        Choice::UPDATE => {update()}
        Choice::VIEW => {view()}
        Choice::CLEAR => {clear()}
        Choice::EXIT => {return false;}
        _ => {}
    }

    return true;
}

fn main() {
    println!("Hello, world!");
    while choice() {};
}