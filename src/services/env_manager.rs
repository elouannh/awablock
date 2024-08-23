use std::env::var;

pub fn get_env_var_as_str<'a>(var_name: &'a str, default: &'a str) -> String
{
    match var(var_name)
    {
        Ok(value) =>
        {
            print!("{:?}", value);
            String::from(value)
        },
        Err(_) =>
        {
            print!("{:?}", default);
            String::from(default)
        },
    }
}
