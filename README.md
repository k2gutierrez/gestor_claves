# Gestor de Claves CLI

Stack tecnológico:

Lenguaje: Rust (Seguridad de memoria garantizada).

Interfaz: CLI (Command Line Interface).

Persistencia: JSON serializado.

Criptografía: ChaCha20Poly1305 (vía cocoon).

Interacción: clap (argumentos), rpassword (input seguro), arboard (portapapeles).

- Debes tener rust instalado

# Termux
- pkg update && pkg upgrade
- pkg install rust git
- pkg install libxcb termux-api
- git clone https://github.com/k2gutierrez/gestor_claves
- cd gestor_claves
- cargo run

# Para correr la app en terminal
- cargo run

# Compilar la aplicación
- cargo build --release