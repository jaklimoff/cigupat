# About cigupat

Cigupat is the tool to communicate with Open AI API. For now it can send requests to `text-davinci` model. Very simple but useful way to get answers on your questions straight from the terminal.

# Installation
```bash
# Use latest night version of rust to build
rustup install nightly
rustup default nightly

# Simply build release version of the tool
cargo build --release

# Copy binary to your bin folder
cp ./target/release/cigupat /usr/local/bin/
```

# How to use
Create a token for yourself to communicate with OpenAI API [from user profile page](https://beta.openai.com/account/api-keys).
```bash
# Just provide token and your question
cigupat -t <OPENAI_TOKEN> -- "What is the meaning of life?"
```
