use std::{env, path::Path};

pub fn cd(s : &Path){
   let res =  env::set_current_dir(s);
   if res.is_err() {
    println!("{:?}" , res.err())
   }
}