use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn save_msg(msg: String) {
    CHAT.with(|msgs| msgs.borrow_mut().push(msg));
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    CHAT.with(|msgs| msgs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}