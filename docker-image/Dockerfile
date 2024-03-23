FROM rockylinux:9 as npb-builder

RUN dnf install -y wget make gcc gcc-gfortran && \
    wget https://www.nas.nasa.gov/assets/npb/NPB3.4.2.tar.gz && \
    tar -xzf NPB3.4.2.tar.gz

WORKDIR /NPB3.4.2/NPB3.4-OMP

COPY docker/npb/*.def config/

RUN make suite

################################################################################
FROM nvidia/cuda:12.3.1-devel-rockylinux9 as split-builder

RUN dnf -y install \
   gcc gcc-c++ wget cmake make git boost-devel

RUN git clone https://projects.task.gda.pl/akrz/split.git /repo
WORKDIR /repo

RUN mkdir -p build && cd build && cmake .. && make -j 10

################################################################################
FROM nvidia/cuda:12.3.1-devel-rockylinux9 as gemm-builder

RUN dnf -y install git make

RUN git clone https://github.com/hma02/cublasgemm-benchmark.git /repo
WORKDIR /repo

COPY docker/gemm/patch.diff /tmp/patch.diff
RUN git apply /tmp/patch.diff

ENV CUDA_ARCH_FLAGS=""
RUN make

################################################################################
FROM nvidia/cuda:12.3.1-devel-rockylinux9

RUN \
    dnf install -y https://mirrors.rpmfusion.org/free/el/rpmfusion-free-release-9.noarch.rpm && \
    dnf install -y python3-pip perf gnuplot libgfortran boost && \
    dnf clean all && \
    rm -rf /var/cache/yum && \
    # install Python dependencies
    pip3 --no-cache-dir install jupyter torch torchvision && \
    mkdir -p /usr/local/nvidia && ln -s /usr/local/lib/python3.9/site-packages/nvidia/cudnn/lib /usr/local/nvidia/lib

COPY --from=npb-builder /NPB3.4.2/NPB3.4-OMP/bin/* /usr/local/bin/
COPY --from=split-builder /repo/build/DEPO /repo/build/StEP /repo/build/SetPowerLimits /repo/build/GetAppPower /usr/local/bin/
COPY --from=gemm-builder /repo/gemm /usr/local/bin/
COPY docker/scripts/* /usr/local/bin/
COPY docker/images/* /images/

COPY jupyter /jupyter
WORKDIR /jupyter

ENTRYPOINT []

CMD [ \
    "/usr/local/bin/jupyter-notebook", \
    "--allow-root", \
    "--port", "8888", \
    "--ip", "0.0.0.0"]
