use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use super::comments::Comments;
#[derive(Serialize, Deserialize, Clone)]
pub struct Post{

    id:i32,
    id_user:i32,
    text:String,
    created_at:DateTime<Utc>,
    update_at: DateTime<Utc>,
    is_update:bool,
    //Комментарии к посту от любых пользователей
    pub comments: Vec<Comments>
}
impl Post{
    pub fn new(text:String, id:i32, id_user:i32)->Self{
        let time = Utc::now();
        Self{id, id_user, text, created_at:time, update_at:time, is_update:false, comments:Vec::new()}
    }
    pub fn update(&mut self,text:String){
        self.text=text;
        self.update_at = Utc::now();
        self.is_update=true;
    }

}