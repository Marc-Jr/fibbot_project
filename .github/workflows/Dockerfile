# Use the official Rust image from Docker Hub
FROM rust:latest

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the working directory
COPY Cargo.toml Cargo.lock ./

# Install dependencies
RUN cargo build --release

# Copy the rest of the project files to the working directory
COPY . .

# Build the project
RUN cargo build --release

# Set the command to run the project
CMD ["cargo", "run"]