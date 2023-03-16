use yew::prelude::*;
use gloo::console::log;
mod books;
use books::{Book, BooksList};

#[function_component(Head)]
fn head() -> Html {
    html! {
        <head>
            <title>{ "pyShelf" }</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <link rel="stylesheet" href="http://" />
        </head>
    }
}

#[function_component(Nav)]
fn nav() -> Html {
    html! {
        <nav class="pysNav-horiz pysNav-main" id="pysNav-Main">
            <ul>
                <li><a href="http://">{ "Home" }</a></li>
                <li><a href="http://">{ "About" }</a></li>
                <li><a href="http://">{ "Contact" }</a></li>
            </ul>
        </nav>
    }
}

#[function_component(Footer)]
fn footer() -> Html {
    html! {
        <footer>
            <p>{ "This is the footer." }</p>
        </footer>
    }
}

#[function_component(App)]
fn app() -> Html {
    let selected_book = use_state(|| None);
    let books = vec![
        Book {
            book_id: 1,
            title: String::from("The Hobbit"),
            author: String::from("J.R.R. Tolkien"),
            categories: String::from("Fantasy"),
            cover: Vec::new(),
            pages: 310,
            progress: 0.0,
            file_name: String::from("The Hobbit.epub"),
            description: String::from("Bilbo Baggins is a hobbit who enjoys a comfortable, unambitious life, rarely traveling any farther than his pantry or cellar. But his contentment is disturbed when the wizard Gandalf and a company of dwarves arrive on his doorstep one day to whisk him away on an adventure."),
            date: String::from("2019-12-01"),
            rights: String::from("Public Domain"),
            tags: String::from(""),
            identifier: String::from(""),
            publisher: String::from("")
        }
    ];
    let on_book_select: Callback<Book> = {
        log!("on_book_select");
        let selected_book = selected_book.clone();
        Callback::from(move |book: Book| {
            selected_book.set(Some(book));
        })
    };

    let details = selected_book.as_ref().map(|book| {
        html! {
            <div class="pysBookDetails">
                <img src={format!("data:image/png;base64,{}", base64::encode(&book.cover))} />
                <h3>{&book.title}</h3>
                <p>{&book.author}</p>
                <p>{&book.description}</p>
            </div>
        }
    });

    html! {
        <>
            <Head />
            <Nav />
            <div class="pysNav-horiz pysNav-main" id="pysNav-Main">
            <BooksList books={books} on_click={on_book_select.clone()}/>
            { for details }
            </div>
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
