package com.github.toncherami.mpd.web.properties

import org.springframework.boot.context.properties.ConfigurationProperties
import org.springframework.boot.context.properties.ConstructorBinding

@ConstructorBinding
@ConfigurationProperties(prefix = "mpd")
data class MpdProperties(
    val host: String,
    val port: Int
)
