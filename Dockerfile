# Use the official Rust image as the base image
FROM rust:slim

# Set the working directory inside the container
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . .

# Build the Rust application
RUN cargo build --release

# Set the environment variable for Rocket address
ENV ROCKET_ADDRESS=0.0.0.0

# Expose the port that Rocket will run on
EXPOSE 8000

# Run the application when the container starts
CMD ["./target/release/backend"]
