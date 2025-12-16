# Stage 1: Build
FROM rust:alpine as builder
WORKDIR /app
# Install dependencies for compilation
RUN apk add --no-cache musl-dev

# Create empty shell project to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Build actual app
COPY . .
# Touch main.rs to force rebuild with new code
RUN touch src/main.rs 
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:3.19
WORKDIR /app
# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/monitor-status-backend .

# Expose port
EXPOSE 3000

# Run the binary
CMD ["./monitor-status-backend"]