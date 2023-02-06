# Rust Commands

1. Compile Rust file
    ```
    $ rustc example1.rs 
    ```

2. Setup New Project with cargo (Project name must be in snake case)
    ```
    $ cargo new proj_example
    ```

3. Build the project i.e. compile the project only
    - Note: it will compile all the rust files into debug folder
    ```
    $ cargo build
    ```

4. Build the project for production
    - Note: use this command for final build that is ready for the production and stored in target/released
    ```
    $ cargo build --release
    ```