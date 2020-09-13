package com.github.toncherami.mpd.web

import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.runApplication

@SpringBootApplication
class MpdWebApplication

fun main(args: Array<String>) {
    runApplication<MpdWebApplication>(*args)
}
