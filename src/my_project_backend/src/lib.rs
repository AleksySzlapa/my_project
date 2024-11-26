use std::cell::RefCell;

thread_local! {
    static CHAT: RefCell<Vec<String>> = RefCell::new(Vec::new());
}


#[ic_cdk::update]
fn save_chat(input_chat: String) {
    CHAT.with(|chat|chat.borrow_mut().push(input_chat))
}

#[ic_cdk::update]
fn del_last_chat() {
    CHAT.with(|chat| {
        let mut chat_vec = chat.borrow_mut();
        chat_vec.pop(); // Removes the last element, if it exists
    });
}

#[ic_cdk::query]
fn get_chat()  -> Vec<String> {
    CHAT.with(|chat| chat.borrow().clone())
}
