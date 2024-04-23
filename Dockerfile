# development enviroment
FROM alpine:3.19 AS development
WORKDIR /src
COPY . .
# speed up apk downloading (for China mainland)
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk update && apk upgrade
# install rust
RUN apk add --no-cache rust cargo
# release build
RUN cargo build -r

# production enviroment (alpine
FROM alpine:3.19 AS production
WORKDIR /teeworlds_srv/
# speed up apk downloading (for China mainland)
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.ustc.edu.cn/g' /etc/apk/repositories
RUN apk update && apk upgrade
RUN apk add --no-cache rust
COPY --from=development /src/target/release ./infclass_srv
EXPOSE 8303/udp
ENTRYPOINT ["./libtw2-server"]
