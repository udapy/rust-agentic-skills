# Use the official Rust image as a parent image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/rust-agentic-skills

# Copy the current directory contents into the container at /usr/src/rust-agentic-skills
COPY . .

# Install Python and Gemini CLI for extension testing
RUN apt-get update && apt-get install -y python3 python3-pip
# Attempt to install the CLI tool. 
# Note: User specified 'install gemini-cli'. 
RUN pip3 install gemini-cli --break-system-packages

# Build the application
# We use --release for a production-like build
RUN cargo build --release


# The executable will be in target/release/rust-agentic-skills
# Ensure the wrapper and demo scripts are executable
RUN chmod +x run_rust_guardian.sh bootstrap.sh local_demo_gemini.sh

# Use the demo script as the default command to verify the full flow
CMD ["./local_demo_gemini.sh"]


