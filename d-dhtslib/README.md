```
curl -fsS https://dlang.org/install.sh | bash -s ldc
source ~/dlang/ldc-1.28.0/activate
LD_LIBRARY_PATH=/usr/local/lib dub build -b release
```
