use yew::prelude::*;
use serde::{Deserialize, Serialize};
use base64;
// use serde_json::json;
use serde_bytes::Bytes;

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Book {
    pub book_id: String,
    pub title: String,
    pub author: String,
    pub categories: String,
    pub cover: Vec<u8>,
    pub pages: String,
    pub progress: String,
    pub file_name: String,
    pub description: String,
    pub date: String,
    pub rights: String,
    pub tags: String,
    pub identifier: String,
    pub publisher: String
}

impl Default for Book {
    fn default() -> Self {
        Book {
            book_id: String::from(""),
            title: String::from(""),
            author: String::from(""),
            categories: String::from(""),
            cover: Vec::new(),
            pages: String::from(""),
            progress: String::from(""),
            file_name: String::from(""),
            description: String::from(""),
            date: String::from(""),
            rights: String::from(""),
            tags: String::from(""),
            identifier: String::from(""),
            publisher: String::from("")
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct BooksListProps {
    pub books: Vec<Book>,
    pub on_click: Callback<Book>
}

#[function_component(BooksList)]
pub fn books_list(BooksListProps { books, on_click } : &BooksListProps) -> Html {
    let on_click: Callback<Book> = on_click.clone();
    books.iter().map(|book: &Book| {
        let on_book_select = {
            let on_click = on_click.clone();
            let book = book.clone();
            Callback::from(move |_| {
                on_click.emit(book.clone())
            })
        };
        html! {
            <div class="pysBook">
                <img src={format!("data:image/png;base64,{}", base64::encode(&book.cover))} onclick={on_book_select} />
                <h3><a href="#">{&book.title}</a></h3>
                <p>{&book.author}</p>
            </div>
        }
    }).collect()
}

#[derive(Properties, PartialEq)]
pub struct BookDetailsProps {
    pub book: Book,
}

#[function_component(BookDetails)]
pub fn book_details(BookDetailsProps { book } : &BookDetailsProps) -> Html {
    html! {
        <div class="pysBookDetails">
            <img src={format!("data:image/png;base64,{}", base64::encode(&book.cover))} />
            <h3>{&book.title}</h3>
            <p>{&book.author}</p>
            <p>{&book.description}</p>
        </div>
    }
}
