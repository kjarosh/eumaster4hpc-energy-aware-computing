# EUMaster4HPC Energy Aware Parallel Computing

## Acknowledgments

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="img/logo-eumaster4hpc-white.svg"/>
  <img alt="EUMaster4HPC logo" src="img/logo-eumaster4hpc.svg" height="50px"/>
</picture>

This project has received funding from the European High-Performance
Computing Joint Undertaking under grant agreement No 101051997.

## System Requirements

* A Linux-based operating system
* Intel RAPL compatible CPU (newer Intel, AMD CPUs)
* NVIDIA graphics card

## Instructions

1. Download and install Docker: https://docs.docker.com/engine/install/
2. Download and install NVIDIA Container Toolkit: https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/latest/install-guide.html
3. Pull the image:
   ```shell
   docker pull eumaster4hpccyfronet/energy-aware-computing-mooc
   ```
4. Run the image:
   ```shell
   docker run -it --rm --privileged --runtime nvidia -p 8888:8888 eumaster4hpccyfronet/energy-aware-computing-mooc
   ```
5. Click a link in the output of the command above, which should look like the following:
   ```
   http://127.0.0.1:8888/tree?token=<token>
   ```

After opening Jupyter in the browser, a list of notebooks will appear.
By opening a notebook, you will be able to run and edit it.
Follow the instructions from the notebooks to perform the exercises.
For running commands inside the docker image, select `File -> New -> Terminal`.

When no NVIDIA card is present, skip installing NVIDIA Container Toolkit
and do not use `--runtime nvidia`.
In that case, only notebooks related to RAPL will work properly.

The container will operate on the whole operating system.
Power caps and energy consumption are set and read globally,
which means that power caps will influence other programs,
and energy consumption will be influenced by other programs.
It is recommended to close all other programs while running the container.

## What is included in the image

The image includes the following:
1. Jupyter Notebook along with notebooks related to the course
2. Required system tools and packages

The notebooks contained in this Docker image require a specific environment
and will usually not work outside of the container.
