use crate::register_problem;
use std::collections::HashSet;
use std::path::Path;

register_problem!(59, "XOR decryption", problem059);

pub fn problem059() -> String {
    // Each character on a computer is assigned a unique code and the preferred standard is ASCII
    // (American Standard Code for Information Interchange). For example, uppercase A = 65, asterisk (*) = 42,
    // and lowercase k = 107.
    //
    // A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte
    // with a given value, taken from a secret key. The advantage with the XOR function is that using
    // the same encryption key on the cipher text, restores the plain text; for example, 65 XOR 42 = 107,
    // then 107 XOR 42 = 65.
    //
    // For unbreakable encryption, the key is the same length as the plain text message, and the key is made
    // up of random bytes. The user would keep the encrypted message and the encryption key in different
    // locations, and without both "halves", it is impossible to decrypt the message.
    //
    // Unfortunately, this method is impractical for most users, so the modified method is to use a password
    // as a key. If the password is shorter than the message, which is likely, the key is repeated cyclically
    // throughout the message. The balance for this method is using a sufficiently long password key for security,
    // but short enough to be memorable.
    //
    // Your task has been made easy, as the encryption key consists of three lower case characters.
    // Using cipher.txt (right click and 'Save Link/Target As...'), a file containing the encrypted ASCII codes,
    // and the knowledge that the plain text must contain common English words, decrypt the message and find
    // the sum of the ASCII values in the original text.
    let path = Path::new("data/p059_cipher.txt");
    let data: Vec<u8> = std::fs::read_to_string(path)
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut tmp_lettres = vec![' ', ',', '(', ')', '[', ']', '.', '!', '\'', ';', '"', '+', '-', '/', '*', ':'];
    tmp_lettres.extend('a'..='z');
    tmp_lettres.extend('A'..='Z');
    tmp_lettres.extend('0'..='9');

    let lettres = tmp_lettres.into_iter().map(|c| c as u8).collect::<HashSet<u8>>();

    let mut message = "".to_string();
    let a = 'a' as u8;
    let z = 'z' as u8;
    for key1 in a..=z {
        if !lettres.contains(&(data[0] ^ key1)) {
            continue;
        }
        for key2 in a..=z {
            if !lettres.contains(&(data[1] ^ key2)) {
                continue;
            }
            for key3 in a..=z {
                if !lettres.contains(&(data[2] ^ key3)) {
                    continue;
                }

                let key = vec![key1, key2, key3];
                let mut decode = string_builder::Builder::new(data.len());
                for n in 0..data.len() {
                    let c = data[n] ^ key[n % 3];
                    if !lettres.contains(&c) {
                        break;
                    }
                    decode.append(c as char);
                }
                if decode.len() == data.len() {
                    message = decode.string().unwrap();
                    println!("Message: {}", message);
                }
            }
        }
    }

    let result:u64 = message.chars().into_iter().map(|c| c as u64).sum();
    result.to_string()
}
