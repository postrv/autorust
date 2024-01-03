# # Autorust - AI Agent Communication implementation in Rust

## Introduction

This is a Rust implementation of the [AI Agent Communication](https://github.com/microsoft/autogen) known as AutoGen developed by Microsoft.

The architecture of the system is shown below:

![AutoGen Architecture](./async-advanced.png)

---

## Overview

Autorust is a Rust-based platform designed to process and execute code in a secure and efficient manner. It utilizes a microservice architecture with different agents handling specific tasks, including image analysis, code generation, execution, and validation. The system integrates with Google Cloud Functions for secure and isolated code execution.

## Features

- **GPT Vision Agent**: Analyzes images and generates specifications using an external image analysis API.
- **Coding Agent**: Translates specifications into executable code.
- **Code Interpreter Agent**: Sends code to Google Cloud Functions for execution in a secure, sandboxed environment.
- **Code Checker Agent**: Validates the execution results and provides feedback.

## Installation and Setup

To set up and run Autorust, follow these steps:

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/your-repository/autorust.git
   cd autorust
   ```

2. **Install Dependencies**:
   Make sure you have Rust and Cargo installed. Then run:
   ```bash
   cargo build
   ```

3. **Configure Google Cloud Functions**:
  - Set up a Google Cloud Function for code execution.
  - Configure the function URL in the Code Interpreter Agent.

4. **Environment Variables**:
  - Set necessary environment variables, such as API keys for image analysis and Google Cloud credentials.

5. **Run the Application**:
   ```bash
   cargo run
   ```

## Usage

- The system can be interacted with through API calls or direct function invocations within the Rust code.
- Messages are processed by different agents depending on their type and content.

## Security

- All code execution is handled in a sandboxed environment using Google Cloud Functions.
- Input validation and sanitization are performed to prevent common security vulnerabilities.

## Testing

- Unit tests can be run using Cargo:
  ```bash
  cargo test
  ```


---

### Additional Notes
- Agent-based parallelism: optimise the threading across agents based on which agent is doing more work at any given time
- Adversarial collaboration:
  - one agent creates specs,
  - another builds code
  - another runs it
  - the final one propagates feedback to the second agent for further iteration