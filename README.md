# CTS GIS Server
## Supported service types
- feature service
- search service
- mbtiles service
- aggregate service
## Docker编译
> 使用docker命令编译服务，会生成四个服务：认证服务、要素服务、查询服务、上传服务
```shell
 docker compose build

 docker compose build nx-upload-server
```
## 启动容器
> 使用docker服务启动容器
```shell
docker compose -f docker-compose-produce.yml up -d
```
