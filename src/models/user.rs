use chrono::{NaiveDate};
use serde::{Serialize, Deserialize};
use super::post::Post;
use super::comments::Comments;
#[derive(Serialize, Deserialize)]
pub(crate) struct User{
    id:i32,
    name: String,
    surname: String,
    nickname: String,

    birthday_date: NaiveDate,
    status: String,

    posts: Vec<Post>,
    comments: Vec<Comments>,


}
impl User{
    pub fn new (id:i32, name: String,surname:String, nickname: String, birthday_date:NaiveDate, status:String )->Self {
        Self {id, name, surname, nickname, birthday_date, status, posts:Vec::new(), comments:Vec::new()}
    }
    pub fn add_post(&mut self, text:String,){
        //здесь явно проблема, при создании бд, надо будет переписать
    //                                              ↓
        let post = Post::new(text, self.id, self.id);
        self.posts.push(post);

    }
    pub fn update_post(&mut self, text:String, post_id:usize){
        //возможно id будет именно индивидуальным номер
        if let Some(post) = self.posts.get_mut(post_id){
            post.update(text)
        }else{
            eprintln!("Post not found")
        }
    }
    pub fn add_comment(&mut self, text: String, post_id: usize) {
        if let Some(post) = self.posts.get_mut(post_id) {
            let comment = Comments::new(self.id, post_id as i32, text);
            post.comments.push(comment);
        } else {
            eprintln!("Post not found");
        }
    }
}