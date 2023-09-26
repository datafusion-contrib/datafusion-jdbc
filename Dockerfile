FROM --platform=linux/arm64 rust:1.70 as builder

# Install JDK 11
RUN apt-get update && \
    apt-get install -y openjdk-11-jdk && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN apt-get update && \
    apt-get -y install build-essential clang llvm cmake

# Set JAVA_HOME and related environment variables
ENV JAVA_HOME /usr/lib/jvm/java-11-openjdk-arm64
ENV LD_LIBRARY_PATH=$JAVA_HOME/lib/server:$LD_LIBRARY_PATH
ENV C_INCLUDE_PATH=$JAVA_HOME/include:$JAVA_HOME/include/linux:$C_INCLUDE_PATH

# Copy source code into the container
COPY . /usr/src/datafusion-jdbc
WORKDIR /usr/src/datafusion-jdbc

RUN rustup component add rustfmt

RUN cargo build --release

RUN mv /usr/src/datafusion-jdbc/target/release/datafusion-jdbc /usr/local/bin

COPY run.sh /usr/src/datafusion-jdbc/run.sh
RUN chmod +x /usr/src/datafusion-jdbc/run.sh

ENTRYPOINT ["/usr/src/datafusion-jdbc/run.sh"]

EXPOSE 50051
