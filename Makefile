_default:
	echo Use install in order to install binaries,
	echo use uninstall in order to remove installed binaries.
	exit 1

install: eumaster4hpc-powercap-get/target/debug/eumaster4hpc-powercap-get eumaster4hpc-powercap-set/target/debug/eumaster4hpc-powercap-set
	cp eumaster4hpc-powercap-get/target/debug/eumaster4hpc-powercap-get /usr/local/bin/
	cp eumaster4hpc-powercap-set/target/debug/eumaster4hpc-powercap-set /usr/local/bin/
	cp eumaster4hpc-task/target/debug/eumaster4hpc-task /usr/local/bin/
	chmod u+s /usr/local/bin/eumaster4hpc-powercap-get
	chmod u+s /usr/local/bin/eumaster4hpc-powercap-set

uninstall:
	rm /usr/local/bin/eumaster4hpc-powercap-get
	rm /usr/local/bin/eumaster4hpc-powercap-set
	rm /usr/local/bin/eumaster4hpc-task

.PHONY: *
