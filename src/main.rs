/**
 * Twelve Days of Christmas 
 * Creates the lyrics of the song "Twelve Days of Christmas"
 * making use of the repetition in the song and automating a lot of that 
 * using Arrays and other programatic features 
 * 
 * For lyrics of the song: http://www.metrolyrics.com/12-days-of-christmas-lyrics-christmas-song.html
 * Created by Hannan Ali as part off the learning through Rust book
 *
 * main.rs
 * 
 *
 */

extern crate term;
extern crate colored;

use std::io::prelude::*;
use colored::*;

const TRUE_LOVE_SURPRISES : [&'static str ; 12] = [
    "Partridge in a Pear Tree",
    "Turtle Doves",
    "French Hens",
    "Calling Birds",
    "Golden Rings",
    "Geese a Laying",
    "Swans a Swimming",
    "Maids a Milking",
    "Ladies Dancing",
    "Lords a Leaping",
    "Pipers Piping",
    "Drummers Drumming",
];

const ORDINALS : [&'static str ; 15] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
    "thirteenth",
    "fourteenth",
    "fifteenth"
];

const NUMBERS_IN_WORDS : [&'static str ; 15] = [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteeen",
    "fifteen"
];

fn main() {
    display_song_info();

    const FIRST_SENTENCE : &'static str = "On the _ day of Christmas my true love sent to me:";
    let total_surprises = TRUE_LOVE_SURPRISES.len();

    for i in (0..total_surprises) {
        let current_ordinal = ORDINALS[i];
        let current_ordinal_sentence = FIRST_SENTENCE.replace("_", current_ordinal);

        println!("\n============================== Day {} ==============================", i + 1);
        println!("\n{}", current_ordinal_sentence);

        for j in (0..i + 1).rev() {
            let mut prefix = String::new();
            let surprise = TRUE_LOVE_SURPRISES[j];
            
            if j == 0 && i == 0 {
                prefix.push_str("a");
            } else if j == 0 {
                prefix.push_str("and a");
            } else {
                let capitalized_number_word = capitalize_word(NUMBERS_IN_WORDS[j]);
                prefix = capitalized_number_word;
            }

            println!("{} {}", prefix, surprise);
        }
    }

    println!("\n=================================\n");
}

/**
  * Capitalizes first letter of the given string
  */
fn capitalize_word(word: &str) -> String {
    let first_char = word.get(0..1).unwrap();
    let rest_part = word.get(1..).unwrap();
    
    first_char.to_uppercase() + rest_part
}

/**
  * Introductory song information
  */
fn display_song_info() {
    const SONG_NAME : &'static str = "Twelve Days of Christmas";
    const YEAR_PUBLISHED : u16 = 1780;
    const LISTEN_LINK : &'static str = "https://youtu.be/oyEyMjdD2uk";

    println!("\n\n-----------------------------------------------------------");
    println!("\n\t\t{} ({})", SONG_NAME, YEAR_PUBLISHED);
    println!("\n\tListen this song at {}", LISTEN_LINK);
    println!("\n\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~")
}