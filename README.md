# Arma Manager

Arma Manager is a tool for managing Arma 3 servers. It provides a web interface
for managing mods, missions, and server settings.

This project is still a work in progress. I've made this public for other people
to look at and learn how to use leptos from a real-world application, although
I'm probably not using best-practices everywhere, I'll gladly take feedback.

> NOTE: Currently only Windows is supported as a platform, there is an open
> issue to add linux support. This is however not a priority for me personally.

## Installation

### From Git

1. Clone the repository
2. Install Rust and Cargo
3. Install Node.js and npm
4. Run `npm install` in the `crates/dashboard` directory
5. Creata a new `db.sqlite` file in the root of the repository
6. Run `sqlx migrate run --database-url=sqlite:../../db.sqlite` in the
   `crates/web` directory
7. Copy `.env.example` to `.env`
8. Configure `.env`
   - JWT_SECRET needs to be filled in with a random string.
   - STEAM_USERNAME
   - STEAM_PASSWORD
     - SteamGuard is not implemented right now, so that needs to be disabled on
       the account used
9. Run `cargo +nightly leptos watch` in the root directory
10. In a seperate terminal run
    `npx tailwindcss -i ./style/input.css -o ./style/output.scss --watch` in the
    `crates/dashboard` directory

### From Releases (todo)

## Usage

1. Open a web browser and navigate to `http://127.0.0.1:3000`
2. Use the web interface to manage your Arma 3 server

## Contributing

Contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.
