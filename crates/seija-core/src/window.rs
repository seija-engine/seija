pub trait IWindow {
    fn set_title(&mut self,str:&str);
    fn title(&self) -> &str;
}

pub struct AppWindow {
    pub inner:Box<dyn IWindow>
}

impl AppWindow {
    pub fn new(win:impl IWindow + 'static) -> AppWindow {
        AppWindow {
            inner:Box::new(win)
        }
    }

    pub fn inner(&self) -> &Box<dyn IWindow>  {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut Box<dyn IWindow> {
        &mut self.inner
    }
}

mod test {
    use crate::window::{AppWindow,IWindow};
    pub struct TestWindow {
        title:String,
    }

    impl IWindow for TestWindow {
        fn set_title(&mut self,str:&str) {
            self.title = String::from(str);
        }

        fn title(&self) -> &str {
            self.title.as_str()
        }
    }

    #[test]
    fn test_create() {
        let winit = TestWindow { title: String::from("title") };
        let mut app = AppWindow::new(winit);
        app.inner_mut().set_title("fk");
        println!("{}",app.inner().title());
    }
}