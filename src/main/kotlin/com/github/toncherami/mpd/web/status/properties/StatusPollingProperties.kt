package com.github.toncherami.mpd.web.status.properties

import org.springframework.boot.context.properties.ConfigurationProperties
import org.springframework.boot.context.properties.ConstructorBinding

@ConstructorBinding
@ConfigurationProperties(prefix = "status.polling")
data class StatusPollingProperties(
    val interval: Long
)
