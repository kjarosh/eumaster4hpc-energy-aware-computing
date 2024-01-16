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
FROM nvidia/cuda:12.3.1-devel-rockylinux9

RUN \
    dnf install -y https://mirrors.rpmfusion.org/free/el/rpmfusion-free-release-9.noarch.rpm && \
    # install Jupyter
    dnf install -y python3-pip && pip3 install jupyter && \
    # install other tools
    dnf install -y perf gnuplot && \
    # install runtime dependencies
    dnf install -y libgfortran boost && \
    # clean up
    dnf clean all && \
    rm -rf /var/cache/yum

COPY --from=npb-builder /NPB3.4.2/NPB3.4-OMP/bin/* /usr/local/bin/
COPY --from=split-builder /repo/build/DEPO /repo/build/StEP /repo/build/SetPowerLimits /repo/build/GetAppPower /usr/local/bin/
COPY docker/scripts/* /usr/local/bin/

COPY jupyter /jupyter
WORKDIR /jupyter

CMD [ \
    "/usr/bin/jupyter-notebook", \
    "--allow-root", \
    "--port", "8888", \
    "--ip", "0.0.0.0"]
