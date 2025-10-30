use std::{io::{stdin}};

const ACCOUNTS: [&str; 2] = ["user@mail.fr", "admin@mail.fr"];
const PASSWORD: &str = "secret";
const MAX_LOGIN_ATTEMPTS: u8 = 3;

fn main() {
    println!("Login to your account :");

    for i in 1..MAX_LOGIN_ATTEMPTS + 1 {
        let mut email = String::new();
        let mut password = String::new();

        println!("Email address :");
        stdin()
            .read_line(&mut email)
            .expect("Error while reading email address");

        let mut is_account_valid = false;
        for account in ACCOUNTS {
            let account = String::from(account);
            let email = email.trim();

            if email == account {
                is_account_valid = true;
                break;
            } else {
                is_account_valid = false;
                continue;
            }
        }

        if is_account_valid == false {
            println!("{}", failed_login_error(MAX_LOGIN_ATTEMPTS - i));
            continue;
        } else {
            println!("Password :");
            stdin()
                .read_line(&mut password)
                .expect("Error while reading password");

            let is_correct = if password.trim().eq(&String::from(PASSWORD)) { true } else { false };

            if is_correct == true {
                println!("Login successfull");
                break;
            } else {
                println!("{}", failed_login_error(MAX_LOGIN_ATTEMPTS - i));
            }
        }

        if i.eq(&MAX_LOGIN_ATTEMPTS) == true {
            println!("Too many login attemps. Please retry later");
            break;
        }
    }
}

fn failed_login_error(remaining_attemps: u8) -> String {
    let error = format!("Failed to login. Remaining login attemps : {}", remaining_attemps);

    error
}