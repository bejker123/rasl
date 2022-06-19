#[derive(Clone)]
pub struct Paths{
    pub save_path : String,
    pub fav_users_file : String,
    pub creds_file : String,
}

#[derive(Clone)]
pub struct Creds{
    pub client_id : String,
    pub oauth : String
}