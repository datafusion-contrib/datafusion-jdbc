FROM openjdk:11-jdk as base

# Set environment variables
ENV LD_LIBRARY_PATH=$JAVA_HOME/lib/server:$LD_LIBRARY_PATH
ENV C_INCLUDE_PATH=$JAVA_HOME/include:$JAVA_HOME/include/linux:$C_INCLUDE_PATH

# Update and install dependencies for building Rust and other required packages
RUN apt-get update && \
    apt-get -y install build-essential curl git build-essential clang

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain 1.72.0
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Hadoop 3.3.2
RUN curl -O https://downloads.apache.org/hadoop/common/hadoop-3.3.2/hadoop-3.3.2.tar.gz && \
    tar -xzvf hadoop-3.3.2.tar.gz -C /opt && \
    rm hadoop-3.3.2.tar.gz

ENV HADOOP_HOME /opt/hadoop-3.3.2
ENV PATH $PATH:$HADOOP_HOME/bin:$HADOOP_HOME/sbin
ENV LD_LIBRARY_PATH $HADOOP_HOME/lib/native:$LD_LIBRARY_PATH

# Set CLASSPATH for Hadoop
RUN echo "export CLASSPATH=$CLASSPATH:`$HADOOP_HOME/bin/hadoop classpath --glob`" >> /etc/environment

# Set the working directory to /build/, build, move the binary and finally clean up
WORKDIR /build
COPY ./src ./src
COPY ./Cargo.toml .
RUN rustup component add rustfmt
RUN cargo build
RUN mv target/debug/datafusion-jdbc /usr/local/bin
RUN rm -rf /build

WORKDIR /jdbc

COPY ./delta /delta

COPY run.sh /jdbc/run.sh
RUN chmod +x /jdbc/run.sh

ENTRYPOINT ["/jdbc/run.sh"]

EXPOSE 50051
