This is the repository containing the code of [PIMSys](https://dl.acm.org/doi/10.1145/3695794.3695797), a virtual prototype for Processing-in-Memory (PIM).

The repository is split in three parts:
- `pim-isa` contains data structures, shared by both the virtual prototype and the application framework.
- `pim-os` contains the custom ARM-based operating system and the application framework, utilizing PIM.
- `pim-vm` is the virtual prototype.

`pim-os` can be built standalone and is expected to be run with gem5. However, a fork of [gem5](https://github.com/CEJMU/gem5) is needed that supports bare metal ARM workloads.
`pim-vm` is expected to be integrated into [DRAMSys](https://github.com/tukl-msd/DRAMSys). The branch `PIMSys` of DRAMSys does this automatically.
