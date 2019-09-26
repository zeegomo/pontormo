FROM rust:1.37

COPY . .
RUN rustup install nightly
RUN cargo +nightly build --release

# Make port 80 available to the world outside this container
EXPOSE 80

# Run pontormo when the container launches
CMD /target/release/pontormo
