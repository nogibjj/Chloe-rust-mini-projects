# Rust Chatbot

This is a chatbot written in Rust that uses the OpenAI API to generate responses. The chatbot engages in a conversation with the user and provides helpful, creative, and friendly responses.

## Requirements
- Rust programming language
- OpenAI API key
    - can get the API key from [here](https://platform.openai.com/account/api-keys)

## Installation

1. Set your OpenAI API key as an environment variable:
    ```
    export OPENAI_API_KEY=<your_api_key>
    ```

2. Run the chatbot:
    ```
    cargo run
    make run
    ```

## Usage

The chatbot will greet the user and ask for input. The user can then type in their response, and the chatbot will generate a response using the OpenAI API. The conversation will continue until the user types "quit" by `Ctrl + C` in command line.