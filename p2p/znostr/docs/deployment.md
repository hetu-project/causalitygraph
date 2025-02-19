# Deployment



## Deploy the relay and zchronod on the server

### 1. Download the git repository
```
git clone https://github.com/hetu-project/ZSocial.git
```
### 2. Run the zchronod
```
cd ZSocial/zchronod/
cargo run
```
### 3. Run the relay
```
cd ZSocial/Nostr_relay/

#config file,default 127.0.0.1:8080
cp rnostr.example.toml config.toml

# Build
cargo build --release

# Show help
./target/release/rnostr relay --help

# Run with config hot reload
./target/release/rnostr relay -c config.toml --watch
```

### 4. Config the ssl in the nginx for wss websocket
#### Nginx config file
```
upstream relaywebsocket {
    # address relay runs on 
    server 127.0.0.1:8080;
}

server {
     listen 80;
     listen 443 ssl;
     server_name <your domain>;
     # paths of two certificated files
     ssl_certificate /etc/nginx/cert/xxx.crt;
     ssl_certificate_key /etc/nginx/cert/xxx.key;
     ssl_session_timeout 5m;
     ssl_protocols TLSv1.2 TLSv1.3;
     ssl_ciphers ECDHE-RSA-AES128-GCM-SHA256:HIGH:!aNULL:!MD5:!RC4:!DHE;
     ssl_prefer_server_ciphers on;
     
     location / {
      proxy_pass http://relaywebsocket;
      proxy_http_version 1.1;
      proxy_set_header Upgrade $http_upgrade;
      proxy_set_header Connection "Upgrade";
      proxy_read_timeout 300s;
     }
 }
```
#### Restart nginx
```
sudo service nginx start
```
#### thus we have
```
wss://<your domain>
```
