# proyecto-matecomp

proyecto-matecomp is our final proyect in which we use python and rust to solve functions with different methods. 

The methods we created/implemented are Bisection method, Newton-Raphson method, Newton-Raphson Hybrid method, and secant method.

## Installation

Make sure you have the required dependencies installed. You can install them using the following commands:

´´´python
pip install -r requirements.txt
´´´
Ensure that you have Rust installed to compile and run the Rust files. You can install Rust from https://www.rust-lang.org/.

## Usage

### Python Program
To run the main Python program, execute the following command:
´´´python
streamlit run main.py
´´´
Visit the provided URL (usually http://localhost:8501) in your web browser to interact with the Streamlit app.
### Rust Programs
To run the Rust programs in the src/bin directory, navigate to the project root and use the following commands:

Bisection method
´´´bash
cargo run --bin bis
´´´
Newton-Raphson method
´´´bash
cargo run --bin nr
´´´
Newton-Raphson Hybrid method
´´´bash
cargo run --bin nrh
´´´
Secant method
´´´bash
cargo run --bin sec
´´´

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

