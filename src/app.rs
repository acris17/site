use ::args;
use ::bookmarks;


pub fn start() {
    let mut app = Application::default();
    app.setup();
    app.run();
}


#[derive(Default)]
struct Application {
    key:  String,
    edit: bool,
}
impl Application {
    fn default() -> Application {
        Application {..Default::default()}
    }
    fn setup(&mut self) {
        let matches = args::match_arguments();

        // flags
        if matches.is_present("edit") {
            self.edit = true;
        }

        // arguments
        if let Some(key) = matches.value_of("KEY") {
            self.key = String::from(key);
        }
    }
    fn run(&self) {
        let mut bookmarks = bookmarks::Bookmarks::default();
        bookmarks.setup();
            
        if self.edit {
            bookmarks.edit();
        } else {
            bookmarks.open(&self.key);
        }
    }
}
