# Rust Intermediate Project 002 - Practical Implementation Plan

## Project Name
Shared Service Configuration with Rc

## Goal
Build a tiny single-threaded Rust application where multiple service structs all share ownership of one application configuration using `Rc<T>`.

## File structure
Keep it small.

- `main.rs`
- `config.rs`

## Step 1 - Design the shared config
Create an `AppConfig` struct.

Use a few realistic fields such as:
- environment
- database_url
- retry_limit
- feature_flag_enabled

The exact field names are not the important part.
The important part is that it feels like one meaningful shared object.

## Step 2 - Design the services
Create small service structs such as:
- `AuthService`
- `PaymentsService`
- `ApiGateway`

Each service should store a handle to the shared config.

That means each service should hold:
- `Rc<AppConfig>`

## Step 3 - Build one config in main
In `main.rs`, create one config value.

Do not create separate configs for each service.
That would defeat the whole point.

## Step 4 - Wrap the config in Rc
Wrap the config in `Rc<T>`.

Now you have one shared-ownership handle.

## Step 5 - Clone the handles for each service
Create each service by giving it an `Rc::clone(...)` of the shared config handle.

This is the key learning moment.

Important idea:
You are not cloning the full config deeply.
You are creating another owner of the same underlying config value.

## Step 6 - Add simple service behavior
Give each service a small method that prints what it sees from config.

For example, each service can print:
- its service name
- environment
- retry limit
- whether the feature flag is enabled

This makes the shared config visible in output.

## Step 7 - Run all service summaries
From `main`, call the display or summary method on each service.

This should make it obvious that all services are using the same config.

## Step 8 - Reflect on the ownership model
After the project works, verify that you understand:
- one config was created
- many owners were created through `Rc::clone()`
- services did not receive independent copied configs

## Optional tiny extension
You can add one more service such as `NotificationService`.

This is useful because it reinforces the same lesson again:
creating a new owner is easy and cheap.

## What not to add
Do not add mutation.
Do not add threads.
Do not add runtime borrow tricks.

This project should remain cleanly focused on shared ownership.

## What you should be able to explain after building it
- why plain ownership was not enough
- why `mut` was irrelevant to the core problem
- why `Rc<T>` solved the problem cleanly
- what `Rc::clone()` really did
