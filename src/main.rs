use app::App;
mod app;
mod game;

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
