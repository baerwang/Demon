# Demon

Daemon is a browser crawler that does URL harvesting in chrome headless mode

# Quick Start

## Run

> Demon needs `chromium`, Please confirm that the environment is installed

```shell
./demon --target http://testphp.vulnweb.com
```

## Use chromium

```shell
./demon --target http://testphp.vulnweb.com chromium /tmp/chromium/chrome
```

# More command parameters

```shell
./demon -h
```

## Roadmap

- [x] Robots
- [x] Sitemap.xml
- [ ] Javascript Content
- [x] Custom Headers
- [x] Proxy
- [x] Form
- [x] Click
- [x] Auto Random filling
- [x] Http auth (Basic,Digest,NTLM)
- [ ] Auto login
- [ ] Weak Cryptography
- [ ] Scan Policy

    1. current site (default)
    2. all website
    3. subdomain
    4. not subdomain
    5. directory

- [x] Random User-Agent
- [x] Duplicate detection Policy

    1. params
    2. params+method (default)
    3. params+method+body(xml,json)

- [ ] AI Captcha
- [ ] Output Policy

    1. JSON
    2. Redis