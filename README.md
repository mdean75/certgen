# Certgen
> A simple command line utility to generate client and server certificates for testing.

Easily generate valid x509 pem formatted certificates.  Certgen was designed for a specific use case of testing mTLS with nginx and generates a cert chain consisting of a root ca, signing ca, and client and server certificates.

## Built with 
<img src="https://img.shields.io/badge/Rust-FFF?style=for-the-badge&logo=rust&logoColor=black" /> 

## Build release binary

Dynamically linked binary (requires glibc installed on host):

```sh
cargo build --release

# executable can be found at ./target/release/certgen
```

Static glibc linked binary (no dependency on host configuration):

```sh
RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-gnu --release

# executable can be found at ./target/x86_64-unknown-linux-gnu/release/certgen
```

## Usage example

```shell
# generate valid certificates with default names for root and signing ca
./certgen gen

# generate expired client and server certificates  
./certgen gen -e

# print out build information including git branch, commit hash, and build timestamp
./cergben -b
```

Files generated:
- self-signed root ca and key
- intermediate signing ca and key, signed by root
- client and server certificates and keys signed by intermediate ca

All Files will be placed in $PWD/certs

Subdirectories for Client and server files are created and named as the current unix epoch time.
- server epoch folder contains server.crt, server.key, and server cert bundle file
- client epoch folder contains client.crt, client.key, and client cert bundle file

## Testing with nginx

#### Testing configuration:
- Centos 7 vm created with kvm and libvirt running on Ubuntu 20.04 hypervisor
- Nginx 1.20.1
- glibc.x86_64 2.17-326.el7_9
- Netcat 7.50
- Certgen Executable built with Jenkins running on Ubuntu 22.04 and using a docker build agent running Rust image

Sample nginx configuration

```shell
                                                                                                                                                      
load_module /usr/lib64/nginx/modules/ngx_stream_module.so;                                                                                            
user nginx;                                                                                                                                           
                                                                                                                                                      
worker_processes auto;                                                                                                                                
worker_cpu_affinity auto;                                                                                                                             

# enable info logging for nginx to capture expired cert log messages                                                                                                              
error_log /var/log/nginx/error.log info;                                                                                                          
pid /run/nginx.pid;                                                                                                                                   
                                                                                                                                                      
events {                                                                                                                                              
    worker_connections 1024;                                                                                                                          
}                                                                                                                                                     
  
                                                                                                                                                      
stream {                                                                                                                                              
    log_format  main  '$ssl_client_s_dn -- $remote_addr - [$time_local]  '                                                                            
                      '$status  '                                                                                                                     
                      '';                                                                                                                             
                                                                                                                                                      
    access_log  /var/log/nginx/access.log  main;                                                                                                      
                                                                                                                                                      
    ssl_session_cache   shared:SSL:10m;

    ssl_session_timeout 10m;

    upstream backend_server {
        server 127.0.0.1:3001; # netcat listener
    }

    upstream fail {
        server 192.168.1.248:3001; # not a valid device
    }
    map_hash_max_size 128;
    map_hash_bucket_size 128;    
    map $ssl_client_s_dn $backend_svr {
        "C=US,OU=Room 303 org unit,O=Room 303 org,CN=Room 303" backend_server;
        default fail;
    }
    
    server {
        listen 3000; 
        listen 192.168.1.220:5000 ssl;

        ssl_certificate                 /etc/nginx/certs/server-bundle.crt; 
        ssl_certificate_key             /etc/nginx/certs/server.key;          
  
        ssl_protocols   TLSv1 TLSv1.1 TLSv1.2;
        ssl_ciphers ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES256-GCM-SHA384:ECDHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES256
-GCM-SHA384:DHE-RSA-AES128-GCM-SHA256;

        ############ mTLS entries
        ssl_client_certificate  /etc/nginx/certs/client-bundle.crt;
        ssl_verify_depth        2;
        ssl_verify_client       on;
        ############ end mTLS entries
         
        proxy_pass $backend_svr; 
        proxy_protocol on;
    }
}
```

### Nginx mTLS testing strategy
- move server cert bundle and key to nginx certs path 
- move client cert bundle to nginx certs path
- update client dn map to values from client cert and reload nginx: nginx -s reload
- start netcat listener: nc -l 127.0.0.1 3001 -k
- use openssl s_client to initiate connection

```shell
# 192.168.1.220 = ip of server running nginx and netcat listener
# 5000 = port set up to listen for ssl connections
# CApath = path where root ca cert is located, required if root ca is not add to trust store
openssl s_client -connect 192.168.1.220:5000 \ 
    -key certs/1667225997/client.key \ 
    -cert certs/1667225997/client-bundle.crt \  
    -state -CAfile root-ca.crt -CApath $PWD 
```
