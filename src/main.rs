use yew::prelude::*;
use gloo::console::log;
use gloo_net::http::Request;
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
    let books = use_state(|| vec![]);
    {
        let books = books.clone();
        use_effect_with_deps(move |_| {
            let books = books.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_books: Vec<Book> = Request::get("http://localhost:8080/books")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                log!("{:?}", fetched_books); // Print the response to the console
                books.set(fetched_books);
            });
            || ()
        }, ());
    }
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
            <BooksList books={(*books).clone()} on_click={on_book_select.clone()}/>
            { for details }
            </div>
            <Footer />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
