FROM rust:1-bookworm

RUN apt-get update && apt-get -yqq --no-install-recommends install unzip curl \
    python3 python3-venv git libgomp1 libopenblas-dev build-essential cmake autoconf libtool

RUN cd /opt && mkdir raxml-ng && cd raxml-ng \
    && curl -LO https://github.com/amkozlov/raxml-ng/releases/download/1.2.2/raxml-ng_v1.2.2_linux_x86_64.zip \
    && unzip raxml-ng_v1.2.2_linux_x86_64.zip \
    && cd /bin && ln -s /opt/raxml-ng/raxml-ng raxml-ng

RUN cd /opt && mkdir pythia && cd pythia \
    && python3 -m venv venv && . venv/bin/activate \
    && pip install git+https://github.com/tschuelia/PyPythia.git \
    && pip install biopython \
    && echo '. /opt/pythia/venv/bin/activate' >> /root/.bashrc

RUN cd /opt && mkdir phyml && cd phyml \
    && curl -LO https://github.com/stephaneguindon/phyml/archive/refs/tags/v3.3.20220408.zip \
    && unzip v3.3.20220408.zip \
    && cd phyml-3.3.20220408 \
    && sh ./autogen.sh \
    && ./configure \
    && make \
    && make install \
    && cd /bin && ln -s /opt/phyml/phyml-3.3.20220408/src/phyml phyml

WORKDIR /opt/corax
RUN --mount=type=bind,source=./CMakeLists.fix.txt,target=/tmp/corax-fix git clone https://codeberg.org/Exelixis-Lab/coraxlib.git \
    && cd coraxlib \
    && cat /tmp/corax-fix >> CMakeLists.txt \
    && mkdir -p build && cd build \
    && cmake .. \
    && make && make install
WORKDIR /opt/tools
RUN --mount=type=bind,rw,source=eval_pythia,target=eval_pythia cd eval_pythia && cargo build --release && cd .. \
    && cp -p ./eval_pythia/target/release/eval_pythia run-evaluation
COPY eval_pythia/ext_runners/predictor.py .
COPY eval_pythia/ext_runners/raxml.sh .
COPY eval_pythia/ext_runners/fasta_to_phy.py .
COPY eval_pythia/ext_runners/phyml.sh .

