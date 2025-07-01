use std::io::{self, Write};

pub fn ask_yes_or_no(promt: &str) -> bool {
    loop {
        print!("{} (y/n)", promt);
        io::stdout().flush().expect("Error al vaciar STDOUT");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("No se pudo leer input.");

        let input = input.trim().to_lowercase();
        match input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Ingresa 'y' (sí) o 'n' (no)")
            }
        }
    }
}
