FROM rust:bookworm

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="$PATH:$HOME/.cargo/bin"
RUN apt-get -y update && apt-get install -y build-essential git cmake pkg-config libncurses-dev libbz2-dev  \
    protobuf-compiler libprotoc-dev libprotobuf-dev libjansson-dev \
    automake gettext autopoint libtool jq bsdmainutils bc rs parallel \
    npm curl unzip redland-utils librdf-dev bison flex gawk lzma-dev \
    liblzma-dev liblz4-dev libffi-dev libcairo-dev libboost-all-dev \
    libzstd-dev pybind11-dev python3-pybind11 libdeflate-dev doxygen


WORKDIR /workspaces/cxx-vg

CMD [ "bash" ]
