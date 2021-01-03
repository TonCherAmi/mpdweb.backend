FROM gradle:6.7.1-jdk11 AS builder

WORKDIR /home/gradle/mpdweb

COPY --chown=gradle:gradle . .

RUN gradle build

FROM openjdk:11-jre-slim-buster

WORKDIR /srv

COPY --from=builder /home/gradle/mpdweb/build/libs/mpdweb.jar /srv

EXPOSE 8080

CMD java -jar mpdweb.jar
