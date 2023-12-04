FROM fedora:38 as npb-builder

RUN dnf install -y wget make gcc gcc-gfortran && \
    wget https://www.nas.nasa.gov/assets/npb/NPB3.4.2.tar.gz && \
    tar -xzf NPB3.4.2.tar.gz

WORKDIR /NPB3.4.2/NPB3.4-OMP

COPY docker/npb/*.def config/

RUN make suite

FROM fedora:38

RUN \
    # install Jupyter
    dnf install -y python3-notebook && \
    # install NVIDIA
    dnf install -y https://download1.rpmfusion.org/nonfree/fedora/rpmfusion-nonfree-release-$(rpm -E %fedora).noarch.rpm && \
    dnf install -y akmod-nvidia xorg-x11-drv-nvidia-cuda && \
    # clean up
    dnf clean all && \
    rm -rf /var/cache/yum

COPY --from=npb-builder /NPB3.4.2/NPB3.4-OMP/bin/* /usr/bin/

COPY jupyter /jupyter
WORKDIR /jupyter

CMD [ \
    "/usr/bin/jupyter-notebook", \
    "--allow-root", \
    "--port", "8888", \
    "--ip", "0.0.0.0"]
