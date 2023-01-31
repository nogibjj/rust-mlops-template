### Working PyTorch Actix

This runs PyTorch + Actix

* To DO:  figure the final containerized steps:
currently missing `libtorch_cpu.so:`

```bash
docker run -it --rm -p 8080:8080 torch
actixtorch: error while loading shared libraries: libtorch_cpu.so: cannot open shared object file: No such file or directory
make: *** [Makefile:14: rundocker] Error 127
```