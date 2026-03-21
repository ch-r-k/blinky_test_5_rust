FROM rust:latest

# Install useful tools
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user (optional but recommended)
ARG USERNAME=vscode
RUN useradd -m $USERNAME

USER $USERNAME
WORKDIR /home/$USERNAME/app