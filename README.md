# AI Agent Communication implementation in Rust

## Introduction

This is a Rust implementation of the [AI Agent Communication](https://github.com/microsoft/autogen) known as AutoGen developed by Microsoft.

## Overview
Per the diagram below, AutoGen will consist of

1. Core Module
2. LLM Communication Module
3. Agent Framework
4. Error Handling and Logging
5. Testing Module
6. API Integration
7. Async Task Management

![AutoGen Architecture](./async-advanced.png)

### Core Module
The primary application logic lives here.

### LLM Communication Module
This is where the protocols and routines allowing LLM's to communicate with one another live.

### Error Handling and Logging
Since LLM's especially remote ones such as OpenAI are prone to network errors, special attention must be paid to fault tolerance

### Testing Module (WIP)
This is where the unit tests will live.

### API Integration
This is where the API's for the various LLM's will be integrated.

### Async Task Management
This is where the async tasks will be managed.

### Advantages
- Agent-based parallelism: optimise the threading across agents based on which agent is doing more work at any given time
- Adversarial collaboration:
  - one agent creates specs,
  - another builds code
  - another runs it
  - the final one propagates feedback to the second agent for further iteration