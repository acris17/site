use ::args;
use ::browser;


pub fn start() {
    let mut app = Application::default();
    app.setup();
    app.run();
}


#[derive(Debug, Default)]
struct Application {
    key: String,
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
        if self.edit {
            browser::edit_config();
        } 
        else {
            let mut bookmarks = browser::Bookmarks::default();
            bookmarks.setup();
            bookmarks.open_site(&self.key);
        }
    }
}
