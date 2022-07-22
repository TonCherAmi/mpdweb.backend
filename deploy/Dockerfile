FROM gradle:7.4-jdk11 AS builder

WORKDIR /home/gradle/mpdweb

COPY --chown=gradle:gradle . .

RUN gradle build

FROM openjdk:11-jre-slim-buster

WORKDIR /srv

COPY --from=builder /home/gradle/mpdweb/build/libs/mpdweb.jar /srv

EXPOSE 8123

CMD java -XX:+UseContainerSupport -XX:MinRAMPercentage=40.0 -XX:MaxRAMPercentage=80.0 -jar mpdweb.jar
