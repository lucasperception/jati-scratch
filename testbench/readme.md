This package contains all scripts necessary to run a JATI benchmark on the ZHAW HPC Cluster for a specific commit
## Setup
1. Add your public key to both srv-lab-t-488.zhaw.ch and austin.zhaw.ch
2. Define a proxy jump configuration in `~/.ssh/config`
```bash
HOST austin.node
USER <username>
HostName austin.zhaw.ch
ProxyJump <username>@srv-lab-t-488.zhaw.ch
```
3. Test that login works with `ssh austin.node`
4. Before running for the first time or after a change to reference or slurm submit script run `just reset-reference`


## Run a benchmark for a specific commit hash
1. Build the container for the commit hash under test `just build-hash <hash>`
2. Run the Benchmark with `just bench <hash> <features>`
3. Optionally listen to the job output with `just listen <hash>`
4. After the benchmark is finished, fetch the criterion report with `just fetch-results <hash>`