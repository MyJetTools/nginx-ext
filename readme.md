## What is this?

This is an add-on to the nginx, which gives the ability to control nginx by rest-api interface.

### How to start it

Example of docker-compose.yaml file

```yaml
version: '2'
services:
  nginx-reverse-proxy:
    image: myjettools/nginx-ext:0.1.0
    container_name: nginx-reverse-proxy
    deploy:
        resources:
            limits:
              cpus: 0.50
              memory: 1024M
            reservations:
              memory: 512M
    network_mode: "host"
    volumes:
      - ./certs:/etc/nginx/certs
      - ./sites-enabled:/etc/nginx/sites-enabled
      - ./logs:/var/log/nginx
      - ./streams:/etc/nginx/streams
      - ./data:/root
```

./certs - Here, we are going to save all the SSL certificates with the pattern
* name.key; - private key 
* name.crt; - Certificate itself
* 
./sites-enabled - here, all the generated files are going to be placed related to http/https traffic;

./logs - here, we can see logs;

./stream - hear all the TCP-related streams that are going to be placed;

./data - here, all the remote control service files are going to be saved;

Please create all the directories before kicking the docker-compose.yaml live

## ./data folder

Please create here configuration file.

**.nginx-ext** file example

```yaml
DataPath: /root
StartNginx: true
NginxConfigFileName: /etc/nginx/sites-enabled/auto-generated.conf
```

All the rest of the files related to non-nginx configurations are kept here.

**nignx.yaml** - All the Nginx configuration changes made by  rest-API are going to be saved in **nginx.yaml**
* UpStream;
* Http Configuration;
* Nginx Configuration Templates;

As well - each folder inside ./data keeps the information about Certificate Authority and about all the generated client certificates

